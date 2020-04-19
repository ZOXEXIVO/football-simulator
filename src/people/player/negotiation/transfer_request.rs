use crate::club::Club;
use crate::people::Player;

pub struct TransferRequestNegotiation {}

pub enum TransferRequestNegotiationResult {
    Complete,
}

impl TransferRequestNegotiation {
    pub fn negotiate(club: &Club, player: &Player) -> TransferRequestNegotiationResult {
        let coach = club.staffs.main_coach();

        TransferRequestNegotiationResult::Complete
    }
}
