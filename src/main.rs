extern crate chrono;
extern crate rayon;

mod simulator;

mod club;
mod continent;
mod country;
mod league;
mod r#match;
mod people;
mod server;
mod transfers;

mod shared;
mod utils;

mod generators;

use club::*;
use country::*;
use simulator::FootballSimulator;

use crate::server::Server;
use crate::simulator::SimulatorData;
use crate::utils::TimeEstimation;

#[actix_rt::main]
async fn main() {
    let server = Server::new("0.0.0.0:18000");

    server.start().await;

    // let mut data_estimation = TimeEstimation::estimate(SimulatorData::generate);
    //
    // println!("data generated with {} ms", data_estimation.1);
    //
    // let mut simulator = FootballSimulator::new();
    //
    // loop {
    //     let simulation_result =
    //         TimeEstimation::estimate(|| simulator.simulate(&mut data_estimation.0));
    //
    //     println!("simulated with {} ms", simulation_result.1);
    // }
}
