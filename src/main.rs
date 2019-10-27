extern crate simulator;

use simulator::{ FootballSimulator, SimulationContext };

extern crate indicatif;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

extern crate chrono;
pub use chrono::prelude::NaiveDate;

fn main() {
    //let pb = ProgressBar::new(1024);

    let mut simulator = FootballSimulator::new(num_cpus::get());

    let mut context = SimulationContext::new(
        NaiveDate::from_ymd(2015, 3, 14)
    );

    loop {
        let count = simulator.simulate(&mut context);

        println!("simulate, date: {0}", context.date);

        //pb.inc(1);

        //thread::sleep(Duration::from_millis(100));
    }
}
