use crate::core::context::SimulationContext;

use chrono::{ Duration };

pub struct FootballSimulator{
    thread_count: i32
}

impl FootballSimulator{
    pub fn new(thread_count: i32) -> Self{
        Self{
            thread_count: thread_count
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext){
        context.date = context.date + Duration::days(1);
    }
}