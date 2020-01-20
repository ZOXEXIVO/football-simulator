use crate::club::Club;
use crate::r#match::FootballEngine;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Match {
    home_club: Club,
    away_club: Club,
}

impl Match {
    pub fn make(home_club: Club, away_club: Club) -> Self {
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

        for player_change in play_result.player_changes {}

        MatchResult {
            original_game: self,
            home_goals: play_result.score.home,
            away_goals: play_result.score.away,
        }
    }
}

#[derive(Clone)]
pub struct MatchResult {
    original_game: Match,
    home_goals: u8,
    away_goals: u8,
}

//DISPLAY
impl Display for MatchResult {
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
