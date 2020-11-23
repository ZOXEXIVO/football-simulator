#[derive(Clone)]
pub struct LeagueContext<'l> {
    id: u32,
    pub club_ids: &'l[u32]
}

impl<'l> LeagueContext<'l> {
    pub fn new(id: u32, club_ids: &'l[u32]) -> Self {
        LeagueContext {
            id,
            club_ids
        }
    }
}
