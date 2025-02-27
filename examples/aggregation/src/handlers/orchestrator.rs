use crate::bn254::{G1PublicKey, PublicKey, Signature, Bn254, PrivateKey};
use crate::bn254;
use alloy::json_abi::Function;
use alloy::sol_types::SolCall;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use commonware_cryptography::{Hasher, Scheme, Sha256};
use commonware_macros::select;
use commonware_p2p::{Receiver, Sender};
use commonware_runtime::Clock;
use commonware_utils::hex;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use prost::Message;
use std::{
    collections::HashMap,
    time::{Duration, UNIX_EPOCH},
    str::FromStr,
    env,
};
use alloy::{hex::ToHexExt, providers::RootProvider, sol};
use tracing::info;
use dotenv::dotenv;
use alloy::providers::Provider;
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;
use alloy_network::Ethereum;
use alloy_primitives::{Address, Bytes, U256, FixedBytes};
use url::Url;
use crate::handlers::wire;
use YourContract::yourFuncCall;
use crate::bindings::votingcontract::VotingContract;


pub struct Orchestrator<E: Clock> {
    runtime: E,
    signer: Bn254,
    aggregation_frequency: Duration,
    contributors: Vec<PublicKey>,
    g1_map: HashMap<PublicKey, G1PublicKey>, // g2 (PublicKey) -> g1 (PublicKey)
    ordered_contributors: HashMap<PublicKey, usize>,
    t: usize,
}

sol! {
    contract YourContract {
        #[derive(Debug)]
        function yourFunc(uint256 transition_index, address contract_address, bytes4 function_sig, bytes storage_updates) public returns (bytes memory);
    }
}

// sol! {
//     #[sol(rpc)]
//     "../../Something.sol"
// }


impl<E: Clock> Orchestrator<E> {
    pub fn new(
        runtime: E,
        signer: Bn254,
        aggregation_frequency: Duration,
        mut contributors: Vec<PublicKey>,
        g1_map: HashMap<PublicKey, G1PublicKey>,
        t: usize,
    ) -> Self {
        dotenv().ok();
        
        contributors.sort();
        let mut ordered_contributors = HashMap::new();
        for (idx, contributor) in contributors.iter().enumerate() {
            ordered_contributors.insert(contributor.clone(), idx);
        }

        Self {
            runtime,
            signer,
            aggregation_frequency,
            contributors,
            g1_map,
            ordered_contributors,
            t,
        }
    }

