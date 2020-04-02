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

mod shared;
mod utils;

mod generators;

use crate::core::SimulationContext;

use club::*;
use country::*;
use simulator::FootballSimulator;

use crate::generators::Generator;
use crate::simulator::SimulatorData;
use crate::utils::TimeEstimation;
use chrono::prelude::{NaiveDateTime, NaiveTime};

fn main() {
    let mut data_estimation = TimeEstimation::estimate(SimulatorData::generate);

    println!("data generated with {} ms", data_estimation.1);

    let date = NaiveDate::from_ymd(2020, 11, 15);
    let time = NaiveTime::from_hms(0, 0, 0);

    let mut context = SimulationContext::new(NaiveDateTime::new(date, time));

    let mut simulator = FootballSimulator::new();

    loop {
        let simulation_result =
            TimeEstimation::estimate(|| simulator.simulate(&mut data_estimation.0, &mut context));

        println!("simulated with {} ms", simulation_result.1);
    }
}
