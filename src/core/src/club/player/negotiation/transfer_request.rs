use crate::club::{Club, Player};

pub struct TransferRequestNegotiation {}

pub enum TransferRequestNegotiationResult {
    Complete,
}

impl TransferRequestNegotiation {
    pub fn negotiate(club: &Club, player: &Player) -> TransferRequestNegotiationResult {
        //let coach = club.staffs.main_coach();

        TransferRequestNegotiationResult::Complete
    }
}
