use crate::club::{Club, Player};

pub struct ContractImproveRequestNegotiation {}

pub enum ContractImproveRequestNegotiationResult {
    Complete,
}

impl ContractImproveRequestNegotiation {
    pub fn negotiate(club: &Club, player: &Player) -> ContractImproveRequestNegotiationResult {
        ContractImproveRequestNegotiationResult::Complete
    }
}
