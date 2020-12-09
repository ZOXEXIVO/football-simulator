use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct MatchHistory{
    date: NaiveDateTime,
    rival_team_id: u32,
    score: (u8, u8)
}

impl MatchHistory {
    pub fn new( date: NaiveDateTime,
                rival_team_id: u32,
                score: (u8, u8)) -> Self {
        MatchHistory {
            date,
            rival_team_id,
            score
        }
    }
}