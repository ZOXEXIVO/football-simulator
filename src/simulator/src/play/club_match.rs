use std::fmt::{Formatter, Display, Result};
use crate::club::Club;

pub struct Match<'c>{
    home_club: &'c Club,
    away_club: &'c Club
}

impl<'c> Match<'c> {
    pub fn make(home_club: &'c Club, away_club: &'c Club) -> Match<'c> {
        Match{
            home_club: home_club,
            away_club: away_club
        }
    }

    pub fn play(&mut self) -> MatchResult {
        let home_players = self.home_club.players.len();
        let away_players = self.home_club.players.len();

        MatchResult{
            original_match: self,
            home_goals: home_players as u32,
            away_goals: away_players as u32
        }
    }
}

pub struct MatchResult<'a> {
    original_match: &'a Match<'a>,
    home_goals: u32,
    away_goals: u32,
}

//DISPLAY
impl<'a> Display for MatchResult<'a> {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {}:{} {}", 
            self.original_match.home_club.name, 
            self.home_goals, 
            self.away_goals,
            self.original_match.away_club.name)
      }
}