use std::f32::consts::E;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use crate::net::{ActivationFunction, NeuralNetwork};
use crate::train::Trainer;
use rayon::prelude::*;

mod net;
mod train;

fn main() {
    // let mut net_configurations = Vec::new();
    //
    // let max_length = 2u32;
    //
    // for momentum in &[0.1f64, 0.15f64] {
    //     for rate in &[0.01,0.02,0.03,0.04] {
    //         for epochs in &[50000] {
    //             for first in 0..max_length {
    //                 for second in 0..max_length {
    //                     for third in 0..max_length {
    //                         for fourth in 0..max_length {
    //                             let mut configuration = Vec::with_capacity(5);
    //
    //                             if first > 0 {
    //                                 configuration.push(first);
    //                             }
    //
    //                             if second > 0 {
    //                                 configuration.push(second);
    //                             }
    //
    //                             if third > 0 {
    //                                 configuration.push(third);
    //                             }
    //
    //                             if fourth > 0 {
    //                                 configuration.push(fourth);
    //                             }
    //
    //                             net_configurations.push((configuration, epochs, rate, momentum)));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // let mut ratings: Mutex<Vec<(f64, Vec<u32>, (u32, f64, f64))>> = Mutex::new(Vec::new()));
    //
    // net_configurations.par_iter().for_each(|(configuration, &epochs, &rate, &momentum)| {
    //     let mut full_configuration = Vec::with_capacity(2 * configuration.len()));
    //
    //     full_configuration.push(2);
    //     full_configuration.extend(configuration);
    //     full_configuration.push(1);
    //
    //     let error = train(&full_configuration, epochs, rate, momentum);
    //
    //     ratings.lock().unwrap().push((error, full_configuration, (epochs, rate, momentum))));
    // });
    //
    // let mut ratings_lock = ratings.lock().unwrap();
    //
    // ratings_lock.sort_by(|(error, _, _), (next_error, _, _)| { error.partial_cmp(next_error).unwrap() });
    //
    // for (index, (error,rating , (epochs, rate, momentum))) in ratings_lock.iter().take(10).enumerate() {
    //     println!("{}) {:?} - {} (epochs: {}, rate: {}, momentum: {}", index + 1, rating, error, *epochs, *rate, *momentum);
    // }

    // let net_json = fs::read_to_string("neural.json")
    //     .expect("Should have been able to read the file");
    //
    // let net = NeuralNetwork::load_json(&net_json);

    // let training_data = [
    //     (vec![0f64, 0f64], vec![0f64]),
    //     (vec![0f64, 1f64], vec![1f64]),
    //     (vec![1f64, 0f64], vec![0f64]),
    //     (vec![1f64, 1f64], vec![1f64])
    // ];
    //

    let mut net = NeuralNetwork::new(&[4, 12, 32, 6], ActivationFunction::Sigmoid);

    let training_data = [
        (vec![211f64, 12f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 0f64, 1f64,0f64]),
        (vec![12f64, 12f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![2f64, 1211f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![1f64, 1f64, 12f64, 12f64], vec![1f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![212f64, 11f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![1f64, 1f64, 12f64, 12f64], vec![0f64, 2f64, 0f64, 0f64, 0f64,0f64]),
        (vec![11f64, 1211f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![13f64, 1f64, 154f64, 122f64], vec![1f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![212f64, 211f64, 124f64, 122f64], vec![0f64, 0f64, 0f64, 0f64, 1f64,0f64]),
        (vec![1f64, 12f64, 122f64, 112f64], vec![0f64, 0f64, 0f64, 0f64, 0f64,1f64])
    ];

    let error = NeuralNetwork::train(&mut net, &training_data, 0.06, 0.1, 1000000);

    println!("error: {}", error);

    // let json_net = net.save_json();
    //
    // let mut f = File::create("training_result.json").expect("Unable to create file");
    // f.write_all(json_net.as_bytes()).expect("Unable to write data");

    // for (input, output) in training_data {
    //     let results = net.run(&input);
    //
    //     println!("{} - {} = {}", input[0], input[1], results[0]);
    // }
}

fn train(configuration: &[u32], epochs: u32, learning_rate: f64, momentum: f64) -> f64 {
    let training_data = [
        (vec![211f64, 12f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 0f64, 1f64,0f64]),
        (vec![12f64, 12f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![2f64, 1211f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![1f64, 1f64, 12f64, 12f64], vec![1f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![212f64, 11f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![1f64, 1f64, 12f64, 12f64], vec![0f64, 2f64, 0f64, 0f64, 0f64,0f64]),
        (vec![11f64, 1211f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 1f64, 0f64,0f64]),
        (vec![13f64, 1f64, 154f64, 122f64], vec![1f64, 0f64, 0f64, 0f64, 0f64,0f64]),
        (vec![212f64, 211f64, 124f64, 122f64], vec![0f64, 0f64, 0f64, 0f64, 1f64,0f64]),
        (vec![1f64, 12f64, 122f64, 112f64], vec![0f64, 0f64, 0f64, 0f64, 0f64,1f64])
    ];

    let mut net = NeuralNetwork::new(&configuration, ActivationFunction::Sigmoid);

    net.train(&training_data, learning_rate, momentum, epochs)
}
