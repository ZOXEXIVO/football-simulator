#[derive(Clone)]
pub struct LeagueContext {
    id: u32
}

impl LeagueContext {
    pub fn new(id: u32) -> Self {
        LeagueContext {
            id
        }
    }
}
