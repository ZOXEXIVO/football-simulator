use crate::club::Club;
use crate::r#match::engine::PlayerChanges;
use crate::r#match::FootballEngine;
use std::fmt::{Result};

#[derive(Clone)]
pub struct Match<'m> {
    home_club: &'m Club,
    away_club: &'m Club,
}

impl<'m> Match<'m> {
    pub fn make(home_club: &'m Club, away_club: &'m Club) -> Self {
        Match {
            home_club,
            away_club,
        }
    }

    pub fn play(self) -> MatchResult {
        let mut engine = FootballEngine::new(
            self.home_club.get_match_squad(),
            self.away_club.get_match_squad(),
        );

        let play_result = engine.play();

        MatchResult {
            player_changes: play_result.player_changes,
            home_club_id: self.home_club.id,
            home_goals: play_result.score.home,
            away_club_id: self.away_club.id,
            away_goals: play_result.score.away,
        }
    }
}

pub struct MatchResult {
    pub player_changes: Vec<PlayerChanges>,
    pub home_club_id: u32,
    pub home_goals: u8,
    pub away_club_id: u32,
    pub away_goals: u8,
}