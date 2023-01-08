use crate::club::{Player, PlayerPositionType};
use crate::r#match::FieldPosition;
use crate::Tactics;

#[derive(Debug)]
pub struct Squad<'s> {
    pub team_id: u32,
    pub tactics: Tactics,
    pub main_squad: Vec<SquadPlayer<'s>>,
    pub substitutes: Vec<SquadPlayer<'s>>,
}

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
