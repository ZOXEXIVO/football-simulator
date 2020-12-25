use crate::club::{Club, Player};

#[derive(Debug)]
pub enum ContractNegotiationStatus {
    PlayerRequestNewContract,
    RequestPlayerExpectations,
    PlayerMakeRequest(u32, u32),
}

#[derive(Debug)]
pub struct ContractNegotiationEngine {
    status: ContractNegotiationStatus,
}

impl ContractNegotiationEngine {
    pub fn new(status: ContractNegotiationStatus) -> Self {
        ContractNegotiationEngine { status }
    }
}
