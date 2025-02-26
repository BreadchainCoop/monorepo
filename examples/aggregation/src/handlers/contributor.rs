use commonware_cryptography::{Hasher, Scheme, Sha256};
use commonware_p2p::{Receiver, Sender};
use commonware_utils::hex;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use prost::Message;
use YourContract::yourFuncCall;
use std::{collections::{HashMap, HashSet}, str::FromStr};
use tracing::info;
use alloy::{hex::ToHexExt, json_abi::Function, providers::RootProvider, sol, sol_types::SolCall};
use crate::bn254::{self, aggregate_signatures, aggregate_verify, Bn254, G1PublicKey, PublicKey, Signature};
use alloy_primitives::{Address, Bytes, U256, FixedBytes};
use url;

// use alloy_provider::ProviderBuilder;
// Import the generated binding for VotingContract
use crate::bindings::votingcontract::VotingContract;
use url::Url;

use super::wire;
sol! {
    contract YourContract {
        #[derive(Debug)]
        function yourFunc(uint256 block_number, address contract_address, bytes4 function_sig, bytes storage_updates) public returns (bytes memory);
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
                let block_number = round;
                let contract_address = Address::from_str("0xFEDB17c4B3556d2D408C003D2e2cCeD28d4A9Cb3").unwrap();
                let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
                let storage_updates = self.get_storage_updates(block_number).await.unwrap();
                let block_number = U256::from(block_number);
                println!("storage_updates: {:?}", storage_updates);
                println!("block_number: {:?}", block_number);
                let encoded = yourFuncCall{
                    block_number,
                    contract_address,
                    function_sig,
                    storage_updates,
                }.abi_encode();
                println!("encoded: {:?}", encoded.encode_hex());
                // Generate signature
                let payload = encoded;
                hasher.update(&payload);
                let payload = hasher.finalize();
                println!("hash: {:?}", payload);
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
                        println!("Partcipating len: {:?}", participating.len());
                        println!(r#"[eth verification] cast c -r https://eth.llamarpc.com 0xb7ba8bbc36AA5684fC44D02aD666dF8E23BEEbF8 "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))" "{:?}" "({:?},{:?})" "({:?},{:?})" "({:?},{:?})""#, hex(&payload), apk.X, apk.Y, apk_g2.X, apk_g2.Y, asig.X, asig.Y);
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
            let block_number = message.round;
            let contract_address = Address::from_str("0xFEDB17c4B3556d2D408C003D2e2cCeD28d4A9Cb3").unwrap();
            let function_sig = Function::parse("writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)").unwrap().selector();
            let storage_updates = self.get_storage_updates(block_number).await.unwrap();
            println!("storage_updates: {:?}", storage_updates);
            println!("block_number: {:?}", block_number);
            println!("round: {:?}", round);
            let encoded = yourFuncCall{
                block_number: U256::from(block_number),
                contract_address,
                function_sig,
                storage_updates,
            }.abi_encode();
            // Generate signature
            let payload = encoded;
            hasher.update(&payload);
            let payload = hasher.finalize();
            let signature = self.signer.sign(None, &payload);

            // Store signature
            signatures
                .entry(round)
                .or_default()
                .insert(self.me, signature.clone());

            // Return signature to orchestrator
            let message = wire::Aggregation {
                round: block_number,
                payload: Some(wire::aggregation::Payload::Signature(wire::Signature {
                    signature: signature.to_vec(),
                })),
            }
            .encode_to_vec()
            .into();
            sender
                .send(commonware_p2p::Recipients::All, message, true)
                .await
                .expect("failed to broadcast signature");
            info!(round, "broadcast signature");
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
