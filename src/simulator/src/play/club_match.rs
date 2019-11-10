use std::fmt::{Formatter, Display, Result};
use crate::club::Club;

#[derive(Clone)]
pub struct Match{
    home_club: Club,
    away_club: Club
}

impl Match {
    pub fn make(home_club: Club, away_club: Club) -> Match {
        Match{
            home_club: home_club,
            away_club: away_club
        }
    }

    pub fn play(self) -> MatchResult {
        let home_players = self.home_club.players.len();
        let away_players = self.home_club.players.len();

        

        MatchResult{
            original_match: self,
            home_goals: home_players as u32,
            away_goals: away_players as u32
        }
    }
}

#[derive(Clone)]
pub struct MatchResult {
    original_match: Match,
    home_goals: u32,
    away_goals: u32,
}

//DISPLAY
impl Display for MatchResult {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {}:{} {}", 
            self.original_match.home_club.name, 
            self.home_goals, 
            self.away_goals,
            self.original_match.away_club.name)
      }
}