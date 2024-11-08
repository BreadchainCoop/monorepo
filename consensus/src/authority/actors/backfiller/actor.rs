use super::{ingress::Mailbox, priority_queue::PriorityQueue, Config, Message};
use crate::{
    authority::{
        actors::{resolver, voter},
        encoder::{proposal_message, proposal_namespace},
        wire, Context, Height, View,
    },
    Automaton, Supervisor,
};
use bytes::Bytes;
use commonware_cryptography::{Digest, Hasher, PublicKey, Scheme};
use commonware_macros::select;
use commonware_p2p::{Receiver, Recipients, Sender};
use commonware_runtime::{Blob, Clock, Storage};
use commonware_storage::archive::{Archive, Translator};
use commonware_utils::hex;
use futures::{
    channel::mpsc,
    future::Either,
    lock::{Mutex, MutexGuard},
    StreamExt,
};
use governor::{
    clock::Clock as GClock, middleware::NoOpMiddleware, state::keyed::HashMapStateStore,
    RateLimiter,
};
use prost::Message as _;
use rand::{prelude::SliceRandom, Rng};
use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet, HashSet},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{debug, warn};

const STARTING_DURATION: Duration = Duration::from_secs(0);

type Status = (PublicKey, SystemTime);

pub struct Actor<
    T: Translator,
    B: Blob,
    E: Clock + GClock + Rng + Storage<B>,
    C: Scheme,
    H: Hasher,
    A: Automaton<Context = Context> + Supervisor<Index = View>,
> {
    runtime: E,
    crypto: C,
    hasher: H,
    application: A,

    proposal_namespace: Vec<u8>,

    proposals: Arc<Mutex<Archive<T, B, E>>>,
    notarizations: Arc<Mutex<Archive<T, B, E>>>,

    mailbox_receiver: mpsc::Receiver<Message>,

    fetch_timeout: Duration,
    max_fetch_count: u32,
    max_fetch_size: usize,
    fetch_rate_limiter:
        RateLimiter<PublicKey, HashMapStateStore<PublicKey>, E, NoOpMiddleware<E::Instant>>,
    fetch_performance: PriorityQueue,
}

