use crate::people::{Player, Staff};
use chrono::NaiveTime;

#[derive(Debug)]
pub struct Training{
    
}

impl Training{
    pub fn train(schedule: TrainingSchedule, players: Vec<Player>, coaches: Vec<Staff>){
        
    }
}

#[derive(Debug)]
pub struct TrainingSchedule{
    pub morning_time: NaiveTime,
    pub evening_time: NaiveTime,
}

impl TrainingSchedule{
    pub fn new(morning_time: NaiveTime, evening_time: NaiveTime) -> Self {
        TrainingSchedule{
            morning_time,
            evening_time
        }
    }
}