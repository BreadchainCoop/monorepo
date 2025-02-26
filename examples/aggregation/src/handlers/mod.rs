mod contributor;
pub use contributor::Contributor;
mod orchestrator;
pub use orchestrator::Orchestrator;
pub use bindings::votingcontract::VotingContract;

use crate::bindings;
mod wire {
    include!(concat!(env!("OUT_DIR"), "/wire.rs"));
}
