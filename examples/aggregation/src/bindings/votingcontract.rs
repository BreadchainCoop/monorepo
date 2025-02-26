use alloy_sol_types::sol;

sol! {
    #[sol(abi)]
    contract VotingContract {
        function operatorExecuteVote(uint256 blockNumber) external view returns (bytes memory);
    }
}

pub use VotingContract::VotingContractInstance;