    pub async fn run(
        mut self,
        mut sender: impl Sender,
        mut receiver: impl Receiver<PublicKey = PublicKey>,
    ) {
        let mut hasher = Sha256::new();
        let mut signatures = HashMap::new();
        
        let registry_coordinator_address: Address = Address::from_str(
            &env::var("REGISTRY_COORDINATOR_ADDRESS")
                .expect("REGISTRY_COORDINATOR_ADDRESS must be set")
        ).unwrap();
        info!("Registry coordinator address: {}", registry_coordinator_address);

        let contract_address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        info!("Target address: {}", contract_address);

        let http_endpoint = env::var("HTTP_ENDPOINT")
            .expect("HTTP_ENDPOINT must be set");
        info!("HTTP endpoint: {}", http_endpoint);

        let provider: RootProvider = RootProvider::new_http(Url::parse(&http_endpoint).unwrap());

        loop {
            // Generate payload
            let current_block_num = provider.get_block_number().await.unwrap();
            let transition_index = self.get_state_transition_count().await.unwrap();
            let transition_index_u64 = transition_index.to::<u64>();
            let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
            let storage_updates = self.get_storage_updates(current_block_num).await.unwrap();
            println!("storage_updates: {:?}", storage_updates);
            let encoded = yourFuncCall {
                transition_index,
                contract_address,
                function_sig,
                storage_updates,
            }.abi_encode();
            let payload = encoded[4..].to_vec(); // Skip first 4 bytes
            println!("payload: {:?}", payload.encode_hex());
            println!("transition_index: {:?}", transition_index);
            // let payload = encoded;
            hasher.update(&payload);
            let payload = hasher.finalize();
            println!("hash: {:?}", payload);
            // Sign the timestamp hash with BN254
            // let payload = self.signer.sign(None, &payload);
            info!(round = transition_index_u64, msg = hex(&payload), "generated and signed message");

            // Broadcast payload
            let message = wire::Aggregation {
                round: transition_index_u64,
                payload: Some(wire::aggregation::Payload::Start(wire::Start {})),
            }
            .encode_to_vec()
            .into();
            sender
                .send(commonware_p2p::Recipients::All, message, true)
                .await
                .expect("failed to broadcast message");
            signatures.insert(transition_index_u64, HashMap::new());
            info!("Created signatures entry for round: {}, threshold is: {}", transition_index_u64, self.t);

            // Listen for messages until the next broadcast
            let continue_time = self.runtime.current() + self.aggregation_frequency;
            loop {
                select! {
                    _ = self.runtime.sleep_until(continue_time) => {break;},
                    msg = receiver.recv() => {
                        // Parse message
                        let (sender, msg) = match msg {
                            Ok(msg) => msg,
                            Err(_) => continue,
                        };

                        // Get contributor
                        let Some(contributor) = self.ordered_contributors.get(&sender) else {
                            info!("Received message from unknown sender: {:?}", sender);
                            continue;
                        };

                        // Check if round exists
                        let Ok(msg) = wire::Aggregation::decode(msg) else {
                            info!("Failed to decode message from sender: {:?}", sender);
                            continue;
                        };
                        let Some(round) = signatures.get_mut(&msg.round) else {
                            info!("Received signature for unknown round: {} from contributor: {:?}", msg.round, contributor);
                            continue;
                        };

                        // Check if contributor has already signed
                        if round.contains_key(contributor) {
                            info!("Contributor already signed for round: {} contributor: {:?}", msg.round, contributor);
                            continue;
                        }

                        // Extract signature
                        let signature = match msg.payload {
                            Some(wire::aggregation::Payload::Signature(signature)) => {
                                info!("Received signature for round: {} from contributor: {:?}", msg.round, contributor);
                                signature.signature
                            },
                            _ => {
                                info!("Received non-signature payload from contributor: {:?}", contributor);
                                continue;
                            }
                        };
                        let Ok(signature) = Signature::try_from(signature) else {
                            info!("Failed to parse signature from contributor: {:?}", contributor);
                            continue;
                        };

                        // Verify signature
                        // Generate the same payload that contributors are signing
                        let transition_index = msg.round;
                        let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
                        let storage_updates = match self.get_storage_updates(transition_index).await {
                            Ok(updates) => {
                                info!("Got storage updates for round: {}", transition_index);
                                updates
                            },
                            Err(e) => {
                                info!("Failed to get storage updates for round: {}, error: {:?}", transition_index, e);
                                Bytes::default()
                            }
                        };
                        
                        let encoded = yourFuncCall {
                            transition_index: U256::from(transition_index),
                            contract_address,
                            function_sig,
                            storage_updates,
                        }.abi_encode();
                        let payload_bytes = encoded[4..].to_vec(); // Skip first 4 bytes
                        hasher.update(&payload_bytes);
                        let payload = hasher.finalize();
                        
                        info!("Verifying signature for round: {} from contributor: {:?}, payload hash: {}", 
                              transition_index, contributor, hex(&payload));
                        
                        if !Bn254::verify(None, &payload, &sender, &signature) {
                            info!("Signature verification failed for contributor: {:?}", contributor);
                            continue;
                        }

                        info!("Signature verification succeeded for contributor: {:?}", contributor);
                        
                        // Insert signature
                        round.insert(contributor, signature);

                        // Check if should aggregate
                        info!("Current signatures count for round {}: {}, threshold: {}", 
                              msg.round, round.len(), self.t);
                        if round.len() < self.t {
                            continue;
                        }

                        // Aggregate signatures
                        let mut participating = Vec::new();
                        let mut participating_g1 = Vec::new();
                        let mut signatures = Vec::new();
                        for i in 0..self.contributors.len() {
                            let Some(signature) = round.get(&i) else {
                                continue;
                            };
                            let contributor = &self.contributors[i];
                            participating_g1.push(self.g1_map[contributor].clone());
                            participating.push(contributor.clone());
                            signatures.push(signature.clone());
                        }
                        let agg_signature = bn254::aggregate_signatures(&signatures).unwrap();

                        // Verify aggregated signature (already verified individual signatures so should never fail)
                        if !bn254::aggregate_verify(&participating, None, &payload, &agg_signature) {
                            panic!("failed to verify aggregated signature");
                        }

                        // Log points
                        let (apk, apk_g2, asig) = bn254::get_points(&participating_g1, &participating, &signatures).unwrap();
                        let apk_g1 = convert_to_g1_point(apk).unwrap();
                        let apk_g2_point = convert_to_g2_point(apk_g2).unwrap();
                        let asig_g1 = convert_to_g1_point(asig).unwrap();
                        info!(
                            round = msg.round,
                            msg = hex(&payload),
                            ?participating,
                            signature = ?agg_signature,
                            apk_x = ?apk_g1.X,
                            apk_y = ?apk_g1.Y,
                            apk_g2_x = ?apk_g2_point.X,
                            apk_g2_y = ?apk_g2_point.Y,
                            asig_x = ?asig_g1.X,
                            asig_y = ?asig_g1.Y,
                            "aggregated signatures",
                        );
                        println!(r#"[eth verification] cast c -r https://eth.llamarpc.com 0xb7ba8bbc36AA5684fC44D02aD666dF8E23BEEbF8 "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))" "{:?}" "({:?},{:?})" "({:?},{:?})" "({:?},{:?})""#, hex(&payload), apk_g1.X, apk_g1.Y, apk_g2_point.X, apk_g2_point.Y, asig_g1.X, asig_g1.Y);
                        
                        // Execute the vote with the aggregated signature
                        match self.execute_vote_with_aggregated_signature(
                            &payload,
                            &participating_g1,
                            &participating,
                            &signatures,
                            msg.round
                        ).await {
                            Ok(result) => {
                                info!(
                                    round = msg.round,
                                    "Successfully executed vote with aggregated signature. Result: {:?}",
                                    result
                                );
                            },
                            Err(e) => {
                                info!(
                                    round = msg.round,
                                    "Failed to execute vote with aggregated signature: {:?}",
                                    e
                                );
                            }
                        }
                    },
                }
            }
        }
    }

    pub async fn get_storage_updates(&self, block_number: u64) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the string to a Url
        let http_endpoint = env::var("HTTP_ENDPOINT")
            .expect("HTTP_ENDPOINT must be set");
        let url = Url::parse(&http_endpoint).unwrap();
        let provider: RootProvider = RootProvider::new_http(url);
        println!("block_number: {:?}", block_number);

        let contract_address: Address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        info!("Target address: {}", contract_address);
        
        let contract = VotingContract::new(contract_address, provider);
        
        let call_return = contract.operatorExecuteVote(U256::from(1))//TODO fix hardcoded
            .call()
            .await?;
        Ok(call_return._0)
    }

