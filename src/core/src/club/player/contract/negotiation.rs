#[derive(Debug)]
pub enum ContractNegotiationStatus {
    PlayerRequestContract,
    PlayerRequestNewContract,
    RequestPlayerExpectations,
    PlayerMakeRequest(u32, u32),
}

#[derive(Debug)]
pub struct ContractNegotiationEngine {
    pub status: ContractNegotiationStatus,
}

impl ContractNegotiationEngine {
    pub fn new(status: ContractNegotiationStatus) -> Self {
        ContractNegotiationEngine { status }
    }

    pub fn request_contract() -> Self{
        ContractNegotiationEngine { status: ContractNegotiationStatus::PlayerRequestContract }
    }
    
    pub fn request_new_contract() -> Self{
        ContractNegotiationEngine { status: ContractNegotiationStatus::PlayerRequestNewContract }
    }
}
