use crate::bn254::{G1PublicKey, PublicKey, Signature, Bn254, PrivateKey};
use crate::bn254;
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
use tracing::info;
use dotenv::dotenv;
use alloy::providers::{Provider, RootProvider};
use alloy_network::Ethereum;
use alloy_primitives::{Address, Bytes, U256, FixedBytes};
use alloy::sol;
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
        function yourFunc(uint256 block_number, address contract_address, bytes4 function_sig, bytes storage_updates) public returns (bytes memory);
    }
}


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
        let http_endpoint = "http://localhost:8545"; // "https://ethereum-holesky.publicnode.com";
        let provider: RootProvider = RootProvider::new_http(Url::parse(&http_endpoint).unwrap());
        
        let registry_coordinator_address: Address = Address::from_str(
            &env::var("REGISTRY_COORDINATOR_ADDRESS")
                .expect("REGISTRY_COORDINATOR_ADDRESS must be set")
        ).unwrap();
        info!("Registry coordinator address: {}", registry_coordinator_address);

        let target_address: Address = Address::from_str(
            &env::var("TARGET_ADDRESS")
                .expect("TARGET_ADDRESS must be set")
        ).unwrap();
        info!("Target address: {}", target_address);

        loop {
            // Generate payload
            let current_block_num = provider.get_block_number().await.unwrap();

            // Sign the timestamp hash with BN254
            let payload = self.signer.sign(None, &current_block_num.to_be_bytes());
            info!(round = current_block_num, msg = hex(&payload), "generated and signed message");

            // Broadcast payload
            let message = wire::Aggregation {
                round: current_block_num,
                payload: Some(wire::aggregation::Payload::Start(wire::Start {})),
            }
            .encode_to_vec()
            .into();
            sender
                .send(commonware_p2p::Recipients::All, message, true)
                .await
                .expect("failed to broadcast message");
            signatures.insert(current_block_num, HashMap::new());

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
                            continue;
                        };

                        // Check if round exists
                        let Ok(msg) = wire::Aggregation::decode(msg) else {
                            continue;
                        };
                        let Some(round) = signatures.get_mut(&msg.round) else {
                            continue;
                        };

                        // Check if contributor has already signed
                        if round.contains_key(contributor) {
                            continue;
                        }

                        // Extract signature
                        let signature = match msg.payload {
                            Some(wire::aggregation::Payload::Signature(signature)) => signature.signature,
                            _ => continue,
                        };
                        let Ok(signature) = Signature::try_from(signature) else {
                            continue;
                        };

                        // Verify signature
                        let payload = msg.round.to_be_bytes();
                        hasher.update(&payload);
                        let payload = hasher.finalize();
                        if !Bn254::verify(None, &payload, &sender, &signature) {
                            continue;
                        }

                        // Insert signature
                        round.insert(contributor, signature);

                        // Check if should aggregate
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
                        let apk = convert_to_g1_point(apk).unwrap();
                        let apk_g2 = convert_to_g2_point(apk_g2).unwrap();
                        let asig = convert_to_g1_point(asig).unwrap();
                        info!(
                            round = msg.round,
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
                        println!(r#"[eth verification] cast c -r https://eth.llamarpc.com 0xb7ba8bbc36AA5684fC44D02aD666dF8E23BEEbF8 "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))" "{:?}" "({:?},{:?})" "({:?},{:?})" "({:?},{:?})""#, hex(&payload), apk.X, apk.Y, apk_g2.X, apk_g2.Y, asig.X, asig.Y);
                    },
                }
            }
        }
    }
    pub async fn get_storage_updates(&self, block_number: u64) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the string to a Url
        let url = Url::parse("http://localhost:8545").unwrap();
        let provider: RootProvider = RootProvider::new_http(url);
        println!("block_number: {:?}", block_number);
        let contract_address = Address::from_str("0xFEDB17c4B3556d2D408C003D2e2cCeD28d4A9Cb3").unwrap();
        
        let contract = VotingContract::new(contract_address, provider);
        
        let call_return = contract.operatorExecuteVote(U256::from(block_number))
            .call()
            .await?;
        Ok(call_return._0)
    }
}