    pub async fn write_execute_vote(
        &self,
        msg_hash: FixedBytes<32>,
        apk: <crate::bindings::votingcontract::BN254::G1Point as alloy::sol_types::SolType>::RustType,
        apk_g2: <crate::bindings::votingcontract::BN254::G2Point as alloy::sol_types::SolType>::RustType,
        sigma: <crate::bindings::votingcontract::BN254::G1Point as alloy::sol_types::SolType>::RustType,
        storage_updates: Bytes,
        transition_index: U256,
        target_addr: Address,
        target_function: FixedBytes<4>
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the string to a Url
        let http_endpoint = env::var("HTTP_ENDPOINT")
            .expect("HTTP_ENDPOINT must be set");
        let url = Url::parse(&http_endpoint).unwrap();
        let provider: RootProvider = RootProvider::new_http(url);
        let contract_address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        
        let private_key = env::var("PRIVATE_KEY")
            .expect("PRIVATE_KEY must be set");
        let contract = VotingContract::new(contract_address, provider);
        let value = U256::from_str_radix("100000000000000000", 10).unwrap();
        
        // Debug logging for all parameters
        info!("Calling writeExecuteVote with parameters:");
        info!("msg_hash: {:?}", msg_hash);
        info!("apk: X={}, Y={}", apk.X, apk.Y);
        info!("apk_g2: X=[{}, {}], Y=[{}, {}]", apk_g2.X[0], apk_g2.X[1], apk_g2.Y[0], apk_g2.Y[1]);
        info!("sigma: X={}, Y={}", sigma.X, sigma.Y);
        info!("storage_updates length: {}", storage_updates.len());
        info!("transition_index: {}", transition_index);
        info!("target_addr: {}", target_addr);
        info!("target_function: {:?}", target_function);
        info!("value: {}", value);
        println!(r#"[eth verification] cast s --private-key 0xc7697fdc93ad14a4b17d4865f2736393a19ba4a10e6306a6d327ecf528b61ef6 0xFEDB17c4B3556d2D408C003D2e2cCeD28d4A9Cb3 --value 100000000000000000 "writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)" "{:?}" "({:?},{:?})" "({:?},{:?})" "({:?},{:?})" "{:?}" "{:?}" "{:?}" "{:?}""#, msg_hash, apk.X, apk.Y, apk_g2.X, apk_g2.Y, sigma.X, sigma.Y, storage_updates, transition_index, target_addr, target_function);
        // Execute the command using std::process::Command
        let output = std::process::Command::new("cast")
            .arg("s")
            .arg("--private-key")
            .arg("0xc7697fdc93ad14a4b17d4865f2736393a19ba4a10e6306a6d327ecf528b61ef6")
            .arg("0xFEDB17c4B3556d2D408C003D2e2cCeD28d4A9Cb3")
            .arg("--value")
            .arg("100000000000000000")
            .arg("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)")
            .arg(format!("{:?}", msg_hash))
            .arg(format!("({:?},{:?})", apk.X, apk.Y))
            .arg(format!("({:?},{:?})", apk_g2.X, apk_g2.Y))
            .arg(format!("({:?},{:?})", sigma.X, sigma.Y))
            .arg(format!("{:?}", storage_updates))
            .arg(format!("{:?}", transition_index))
            .arg(format!("{:?}", target_addr))
            .arg(format!("{:?}", target_function))
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            info!("Command execution failed: {}", error);
        } else {
            let success = String::from_utf8_lossy(&output.stdout);
            info!("Command executed successfully: {}", success);
        }
        // Use call() instead of send() to avoid needing a signer
        // let call_return = contract.writeExecuteVote(
        //     msg_hash,
        //     apk,
        //     apk_g2,
        //     sigma,
        //     storage_updates,
        //     transition_index,
        //     target_addr,
        //     target_function
        // )
        // .value(value)
        // .send()
        // .await?;
        
        // info!("Successfully called writeExecuteVote with transition index: {}", transition_index);
        Ok(Bytes::new())
    }

