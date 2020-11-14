use crate::Club;

#[derive(Clone)]
pub struct LeagueContext {
    id: u32,
    club_ids: Vec<u32>
}

impl LeagueContext {
    pub fn new(id: u32, club_ids: Vec<u32>) -> Self {
        LeagueContext {
            id,
            club_ids
        }
    }
}
