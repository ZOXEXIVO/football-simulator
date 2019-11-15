use std::fmt::{Formatter, Display, Result};
use crate::club::Club;
use crate::r#match::simulation::{FootballMatch};

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
       
        let details = FootballMatch::play(home_players, away_players);

        for player_change in details.player_changes{

        }

        MatchResult{
            original_game: self,
            home_goals: details.score.home,
            away_goals: details.score.away
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
            write!(f, "{} {}:{} {}", 
            self.original_game.home_club.name, 
            self.home_goals, 
            self.away_goals,
            self.original_game.away_club.name)
      }
}