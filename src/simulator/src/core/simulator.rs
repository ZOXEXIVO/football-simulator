use crate::models::country::Country;
use crate::core::context::SimulationContext;

use crate::generators::CountryGenerator;

use chrono::{ Duration };

pub struct FootballSimulator{
    thread_count: i32,
    coutries: Vec<Country>
}

impl FootballSimulator{
    pub fn new(thread_count: i32) -> Self{
        Self{
            thread_count: thread_count,
            coutries: CountryGenerator::generate(10)
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext){
        context.date = context.date + Duration::days(1);
    }
}