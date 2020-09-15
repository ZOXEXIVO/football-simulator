use crate::club::tactics::Tactics;
use crate::club::{PlayerPositionType, Player};

#[derive(Debug)]
pub struct Squad<'s> {
    pub club_id: u32,
    pub tactics: Tactics,
    pub players: Vec<SquadPlayer<'s>>,
}

impl<'s> Squad<'s> {}

#[derive(Debug)]
pub struct SquadPlayer<'p> {
    pub player: &'p Player,
    pub position: PlayerPositionType,
}

impl<'p> SquadPlayer<'p> {
    pub fn new(player: &'p Player, position: PlayerPositionType) -> Self {
        SquadPlayer { player, position }
    }
}
