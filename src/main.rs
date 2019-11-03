extern crate crossbeam_channel;
extern crate simulator;

use simulator::TimeEstimation;

use simulator::{FootballSimulator, SimulationContext};

extern crate chrono;
pub use chrono::prelude::NaiveDate;

fn main() {
    let mut simulator = FootballSimulator::new(num_cpus::get());

    let generation_elapsed = TimeEstimation::estimate(|| simulator.generate());

    println!("generated with {} ms", generation_elapsed);

    let mut context = SimulationContext::new(NaiveDate::from_ymd(2020, 11, 15));
    
    let total_items = simulator.items_count();

    println!("running with {} items", total_items);

    loop {
        let simulation_elapsed = TimeEstimation::estimate(|| simulator.simulate(&mut context));

        println!("simulated with {} ms", simulation_elapsed);
    }
}
