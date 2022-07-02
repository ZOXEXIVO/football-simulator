use chrono::NaiveDateTime;

const DEFAULT_MATCH_LIST_SIZE: usize = 10;

#[derive(Debug)]
pub struct MatchHistory {
    items: Vec<MatchHistoryItem>,
}

impl MatchHistory {
    pub fn new() -> Self {
        MatchHistory {
            items: Vec::with_capacity(DEFAULT_MATCH_LIST_SIZE),
        }
    }

    pub fn add(&mut self, item: MatchHistoryItem) {
        self.items.push(item);
    }
}

#[derive(Debug)]
pub struct MatchHistoryItem {
    date: NaiveDateTime,
    rival_team_id: u32,
    score: (i32, i32),
}

impl MatchHistoryItem {
    pub fn new(date: NaiveDateTime, rival_team_id: u32, score: (i32, i32)) -> Self {
        MatchHistoryItem {
            date,
            rival_team_id,
            score,
        }
    }
}
