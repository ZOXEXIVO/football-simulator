extern crate simulator;
extern crate crossbeam_channel;

use std::thread;

use simulator::{ FootballSimulator, SimulationContext };

extern crate indicatif;
use indicatif::{ProgressBar};

extern crate chrono;
pub use chrono::prelude::NaiveDate;

use crossbeam_channel::{unbounded, Receiver};

fn main() { 
    let (sender, reciever) = unbounded::<i32>();

    let mut simulator = FootballSimulator::new(num_cpus::get()); 

    run_recieved_thread(reciever);

    let mut context = SimulationContext::new(
        NaiveDate::from_ymd(2020, 11, 15)
    );
    
    let total_items = simulator.items_count();

    println!("running with {} items", total_items);

    loop {
        simulator.simulate(&mut context, &sender);
    }
}

fn run_recieved_thread(reciever: Receiver<i32>) {
    thread::spawn(move || {      
        let mut progress_bar = ProgressBar::new(900);

        loop{
            let recieved_val = reciever.recv().unwrap();

            if recieved_val == 0 {
                progress_bar = ProgressBar::new(900);
            }else{                
                progress_bar.inc(1);
            }
        }        
    });
}
