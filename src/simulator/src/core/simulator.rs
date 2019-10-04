use crate::models::country::Country;
use crate::core::context::SimulationContext;
use crate::generators::Generator;

use chrono::{ Duration };

pub struct FootballSimulator{
    thread_count: i32,
    coutries: Vec<Country>
}

impl FootballSimulator{
    pub fn new(thread_count: i32) -> Self {      
        Self {
            thread_count: thread_count,
            coutries: (0..10).map(|_| Generator::generate()).collect()               
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> usize {
        let mut total_count = 0;

        for country in &mut self.coutries{
            country.simulate(context);

            total_count += country.items_count();
        }
        context.date = context.date + Duration::days(1);

        total_count
    }
}