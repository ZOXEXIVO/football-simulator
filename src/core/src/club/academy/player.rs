use crate::Player;

#[derive(Debug)]
pub struct AcademyPlayer {
    pub player: Player,
    pub completed: bool,
}

impl AcademyPlayer {
    pub fn from_player(player: Player) -> Self {
        AcademyPlayer {
            player,
            completed: false,
        }
    }
}
