use crate::club::tactics::Tactics;
use crate::player::{Player, PlayerPosition};

#[derive(Debug)]
pub struct Squad {
    pub tactics: Tactics,
    pub players: Vec<(PlayerPosition, Player)>
}

impl Squad{
    
}
