use crate::r#match::player::MatchPlayer;
use crate::Tactics;

#[derive(Debug, Clone)]
pub struct TeamSquad {
    pub team_id: u32,
    pub team_name: String,
    pub tactics: Tactics,
    pub main_squad: Vec<MatchPlayer>,
    pub substitutes: Vec<MatchPlayer>,
}

impl TeamSquad {
    pub fn contains_player(&self, player_id: u32) -> bool {
        self.main_squad.iter().any(|p| p.id == player_id)
            || self.substitutes.iter().any(|p| p.id == player_id)
    }
}
