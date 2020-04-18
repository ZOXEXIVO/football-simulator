use crate::club::{ClubResult};

pub struct LeagueResult{
    pub clubs: Vec<ClubResult>,
}

impl LeagueResult {
    pub fn new(clubs: Vec<ClubResult>) -> Self {
        LeagueResult {
            clubs
        }
    }
}