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

use crate::utils::TimeEstimation;
use chrono::prelude::{NaiveDateTime, NaiveTime};

fn main() {
    let mut simulator = FootballSimulator::new();

    println!(
        "generated with {} ms",
        TimeEstimation::estimate(|| simulator.generate())
    );

    let date = NaiveDate::from_ymd(2020, 11, 15);
    let time = NaiveTime::from_hms(0, 0, 0);

    let mut context = SimulationContext::new(NaiveDateTime::new(date, time));

    let total_items = simulator.items_count();

    println!("running with {} items", total_items);

    loop {
        println!(
            "simulated with {} ms",
            TimeEstimation::estimate(|| simulator.simulate(&mut context))
        );
    }
}
