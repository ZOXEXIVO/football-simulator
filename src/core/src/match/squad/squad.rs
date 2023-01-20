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