impl<
        T: Translator,
        B: Blob,
        E: Clock + GClock + Rng + Storage<B>,
        C: Scheme,
        H: Hasher,
        A: Automaton<Context = Context> + Supervisor<Index = View>,
    > Actor<T, B, E, C, H, A>
{
    pub fn new(
        runtime: E,
        proposals: Arc<Mutex<Archive<T, B, E>>>,
        notarizations: Arc<Mutex<Archive<T, B, E>>>,
        cfg: Config<C, H, A>,
    ) -> (Self, Mailbox) {
        // Initialize rate limiter
        //
        // This ensures we don't exceed the inbound rate limit on any peer we are communicating with (which
        // would halt their processing of all our messages).
        let fetch_rate_limiter = RateLimiter::hashmap_with_clock(cfg.fetch_rate_per_peer, &runtime);

        // Initialize mailbox
        let (sender, receiver) = mpsc::channel(1024);
        (
            Self {
                runtime,
                crypto: cfg.crypto,
                hasher: cfg.hasher,
                application: cfg.application,

                proposal_namespace: proposal_namespace(&cfg.namespace),

                proposals,
                notarizations,

                mailbox_receiver: receiver,

                fetch_timeout: cfg.fetch_timeout,
                max_fetch_count: cfg.max_fetch_count,
                max_fetch_size: cfg.max_fetch_size,
                fetch_rate_limiter,
                fetch_performance: PriorityQueue::new(),
            },
            Mailbox::new(sender),
        )
    }

    // TODO: remove duplicated code
    fn leader(&self, view: View) -> Option<PublicKey> {
        let validators = match self.application.participants(view) {
            Some(validators) => validators,
            None => return None,
        };
        Some(validators[view as usize % validators.len()].clone())
    }

    async fn send(
        &mut self,
        msg: Bytes,
        sent: &mut HashSet<PublicKey>,
        sender: &mut impl Sender,
    ) -> (PublicKey, SystemTime) {
        // Loop until we find a recipient
        loop {
            let mut iter = self.fetch_performance.iter();
            while let Some(next) = iter.next() {
                // Check if self
                if next.public_key == self.crypto.public_key() {
                    continue;
                }

                // Check if already sent this request
                if sent.contains(&next.public_key) {
                    debug!(
                        peer = hex(&next.public_key),
                        "skipping request because already sent"
                    );
                    continue;
                }

                // Check if rate limit is exceeded
                let validator = &next.public_key;
                if self.fetch_rate_limiter.check_key(validator).is_err() {
                    debug!(
                        peer = hex(&validator),
                        "skipping request because rate limited"
                    );
                    continue;
                }

                // Send message
                if sender
                    .send(Recipients::One(validator.clone()), msg.clone(), false)
                    .await
                    .unwrap()
                    .is_empty()
                {
                    // Try again
                    debug!(peer = hex(&validator), "failed to send request");
                    continue;
                }
                debug!(peer = hex(&validator), "sent request");
                sent.insert(validator.clone());
                let deadline = self.runtime.current() + self.fetch_timeout;

                // Minimize footprint of rate limiter
                self.fetch_rate_limiter.shrink_to_fit();
                return (validator.clone(), deadline);
            }

            // Avoid busy looping when disconnected
            warn!("failed to send request to any validator");
            self.runtime.sleep(self.fetch_timeout).await;

            // Clear sent
            sent.clear();
        }
    }

    async fn send_block_request(
        &mut self,
        digest: Digest,
        parents: u32,
        sent: &mut HashSet<PublicKey>,
        sender: &mut impl Sender,
    ) -> Status {
        // Create new message
        let msg = wire::Backfiller {
            payload: Some(wire::backfiller::Payload::ProposalRequest(
                wire::ProposalRequest { digest, parents },
            )),
        }
        .encode_to_vec()
        .into();

        // Send message
        let (public_key, deadline) = self.send(msg, sent, sender).await;
        (public_key, deadline)
    }

    async fn send_notarization_request(
        &mut self,
        view: View,
        children: u32,
        sent: &mut HashSet<PublicKey>,
        sender: &mut impl Sender,
    ) -> Status {
        // Create new message
        let msg = wire::Backfiller {
            payload: Some(wire::backfiller::Payload::NotarizationRequest(
                wire::NotarizationRequest { view, children },
            )),
        }
        .encode_to_vec()
        .into();

        // Send message
        let (public_key, deadline) = self.send(msg, sent, sender).await;
        (public_key, deadline)
    }

    pub async fn run(
        mut self,
        last_notarized: View,
        voter: &mut voter::Mailbox,
        resolver: &mut resolver::Mailbox,
        mut sender: impl Sender,
        mut receiver: impl Receiver,
    ) {
        // Instantiate priority queue
        let validators = self.application.participants(last_notarized).unwrap();
        self.fetch_performance
            .retain(self.fetch_timeout / 2, validators);

        // Wait for an event
        let mut outstanding_proposal: Option<(Digest, u32, HashSet<PublicKey>, Status)> = None;
        let mut outstanding_notarization: Option<(View, u32, HashSet<PublicKey>, Status)> = None;
        loop {
            // Set timeout for next proposal
            let proposal_timeout = if let Some((_, _, _, status)) = &outstanding_proposal {
                Either::Left(self.runtime.sleep_until(status.1))
            } else {
                Either::Right(futures::future::pending())
            };

            // Set timeout for next notarization
            let notarization_timeout = if let Some((_, _, _, status)) = &outstanding_notarization {
                Either::Left(self.runtime.sleep_until(status.1))
            } else {
                Either::Right(futures::future::pending())
            };

            // Wait for an event
            select! {
                _ = proposal_timeout => {
                    // Penalize requester for timeout
                    let (digest, parents, mut sent, status) = outstanding_proposal.take().unwrap();
                    self.fetch_performance.put(status.0, self.fetch_timeout);

                    // Send message
                    let status = self.send_block_request(digest.clone(), parents, &mut sent, &mut sender).await;
                    outstanding_proposal = Some((digest, parents, sent, status));
                    continue;
                },
                _ = notarization_timeout => {
                    // Penalize requester for timeout
                    let (view, children, mut sent, status) = outstanding_notarization.take().unwrap();
                    self.fetch_performance.put(status.0, self.fetch_timeout);

                    // Send message
                    let status = self.send_notarization_request(view, children, &mut sent,  &mut sender).await;
                    outstanding_notarization = Some((view, children, sent, status));
                    continue;
                },
                mailbox = self.mailbox_receiver.next() => {
                    let msg = mailbox.unwrap();
                    match msg {
                        Message::Notarized { view } => {
                            // Update stored validators
                            let validators = self.application.participants(view).unwrap();
                            self.fetch_performance.retain(self.fetch_timeout/2, validators);
                            continue;
                        },
                        Message::Proposals { digest, parents } => {
                            // Send message
                            let mut sent = HashSet::new();
                            let status = self.send_block_request(digest.clone(), parents, &mut sent, &mut sender).await;
                            outstanding_proposal = Some((digest, parents, sent, status));
                            continue;
                        },
                        Message::Notarizations { view, children } => {
                            // Send message
                            let mut sent = HashSet::new();
                            let status = self.send_notarization_request(view, children, &mut sent, &mut sender).await;
                            outstanding_notarization = Some((view, children, sent, status));
                            continue;
                        },
                    }
                },
                network = receiver.recv() => {
                    let (s, msg) = network.unwrap();
                    let msg = match wire::Backfiller::decode(msg) {
                        Ok(msg) => msg,
                        Err(err) => {
                            warn!(?err, sender = hex(&s), "failed to decode message");
                            continue;
                        },
                    };
                    let payload = match msg.payload {
                        Some(payload) => payload,
                        None => {
                            warn!(sender = hex(&s), "missing payload");
                            continue;
                        },
                    };
                    match payload {
                        wire::backfiller::Payload::ProposalRequest(request) => {
                            // Confirm request is valid
                            if !H::validate(&request.digest) {
                                warn!(sender = hex(&s), "invalid digest");
                                continue;
                            }

                            // Populate as many proposals as possible
                            let mut proposal_bytes = 0;
                            let mut proposals_found = Vec::new();
                            let mut cursor = request.digest.clone();
                            {
                                let proposals = self.proposals.lock().await;
                                loop {
                                    // Check to see if we have proposal
                                    let proposal = match proposals.get(&cursor).await {
                                        Ok(proposal) => proposal,
                                        Err(err) => {
                                            debug!(
                                                sender = hex(&s),
                                                proposal = hex(&cursor),
                                                ?err,
                                                "unable to load proposal",
                                            );
                                            break;
                                        }
                                    };
                                    let proposal = match proposal {
                                        Some(proposal) => proposal,
                                        None => {
                                            debug!(
                                                sender = hex(&s),
                                                proposal = hex(&cursor),
                                                "missing proposal",
                                            );
                                            break;
                                        }
                                    };
                                    let proposal = wire::Proposal::decode(proposal).expect("unable to decode persisted proposal");

                                    // If we don't have any more space, stop
                                    proposal_bytes += proposal.encoded_len();
                                    if proposal_bytes > self.max_fetch_size {
                                        debug!(
                                            requested = request.parents + 1,
                                            found = proposals_found.len(),
                                            peer = hex(&s),
                                            "reached max response size",
                                        );
                                        break;
                                    }

                                    // If we do have space, add to proposals
                                    cursor = proposal.parent.clone();
                                    proposals_found.push(proposal);

                                    // If we have all parents requested, stop gathering more
                                    let fetched = proposals_found.len() as u32;
                                    if fetched == request.parents + 1 || fetched == self.max_fetch_count {
                                        break;
                                    }
                                }
                            }

                            // Send response
                            debug!(digest = hex(&request.digest), requested = request.parents + 1, found = proposals_found.len(), peer = hex(&s), "responding to backfill request");
                            let msg =  wire::Backfiller {
                                payload: Some(wire::backfiller::Payload::ProposalResponse(wire::ProposalResponse {
                                    proposals: proposals_found,
                                })),
                            }
                            .encode_to_vec()
                            .into();
                            sender.send(Recipients::One(s), msg, false).await.unwrap();
                        },
                        wire::backfiller::Payload::ProposalResponse(response) => {
                            // Parse proposals
                            let mut next = None;
                            let mut proposals_found = Vec::new();
                            for proposal in response.proposals {
                                // Ensure this is the container we want
                                if !H::validate(&proposal.parent) {
                                    debug!(sender = hex(&s), "invalid proposal parent digest size");
                                    break;
                                }
                                let payload_digest = match self.application.parse(proposal.payload.clone()).await {
                                    Some(payload_digest) => payload_digest,
                                    None => {
                                        debug!(sender = hex(&s), "unable to parse notarized/finalized payload");
                                        break;
                                    }
                                };
                                let proposal_message = proposal_message(
                                    proposal.view,
                                    proposal.height,
                                    &proposal.parent,
                                    &payload_digest,
                                );
                                self.hasher.update(&proposal_message);
                                let proposal_digest = self.hasher.finalize();
                                if let Some((height, ref digest)) = next {
                                    if proposal.height != height || proposal_digest != digest {
                                        debug!(sender = hex(&s), "received invalid batch proposal");
                                        break;
                                    }
                                }

                                // Verify leader signature
                                let signature = match &proposal.signature {
                                    Some(signature) => signature,
                                    None => {
                                        debug!(sender = hex(&s), "missing proposal signature");
                                        break;
                                    }
                                };
                                if !C::validate(&signature.public_key) {
                                    debug!(sender = hex(&s), "invalid proposal public key");
                                    break;
                                }
                                let expected_leader = match self.leader(proposal.view) {
                                    Some(leader) => leader,
                                    None => {
                                        debug!(
                                            proposal_leader = hex(&signature.public_key),
                                            reason = "unable to compute leader",
                                            "dropping proposal"
                                        );
                                        break;
                                    }
                                };
                                if expected_leader != signature.public_key {
                                    debug!(
                                        proposal_leader = hex(&signature.public_key),
                                        view_leader = hex(&expected_leader),
                                        reason = "leader mismatch",
                                        "dropping proposal"
                                    );
                                    break;
                                }
                                if !C::verify(
                                    &self.proposal_namespace,
                                    &proposal_message,
                                    &signature.public_key,
                                    &signature.signature,
                                ) {
                                    warn!(sender = hex(&s), "invalid proposal signature");
                                    break;
                                }

                                // Record the proposal
                                let height = proposal.height;
                                if height > 1 {
                                    next = Some((height-1, proposal.parent.clone()));
                                } else {
                                    next = None;
                                }
                                debug!(height, digest = hex(&proposal_digest), peer = hex(&s), "received proposal via backfill");

                                // Remove outstanding task if we were waiting on this
                                //
                                // Note, we don't care if we are sent the proposal from someone unexpected (although
                                // this is unexpected).
                                if let Some(ref outstanding) = outstanding_proposal{
                                    if outstanding.0 == proposal_digest {
                                        debug!(height = outstanding.1, digest = hex(&proposal_digest), peer = hex(&s), "resolved missing proposal via backfill");
                                        outstanding_proposal = None;

                                        // TODO: notify requester
                                    }
                                }
                                proposals_found.push((height, proposal_digest, proposal));

                                // Stop processing if we don't need anything else
                                if next.is_none() {
                                    break;
                                }
                            }

                            // Persist proposals
                            let mut proposals = self.proposals.lock().await;
                            for (height, digest, proposal) in proposals_found {
                                let section = height & 0xFFFF_FFFF_FFFF_0000u64;
                                let proposal = proposal.encode_to_vec().into();
                                let result = proposals.put(section, &digest, proposal, false).await;
                                if let Err(err) = result {
                                    warn!(height, digest = hex(&digest), ?err, "unable to persist proposal");
                                } else {
                                    debug!(height, digest = hex(&digest), peer = hex(&s), "persisted proposal");
                                }
                            }
                        },
                        wire::backfiller::Payload::NotarizationRequest(request) => {
                            // Populate as many notarizations as we can
                            let mut notarization_bytes = 0; // TODO: add a buffer
                            let mut notarizations_found = Vec::new();
                            let mut cursor = request.view;
                            {
                                let notarizations = self.notarizations.lock().await;
                                loop {
                                    // Attempt to fetch notarization
                                    let mut key = [0u8; 9];
                                    key[0..8].copy_from_slice(&cursor.to_be_bytes());
                                    key[8] = 0x01;
                                    let mut notarization = match notarizations.get(&key).await {
                                        Ok(notarization) => notarization,
                                        Err(err) => {
                                            debug!(
                                                sender = hex(&s),
                                                view = cursor,
                                                ?err,
                                                "unable to load notarization",
                                            );
                                            break;
                                        }
                                    };
                                    if notarization.is_none() {
                                        key[8] = 0x00;
                                        notarization = match notarizations.get(&key).await {
                                            Ok(notarization) => notarization,
                                            Err(err) => {
                                                debug!(
                                                    sender = hex(&s),
                                                    view = cursor,
                                                    ?err,
                                                    "unable to load notarization",
                                                );
                                                break;
                                            }
                                        };
                                    }
                                    if notarization.is_none() {
                                        debug!(
                                            sender = hex(&s),
                                            view = cursor,
                                            "missing notarization",
                                        );
                                        break;
                                    }
                                    let notarization = wire::Notarization::decode(notarization.unwrap()).expect("unable to decode persisted notarization");

                                    // If we don't have any more space, stop
                                    notarization_bytes += notarization.encoded_len();
                                    if notarization_bytes > self.max_fetch_size{
                                        debug!(
                                            requested = request.children + 1,
                                            fetched = notarizations_found.len(),
                                            peer = hex(&s),
                                            "reached max fetch size"
                                        );
                                        break;
                                    }
                                    notarizations_found.push(notarization);

                                    // If we have all children or we hit our limit, stop
                                    let fetched = notarizations_found.len() as u32;
                                    if fetched == request.children +1 || fetched == self.max_fetch_count {
                                        break;
                                    }
                                    cursor +=1;
                                }
                            }

                            // Send back notarizations
                            debug!(view = cursor, fetched = notarizations_found.len(), peer = hex(&s), "responding to notarization request");
                            let msg = wire::Backfiller {
                                payload: Some(wire::backfiller::Payload::NotarizationResponse(
                                    wire::NotarizationResponse {
                                        notarizations: notarizations_found,
                                    },
                                )),
                            }
                            .encode_to_vec()
                            .into();
                            sender.send(Recipients::One(s), msg, false).await.unwrap();
                        },
                        wire::backfiller::Payload::NotarizationResponse(response) => {
                            // TODO: skip duration update if response is empty
                        },
                    }
                }
            }
        }
    }
}
