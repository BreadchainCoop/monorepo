mod actor;
mod ingress;

use crate::simplex::Context;
use crate::{simplex::View, Automaton, Supervisor};
use crate::{Committer, Relay};
pub use actor::Actor;
use commonware_cryptography::{Component, Scheme};
pub use ingress::{Mailbox, Message};
use prometheus_client::registry::Registry;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Config<
    C: Scheme,
    D: Component,
    A: Automaton<Context = Context<D>, Digest = D>,
    R: Relay<Digest = D>,
    F: Committer<Digest = D>,
    S: Supervisor<Index = View>,
> {
    pub crypto: C,
    pub automaton: A,
    pub relay: R,
    pub committer: F,
    pub supervisor: S,

    pub registry: Arc<Mutex<Registry>>,
    pub namespace: Vec<u8>,
    pub mailbox_size: usize,
    pub leader_timeout: Duration,
    pub notarization_timeout: Duration,
    pub nullify_retry: Duration,
    pub activity_timeout: View,
    pub replay_concurrency: usize,
}
