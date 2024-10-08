use std::io::Write;
use std::sync::Mutex;
use crate::net::{ActivationFunction, NeuralNetwork};
use crate::train::Trainer;
use rayon::prelude::*;

mod net;
mod train;

fn train(configuration: &[u32], epochs: u32, learning_rate: f64, momentum: f64) -> f64 {
    let training_data = [
        (vec![211f64, 12f64, 12f64], vec![0f64, 0f64, 0f64, 0f64, 1f64,0f64]),
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

    let mut net = NeuralNetwork::new(&configuration, ActivationFunction::Relu);

    net.train(&training_data, learning_rate, momentum, epochs)
}

fn main() {
    let mut net_configurations = Vec::new();

    let max_length = 2u32;

    for momentum in &[0.1f64, 0.15f64, 0.2f64] {
        for rate in &[0.01] {
            for epochs in &[100000] {
                for first in 0..max_length {
                    for second in 0..max_length {
                        for third in 0..max_length {
                            for fourth in 0..max_length {
                                let mut configuration = Vec::with_capacity(5);

                                if first > 0 {
                                    configuration.push(first);
                                }

                                if second > 0 {
                                    configuration.push(second);
                                }

                                if third > 0 {
                                    configuration.push(third);
                                }

                                if fourth > 0 {
                                    configuration.push(fourth);
                                }

                                net_configurations.push((configuration, epochs, rate, momentum));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ratings: Mutex<Vec<(f64, Vec<u32>, (u32, f64, f64))>> = Mutex::new(Vec::new());

    net_configurations.par_iter().for_each(|(configuration, &epochs, &rate, &momentum)| {
        let mut full_configuration = Vec::with_capacity(2 * configuration.len());

        full_configuration.push(2);
        full_configuration.extend(configuration);
        full_configuration.push(1);

        let error = train(&full_configuration, epochs, rate, momentum);

        ratings.lock().unwrap().push((error, full_configuration, (epochs, rate, momentum)));
    });

    let mut ratings_lock = ratings.lock().unwrap();

    ratings_lock.sort_by(|(error, _, _), (next_error, _, _)| { error.partial_cmp(next_error).unwrap() });

    for (index, (error, data, (epochs, rate, momentum))) in ratings_lock.iter().take(10).enumerate() {
        println!("{}) {:?} - {} (epochs: {}, rate: {}, momentum: {})", index + 1, data, error, *epochs, *rate, *momentum);
    }
}