    pub async fn execute_vote_with_aggregated_signature(
        &self,
        payload_hash: &[u8],
        participating_g1: &[G1PublicKey],
        participating: &[PublicKey],
        signatures: &[Signature],
        block_number: u64
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Get the storage updates for the given block number
        let storage_updates = self.get_storage_updates(block_number).await?;
        
        // Get the points for the aggregated signature
        let (apk, apk_g2, asig) = bn254::get_points(participating_g1, participating, signatures).unwrap();
        
        // Convert to G1Point and G2Point types
        let apk_g1 = convert_to_g1_point(apk).unwrap();
        let apk_g2_point = convert_to_g2_point(apk_g2).unwrap();
        let asig_g1 = convert_to_g1_point(asig).unwrap();
        
        // Create G1Point and G2Point structs for the contract call
        let apk_struct = crate::bindings::votingcontract::BN254::G1Point {
            X: U256::from_str(&apk_g1.X.to_string()).unwrap(),
            Y: U256::from_str(&apk_g1.Y.to_string()).unwrap(),
        };
        
        let apk_g2_struct = crate::bindings::votingcontract::BN254::G2Point {
            X: [
                U256::from_str(&apk_g2_point.X[0].to_string()).unwrap(),
                U256::from_str(&apk_g2_point.X[1].to_string()).unwrap(),
            ],
            Y: [
                U256::from_str(&apk_g2_point.Y[0].to_string()).unwrap(),
                U256::from_str(&apk_g2_point.Y[1].to_string()).unwrap(),
            ],
        };
        
        let sigma_struct = crate::bindings::votingcontract::BN254::G1Point {
            X: U256::from_str(&asig_g1.X.to_string()).unwrap(),
            Y: U256::from_str(&asig_g1.Y.to_string()).unwrap(),
        };
        
        // Convert payload hash to FixedBytes<32>
        let mut msg_hash_bytes = [0u8; 32];
        if payload_hash.len() >= 32 {
            msg_hash_bytes.copy_from_slice(&payload_hash[0..32]);
        } else {
            msg_hash_bytes[0..payload_hash.len()].copy_from_slice(payload_hash);
        }
        let msg_hash = FixedBytes::<32>::from(msg_hash_bytes);
        
        // Debug print the raw hex of the message hash
        info!("Message hash raw bytes: {}", hex(payload_hash));
        info!("Message hash as FixedBytes<32>: {:?}", msg_hash);
        
        // Get target address and function selector
        let target_addr = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        
        // Function selector for the target function
        let target_function = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
        // Call the writeExecuteVote function
        self.write_execute_vote(
            msg_hash,
            apk_struct,
            apk_g2_struct,
            sigma_struct,
            storage_updates,
            U256::from(1), // TODO fix hardcoded.
            target_addr,
            target_function
        ).await
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
