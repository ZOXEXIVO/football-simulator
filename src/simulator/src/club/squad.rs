use crate::club::tactics::Tactics;
use crate::player::{Player, PlayerPosition};

pub struct Squad {
    pub tactics: Tactics,
    pub players: Vec<(PlayerPosition, Player)>
}
