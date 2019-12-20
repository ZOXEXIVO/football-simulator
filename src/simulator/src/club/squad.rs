use crate::club::tactics::Tactics;
use crate::player::{Player, PlayerPositionType};

#[derive(Debug)]
pub struct Squad {
    pub tactics: Tactics,
    pub players: Vec<(PlayerPositionType, Player)>
}

impl Squad{
    
}
