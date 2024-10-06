#[derive(Clone)]
pub struct LeagueContext<'l> {
    pub id: u32,
    pub slug: String,
    pub team_ids: &'l [u32],
}

impl<'l> LeagueContext<'l> {
    pub fn new(id: u32, slug: String, team_ids: &'l [u32]) -> Self {
        LeagueContext { id, slug, team_ids }
    }
}
