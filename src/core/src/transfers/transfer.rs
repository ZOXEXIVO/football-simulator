use crate::Player;

pub struct PlayerTransfer {
    pub player: Player,
    pub club_id: u32,
}

impl PlayerTransfer {
    pub fn new(player: Player, club_id: u32) -> Self {
        PlayerTransfer { player, club_id }
    }
}
