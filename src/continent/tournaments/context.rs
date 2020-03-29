use crate::continent::ContinentContext;
use chrono::NaiveDateTime;

pub struct TournamentContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl TournamentContext {
    pub fn new(context: &ContinentContext) -> Self {
        TournamentContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }
}
