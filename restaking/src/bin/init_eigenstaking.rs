use commonware_restaking::eigenlayer::EigenStakingClient;
use eigen_logging::{init_logger, log_level::LogLevel};
use alloy_primitives::address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger with INFO level
    init_logger(LogLevel::Info);
    let client = EigenStakingClient::new(
        String::from("https://eth.llamarpc.com"),
        String::from("wss://eth.drpc.org"),
        address!("0xeCd099fA5048c3738a5544347D8cBc8076E76494").into(),
        20227142,
    ).await?;
    
    println!("\nRetrieving operator states...");
    let quorum_infos = client.get_operator_states().await?;
    
    println!("\nSummary of Retrieved Data:");
    println!("Number of Quorums: {}", quorum_infos.len());
    
    for quorum in &quorum_infos {
        println!("\nQuorum {} Summary:", quorum.quorum_number);
        println!("Total Operators: {}", quorum.operator_count);
        println!("Total Stake: {} wei", quorum.total_stake);
        println!("Operators with Public Keys: {}", quorum.operators.iter().filter(|op| op.pub_keys.is_some()).count());
        println!("Operators with Sockets: {}", quorum.operators.iter().filter(|op| op.socket.is_some()).count());
    }
    
    println!("\nEigenStaking client initialized and operator states retrieved successfully!");
    Ok(())
} 