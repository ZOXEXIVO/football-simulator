use crate::models::country::Country;
use crate::core::context::SimulationContext;
use crate::generators::Generator;

use chrono::{ Duration };

pub struct FootballSimulator{
    thread_count: i32,
    coutries: Vec<Country>
}

impl FootballSimulator{
    pub fn new(thread_count: i32) -> Self{
             let n = 10;

            let mut vec = Vec::with_capacity(n);

            for i in 0..n {
                  vec.push(Generator::generate());
            }


        Self{
            thread_count: thread_count,
            coutries: vec
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext){
        for country in &mut self.coutries{
            country.simulate(context);
        }
        context.date = context.date + Duration::days(1);
    }
}