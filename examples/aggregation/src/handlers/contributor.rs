use commonware_cryptography::{Hasher, Scheme, Sha256};
use commonware_p2p::{Receiver, Sender};
use commonware_utils::hex;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use prost::Message;
use YourContract::yourFuncCall;
use std::{collections::{HashMap, HashSet}, env, str::FromStr};
use tracing::info;
use alloy::{hex::ToHexExt, json_abi::Function, providers::RootProvider, sol, sol_types::SolCall};
use crate::bn254::{self, aggregate_signatures, aggregate_verify, Bn254, G1PublicKey, PublicKey, Signature};
use alloy_primitives::{Address, Bytes, U256, FixedBytes};
use url;
use dotenv::dotenv;

// use alloy_provider::ProviderBuilder;
// Import the generated binding for VotingContract
use crate::bindings::votingcontract::VotingContract;
use url::Url;

use super::wire;
sol! {
    contract YourContract {
        #[derive(Debug)]
        function yourFunc(uint256 transition_index, address contract_address, bytes4 function_sig, bytes storage_updates) public returns (bytes memory);
    }
}

pub struct Contributor {
    orchestrator: PublicKey,
    signer: Bn254,
    me: usize,
    g1_map: HashMap<PublicKey, G1PublicKey>, // g2 (PublicKey) -> g1 (PublicKey)
    contributors: Vec<PublicKey>,
    ordered_contributors: HashMap<PublicKey, usize>,
    t: usize,
}

impl Contributor {
    pub fn new(
        orchestrator: PublicKey,
        signer: Bn254,
        mut contributors: Vec<PublicKey>,
        t: usize,
        g1_map: HashMap<PublicKey, G1PublicKey>,
    ) -> Self {
        dotenv().ok();
        contributors.sort();
        let mut ordered_contributors = HashMap::new();
        for (idx, contributor) in contributors.iter().enumerate() {
            ordered_contributors.insert(contributor.clone(), idx);
        }
        let me = *ordered_contributors.get(&signer.public_key()).unwrap();
        Self {
            orchestrator,
            signer,
            me,
            contributors,
            ordered_contributors,
            t,
            g1_map,
        }
    }

