use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_logger;
use eigen_services_operatorsinfo::operator_info::OperatorInfoService;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_utils::middleware::operatorstateretriever::OperatorStateRetriever;
use eigen_common::get_provider;

use alloy_primitives::{Address, address, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_network::Ethereum;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point};

use url::Url;
use std::sync::Arc;

#[derive(Debug)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

#[derive(Debug)]
pub struct OperatorInfo {
    pub address: Address,
    pub stake: U256,
    pub pub_keys: Option<OperatorPubKeys>,
    pub socket: Option<String>,
    pub quorum_number: u8,
}

#[derive(Debug)]
pub struct QuorumInfo {
    pub quorum_number: u8,
    pub operator_count: usize,
    pub total_stake: U256,
    pub operators: Vec<OperatorInfo>,
}

/// source: https://github.com/Layr-Labs/eigenlayer-middleware
/// Contracts from middleware are supposed to be deployed for each AVS but
/// OperatorStateRetriever looks generic for everyone.
const OPERATOR_STATE_RETRIEVER_ADDRESS: Address =
    address!("D5D7fB4647cE79740E6e83819EFDf43fa74F8C31");

pub struct EigenStakingClient {
    http_endpoint: String,
    registry_coordinator_address: Address,
    registry_coordinator_deploy_block: u64,
    operator_info_service: Arc<OperatorInfoServiceInMemory>,
}

impl EigenStakingClient {
    pub async fn new(
        http_endpoint: String,
        ws_endpoint: String,
        registry_coordinator_address: Address,
        registry_coordinator_deploy_block: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_logger().clone(),
            registry_coordinator_address,
            OPERATOR_STATE_RETRIEVER_ADDRESS,
            http_endpoint.clone(),
        ).await?;
        
        let (operator_info_service, _rx) = OperatorInfoServiceInMemory::new(
            get_logger(),
            avs_registry_reader.clone(),
            ws_endpoint.clone(),
        ).await?;

        Ok(Self {
            http_endpoint,
            registry_coordinator_address,
            registry_coordinator_deploy_block,
            operator_info_service: Arc::new(operator_info_service),
        })
    }

    pub async fn get_operator_states(&self) -> Result<Vec<QuorumInfo>, Box<dyn std::error::Error>> {
        let provider: RootProvider<_, Ethereum> = RootProvider::new_http(Url::parse(&self.http_endpoint)?);
        let current_block_number = provider.get_block_number().await?;
        self.operator_info_service
            .query_past_registered_operator_events_and_fill_db(
                self.registry_coordinator_deploy_block,
                current_block_number
            ).await?;

        let provider = get_provider(&self.http_endpoint);
        let operator_state_retriever = OperatorStateRetriever::new(OPERATOR_STATE_RETRIEVER_ADDRESS, provider);
        let quorum_numbers: Vec<u8> = vec![0];
        let operators_state = operator_state_retriever
            .getOperatorState_0(
                self.registry_coordinator_address,
                quorum_numbers.into(),
                current_block_number.try_into().unwrap()
            )
            .call()
            .await?
            ._0;

        let mut quorum_infos = Vec::new();
        
        for (quorum_number, operators) in operators_state.iter().enumerate() {
            let mut quorum_operators = Vec::new();
            let mut total_stake = U256::ZERO;
            
            for op in operators {
                let stake = U256::from(op.stake);
                total_stake += stake;
                
                let pub_keys = if let Ok(info) = self.operator_info_service.get_operator_info(op.operator).await {
                    info.map(|keys| OperatorPubKeys {
                        g1_pub_key: keys.g1_pub_key,
                        g2_pub_key: keys.g2_pub_key,
                    })
                } else {
                    None
                };
                
                let socket = self.operator_info_service.get_operator_socket(op.operator).await.ok().flatten();
                
                quorum_operators.push(OperatorInfo {
                    address: op.operator,
                    stake,
                    pub_keys,
                    socket,
                    quorum_number: quorum_number as u8,
                });
            }
            
            for operator in &quorum_operators {
                println!("\n  Operator Address: {}", operator.address);
                println!("  Stake: {} wei", operator.stake);
                if let Some(ref keys) = operator.pub_keys {
                    println!("  Operator Public Keys: {:?}", keys);
                }
                if let Some(ref socket) = operator.socket {
                    println!("  Operator Socket: {}", socket);
                }
            }

            quorum_infos.push(QuorumInfo {
                quorum_number: quorum_number as u8,
                operator_count: operators.len(),
                total_stake,
                operators: quorum_operators,
            });
        }
        
        
        
        Ok(quorum_infos)
    }
}
