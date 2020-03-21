use crate::club::Club;
use crate::people::Player;

pub struct ContractImproveRequestNegotiation {}

pub enum ContractImproveRequestNegotiationResult {
    Complete,
}

impl ContractImproveRequestNegotiation {
    pub fn negotiate(club: &Club, player: &Player) -> ContractImproveRequestNegotiationResult {
        ContractImproveRequestNegotiationResult::Complete
    }
}
