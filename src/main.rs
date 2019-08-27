extern crate simulator;

use simulator::{ FootballSimulator, SimulationContext };

extern crate chrono;
pub use chrono::prelude::NaiveDate;

fn main() {
    let mut simulator = FootballSimulator::new(10);

    let mut context = SimulationContext::new(
        NaiveDate::from_ymd(2015, 3, 14)
    );

    loop {
        simulator.simulate(&mut context);
        println!("simulate, date: {0}", context.date);
    }
}
