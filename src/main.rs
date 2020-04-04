extern crate chrono;
extern crate rayon;

use chrono::prelude::NaiveDate;

mod simulator;

mod club;
mod continent;
mod core;
mod country;
mod league;
mod r#match;
mod people;
mod transfers;

mod shared;
mod utils;

mod generators;

use club::*;
use country::*;
use simulator::FootballSimulator;

use crate::generators::Generator;
use crate::simulator::SimulatorData;
use crate::utils::TimeEstimation;

fn main() {
    let mut data_estimation = TimeEstimation::estimate(SimulatorData::generate);
    
    println!("data generated with {} ms", data_estimation.1);

    let mut simulator = FootballSimulator::new();

    loop {
        let simulation_result =
            TimeEstimation::estimate(|| simulator.simulate(&mut data_estimation.0));

        println!("simulated with {} ms", simulation_result.1);
    }
}
