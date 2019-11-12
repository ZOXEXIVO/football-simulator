use std::fmt::{Formatter, Display, Result};
use crate::club::Club;

#[derive(Clone)]
pub struct Match{
    home_club: Club,
    away_club: Club
}

impl Match {
    pub fn make(home_club: Club, away_club: Club) -> Self {
        Match {
            home_club: home_club,
            away_club: away_club
        }
    }

    pub fn play(self) -> MatchResult {
        let home_players = self.home_club.get_players_for_match();
        let away_players = self.home_club.get_players_for_match();
       
        MatchResult{
            original_game: self,
            home_goals: 0,
            away_goals: 0
        }
    }
}

#[derive(Clone)]
pub struct MatchResult {
    original_game: Match,
    home_goals: u32,
    away_goals: u32,
}

//DISPLAY
impl Display for MatchResult {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {}:{} {}", 
            self.original_game.home_club.name, 
            self.home_goals, 
            self.away_goals,
            self.original_game.away_club.name)
      }
}