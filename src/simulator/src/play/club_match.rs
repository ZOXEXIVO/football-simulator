use std::cell::RefCell;
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
            home_goals: home_players as u32,
            away_goals: away_players as u32
        }
    }
}

pub struct MatchResult {
    home_goals: u32,
    away_goals: u32
}