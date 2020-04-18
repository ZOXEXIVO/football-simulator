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

use crate::server::Server;
use crate::utils::TimeEstimation;
use crate::simulator::{FootballSimulator, SimulatorData};

#[actix_rt::main]
async fn main() {
    let server = Server::new("0.0.0.0:18000");

    server.start().await;

    // let (mut data, generate_estimation) = TimeEstimation::estimate(SimulatorData::generate);
    //
    // println!("data generated with {} ms", generate_estimation);
    //
    // loop {
    //     let (_, result_estimation) =
    //         TimeEstimation::estimate(|| FootballSimulator::simulate(&mut data));
    //
    //     println!("simulated with {} ms", result_estimation);
    // }
}
