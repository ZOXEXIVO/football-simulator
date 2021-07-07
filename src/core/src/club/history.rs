use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct MatchHistory{
    date: NaiveDateTime,
    rival_team_id: u32,
    score: (i32, i32)
}

impl MatchHistory {
    pub fn new( date: NaiveDateTime,
                rival_team_id: u32,
                score: (i32, i32)) -> Self {
        MatchHistory {
            date,
            rival_team_id,
            score
        }
    }
}