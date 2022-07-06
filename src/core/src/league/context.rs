#[derive(Clone)]
pub struct LeagueContext<'l> {
    pub id: u32,
    pub team_ids: &'l [u32],
}

impl<'l> LeagueContext<'l> {
    pub fn new(id: u32, team_ids: &'l [u32]) -> Self {
        LeagueContext { id, team_ids }
    }
}
