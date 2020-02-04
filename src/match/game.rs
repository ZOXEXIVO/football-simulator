use crate::club::Club;
use crate::r#match::FootballEngine;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Match<'c> {
    home_club: &'c Club,
    away_club: &'c Club,
}

impl<'c> Match<'c> {
    pub fn make(home_club: &'c Club, away_club: &'c Club) -> Self {
        Match {
            home_club,
            away_club,
        }
    }

    pub fn play(self) -> MatchResult<'c> {
        let mut engine = FootballEngine::new(
            self.home_club.get_match_squad(),
            self.away_club.get_match_squad(),
        );

        let play_result = engine.play();

        for player_change in play_result.player_changes {}

        MatchResult {
            original_game: self,
            home_goals: play_result.score.home,
            away_goals: play_result.score.away,
        }
    }
}

#[derive(Clone)]
pub struct MatchResult<'m> {
    original_game: Match<'m>,
    home_goals: u8,
    away_goals: u8,
}

//DISPLAY
impl<'m> Display for MatchResult<'m> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {}:{} {}",
            self.original_game.home_club.name,
            self.home_goals,
            self.away_goals,
            self.original_game.away_club.name
        )
    }
}