    pub async fn run(
        mut self,
        mut sender: impl Sender,
        mut receiver: impl Receiver<PublicKey = PublicKey>,
    ) {
        let mut hasher = Sha256::new();
        let mut signed = HashSet::new();
        let mut signatures: HashMap<u64, HashMap<usize, Signature>> = HashMap::new();
        while let Ok((s, message)) = receiver.recv().await {
            // Parse message
            let Ok(message) = wire::Aggregation::decode(message) else {
                continue;
            };
            let round = message.round;

            // Check if from orchestrator
            if s != self.orchestrator {
                // Get contributor
                let Some(contributor) = self.ordered_contributors.get(&s) else {
                    continue;
                };

                // Check if contributor already signed
                let Some(signatures) = signatures.get_mut(&round) else {
                    continue;
                };
                if signatures.contains_key(contributor) {
                    continue;
                }

                // Extract signature
                let signature = match message.payload {
                    Some(wire::aggregation::Payload::Signature(signature)) => signature.signature,
                    _ => continue,
                };
                let Ok(signature) = Signature::try_from(signature) else {
                    continue;
                };
                // Verify signature
                let transition_index = round;
                // Check if transition_index matches state transition count
                let state_transition_count = self.get_state_transition_count().await.unwrap();
                if U256::from(transition_index) != state_transition_count {
                    println!("WARNING: transition_index ({}) does not match state_transition_count ({})", 
                        transition_index, state_transition_count);
                }
                let private_key = env::var("PRIVATE_KEY")
                    .expect("PRIVATE_KEY must be set");

                let contract_address = Address::from_str(
                    &env::var("TARGET_ADDRESS")
                        .expect("TARGET_ADDRESS must be set")
                ).unwrap();
                info!("Target address: {}", contract_address);
                let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
                let storage_updates = self.get_storage_updates(transition_index).await.unwrap(); 
                println!("storage_updates: {:?}", storage_updates);
                println!("transition_index: {:?}", transition_index);
                let storage_updates_clone = storage_updates.clone();
                let encoded = yourFuncCall{
                    transition_index: U256::from(transition_index),
                    contract_address,
                    function_sig,
                    storage_updates: storage_updates_clone,
                }.abi_encode();
                // Generate signature
                let payload = encoded[4..].to_vec(); // Skip first 4 bytes
                println!("payload: {:?}", payload.encode_hex());
                hasher.update(&payload);
                let payload = hasher.finalize();
                println!("hash: {:?}", payload);
                println!("round: {:?}", round);
                println!("msg: {:?}", hex(&payload));
                if !Bn254::verify(None, &payload, &s, &signature) {
                    continue;
                }

                // Insert signature
                signatures.insert(*contributor, signature);

                // Check if should aggregate
                if signatures.len() < self.t {
                    continue;
                }

                // Aggregate signatures
                let mut participating = Vec::new();
                let mut participating_g1 = Vec::new();
                let mut sigs = Vec::new();
                for i in 0..self.contributors.len() {
                    let Some(signature) = signatures.get(&i) else {
                        continue;
                    };
                    let contributor = &self.contributors[i];
                    participating.push(contributor.clone());
                    participating_g1.push(self.g1_map[contributor].clone());
                    sigs.push(signature.clone());
                }
                let agg_signature = aggregate_signatures(&sigs).unwrap();

                // Verify aggregated signature (already verified individual signatures so should never fail)
                if !aggregate_verify(&participating, None, &payload, &agg_signature) {
                    panic!("failed to verify aggregated signature");
                }
                let (apk, apk_g2, asig) = bn254::get_points(&participating_g1, &participating, &sigs).unwrap();
                        let apk = convert_to_g1_point(apk).unwrap();
                        let apk_g2 = convert_to_g2_point(apk_g2).unwrap();
                        let asig = convert_to_g1_point(asig).unwrap();
                        info!(
                            msg = hex(&payload),
                            ?participating,
                            signature = ?agg_signature,
                            apk_x = ?apk.X,
                            apk_y = ?apk.Y,
                            apk_g2_x = ?apk_g2.X,
                            apk_g2_y = ?apk_g2.Y,
                            asig_x = ?asig.X,
                            asig_y = ?asig.Y,
                            "aggregated signatures",
                        );
                        let payload_hash = payload.to_vec();
                        let mut msg_hash_bytes = [0u8; 32];
                        if payload_hash.len() >= 32 {
                            msg_hash_bytes.copy_from_slice(&payload_hash[0..32]);
                        } else {
                            msg_hash_bytes[0..payload_hash.len()].copy_from_slice(&payload_hash);
                        }
                        let msg_hash = FixedBytes::<32>::from(msg_hash_bytes);
                        let rpc_url = env::var("HTTP_ENDPOINT").unwrap();
                        println!("Partcipating len: {:?}", participating.len());
                        println!(r#"[eth verification] cast c -r https://eth.llamarpc.com 0xb7ba8bbc36AA5684fC44D02aD666dF8E23BEEbF8 "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))" "{:?}" "({:?},{:?})" "({:?},{:?})" "({:?},{:?})""#, hex(&payload), apk.X, apk.Y, apk_g2.X, apk_g2.Y, asig.X, asig.Y);
                        let output = std::process::Command::new("cast")
                        .arg("s")
                        .arg("--private-key")
                        .arg(&private_key)
                        .arg(contract_address.to_string())
                        .arg("--rpc-url")
                        .arg(rpc_url)
                        .arg("https://eth.llamarpc.com")
                        .arg("--value")
                        .arg("100000000000000000")
                        .arg("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)")
                        .arg(format!("{:?}", msg_hash))
                        .arg(format!("({:?},{:?})", apk.X, apk.Y))
                        .arg(format!("({:?},{:?})", apk_g2.X, apk_g2.Y))
                        .arg(format!("({:?},{:?})", asig.X, asig.Y))
                        .arg(format!("{:?}", storage_updates))
                        .arg(format!("{:?}", transition_index))
                        .arg(format!("{:?}", contract_address))
                        .arg(format!("{:?}", function_sig))
                        .output()
                        .expect("Failed to execute command");
                info!(
                    round,
                    msg = hex(&payload),
                    ?participating,
                    signature = ?agg_signature,
                    "aggregated signatures",
                );
                continue;
            }

            // Handle message from orchestrator
            match message.payload {
                Some(wire::aggregation::Payload::Start(start)) => start,
                _ => continue,
            };

            // Check if already signed at round
            if !signed.insert(round) {
                continue;
            }
            let transition_index = message.round;
            let contract_address = Address::from_str(
                &env::var("TARGET_ADDRESS")
                    .expect("TARGET_ADDRESS must be set")
            ).unwrap();
            let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
            let storage_updates = self.get_storage_updates(transition_index).await.unwrap(); 
            println!("storage_updates: {:?}", storage_updates);
            println!("transition_index: {:?}", transition_index);
            println!("round: {:?}", round);
            let storage_updates_clone = storage_updates.clone();
            let encoded = yourFuncCall{
                transition_index: U256::from(transition_index),
                contract_address,
                function_sig,
                storage_updates: storage_updates_clone,
            }.abi_encode();
            // Generate signature
            let payload = encoded[4..].to_vec(); // Skip first 4 bytes;
            hasher.update(&payload);
            let payload = hasher.finalize();
            info!("Generating signature for round: {}, payload hash: {}", round, hex(&payload));
            let signature = self.signer.sign(None, &payload);
            info!("Generated signature for round: {}", round);

            // Store signature
            signatures
                .entry(round)
                .or_default()
                .insert(self.me, signature.clone());

            // Return signature to orchestrator
            let message = wire::Aggregation {
                round: transition_index,
                payload: Some(wire::aggregation::Payload::Signature(wire::Signature {
                    signature: signature.to_vec(),
                })),
            }
            .encode_to_vec()
            .into();
            info!("Sending signature for round: {}", round);
            
            // Broadcast to all (including orchestrator)
            sender
                .send(commonware_p2p::Recipients::All, message, true)
                .await
                .expect("failed to broadcast signature");
            info!(round, "broadcast signature");
        }
    }

    pub async fn get_storage_updates(&self, transition_index: u64) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the string to a Url
        let http_endpoint = env::var("HTTP_ENDPOINT")
            .expect("HTTP_ENDPOINT must be set");
        let url = Url::parse(&http_endpoint).unwrap();
        let provider: RootProvider = RootProvider::new_http(url);
        println!("transition_index: {:?}", transition_index);
        let contract_address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        
        let contract = VotingContract::new(contract_address, provider);
        
        let call_return = contract.operatorExecuteVote(U256::from(transition_index))
            .call()
            .await?;
        Ok(call_return._0)
    }

    pub async fn get_state_transition_count(&self) -> Result<U256, Box<dyn std::error::Error + Send + Sync>> {
        // Get the HTTP endpoint from environment variables
        let http_endpoint = env::var("HTTP_ENDPOINT")
            .expect("HTTP_ENDPOINT must be set");
        let url = Url::parse(&http_endpoint).unwrap();
        
        // Create a provider
        let provider: RootProvider = RootProvider::new_http(url);
        
        // Get the contract address from environment variables
        let contract_address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        
        // Create a contract instance
        let contract = VotingContract::new(contract_address, provider);
        
        // Call the stateTransitionCount function
        let call_return = contract.stateTransitionCount()
            .call()
            .await?;
        
        // Return the count value
        Ok(call_return.count)
    }
    
}
