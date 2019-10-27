extern crate simulator;

use std::sync::Arc;

use simulator::{ FootballSimulator, SimulationContext };

extern crate indicatif;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

extern crate chrono;
pub use chrono::prelude::NaiveDate;

fn main() {
    let mut progress_bar = ProgressBar::new(1024);

    let progress = Arc::new(Box::new(move |progress| {
        progress_bar.inc(progress as u64);
    }));

    let mut simulator = FootballSimulator::new(num_cpus::get(), progress); 

    let mut context = SimulationContext::new(
        NaiveDate::from_ymd(2020, 11, 15)
    );

    loop {
        let count = simulator.simulate(&mut context);
    }
}
