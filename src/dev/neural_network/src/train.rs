use std::iter::{Enumerate, Zip};
use std::ops::{Index, IndexMut};
use std::slice;
use crate::net::{Layer, NeuralNetwork};

pub trait Trainer {
    fn train(&mut self, training_data: &[(Vec<f64>, Vec<f64>)], learning_rate: f64, momentum: f64, epochs: u32) -> f64;
    fn error(&self, run_results: &Vec<Vec<f64>>, required_values: &Vec<f64>) -> f64;
    fn weight_updates(&self, results: &Vec<Vec<f64>>, targets: &Vec<f64>) -> Vec<Vec<Vec<f64>>>;
    fn updates_weights(&mut self, weight_updates: Vec<Vec<Vec<f64>>>, deltas: &mut Vec<Vec<Vec<f64>>>, learning_rate: f64, momentum: f64);
    fn initial_deltas(&self) -> Vec<Vec<Vec<f64>>>;
}

impl Trainer for NeuralNetwork{
    fn train(&mut self, training_data: &[(Vec<f64>, Vec<f64>)], learning_rate: f64, momentum: f64, epochs: u32) -> f64{
        let mut deltas = self.initial_deltas();
        let mut error_rate = 0f64;

        for epoch in 0..epochs {
            error_rate = 0f64;

            for (input, target) in training_data.iter() {
                // run network and calculate errors, then update weights
                let run_results = self.run_internal(&input);

                error_rate += self.error(&run_results, &target);

                 self.updates_weights(
                     self.weight_updates(&run_results, &target),
                     &mut deltas, learning_rate, momentum);
            }

            // if epoch % 1000 == 0 {
            //     println!("error rate = {}", error_rate);
            // }
        }

        error_rate
    }

    fn error(&self, run_results: &Vec<Vec<f64>>, required_values: &Vec<f64>) -> f64 {
        let mut error = 0f64;

        let output_layer_result = run_results.last().unwrap();

        for (&result, &target_output) in output_layer_result.iter().zip(required_values) {
            error += (target_output - result).powi(2);
        }

        error
    }

    fn weight_updates(&self, results: &Vec<Vec<f64>>, targets: &Vec<f64>) -> Vec<Vec<Vec<f64>>> {
        let mut network_errors: Vec<Vec<f64>> = Vec::new();
        let mut network_weight_updates = Vec::new();

        let layers = &self.layers;
        let network_results = &results[1..];

        let mut next_layer_nodes: Option<&Layer> = None;

        for (layer_index, (layer_nodes, layer_results)) in iter_zip_enum(layers, network_results).rev() {
            let prev_layer_results = &results[layer_index];
            let mut layer_errors = Vec::with_capacity(layer_nodes.neurons.len());
            let mut layer_weight_updates = Vec::with_capacity(layer_nodes.neurons.len());

            for (node_index, (neuron, &result)) in layer_nodes.neurons.iter().zip(layer_results).enumerate() {
                let mut node_weight_updates = Vec::with_capacity(neuron.weights.len());
                let mut node_error;

                if layer_index == layers.len() - 1 {
                    node_error = result * (1f64 - result) * (targets[node_index] - result);
                } else {
                    let mut sum = 0f64;
                    let next_layer_errors = &network_errors[network_errors.len() - 1];
                    for (next_node, &next_node_error_data) in next_layer_nodes.unwrap().neurons.iter().zip((next_layer_errors).iter()) {
                        sum += next_node.weights[node_index + 1] * next_node_error_data;
                    }
                    node_error = result * (1f64 - result) * sum;
                }

                for weight_index in 0..neuron.weights.len() {
                    let mut prev_layer_result;
                    if weight_index == 0 {
                        prev_layer_result = 1f64;
                    } else {
                        prev_layer_result = prev_layer_results[weight_index - 1];
                    }
                    let weight_update = node_error * prev_layer_result;

                    node_weight_updates.push(weight_update);
                }

                layer_errors.push(node_error);

                layer_weight_updates.push(node_weight_updates);
            }

            network_errors.push(layer_errors);
            network_weight_updates.push(layer_weight_updates);

            next_layer_nodes = Some(&layer_nodes);
        }

        network_weight_updates.reverse();
        network_weight_updates
    }

    fn updates_weights(&mut self, weight_updates: Vec<Vec<Vec<f64>>>, deltas: &mut Vec<Vec<Vec<f64>>>, learning_rate: f64, momentum: f64)  {
        for layer_index in 0..self.layers.len() {
            let layer = &mut self.layers[layer_index];
            let layer_weight_updates = &weight_updates[layer_index];

            for neuron_index in 0..layer.neurons.len() {
                let neuron = &mut layer.index_mut(neuron_index as u32);
                let neuron_weight_updates = &layer_weight_updates[neuron_index];

                for weight_index in 0..neuron.weights.len() {
                    let weight_update = neuron_weight_updates[weight_index];

                    let prev_delta = deltas[layer_index][neuron_index][weight_index];
                    let delta = (learning_rate * weight_update) + (momentum * prev_delta);

                    neuron.weights[weight_index] += delta;

                    deltas[layer_index][neuron_index][weight_index] = delta;
                }
            }
        }
    }

    fn initial_deltas(&self) -> Vec<Vec<Vec<f64>>> {
        let mut result = Vec::with_capacity(self.layers.len());

        for layer in &self.layers {
            let mut layer_stage = Vec::with_capacity(layer.neurons.len());

            for neuron in &layer.neurons {
                let mut neuron_stage = Vec::with_capacity(neuron.weights.len());

                for _ in 0..neuron.weights.len() {
                    neuron_stage.push(0f64);
                }

                layer_stage.push(neuron_stage);
            }

            result.push(layer_stage);
        }

        result
    }
}

fn iter_zip_enum<'s, 't, S: 's, T: 't>(s: &'s [S], t: &'t [T]) ->
Enumerate<Zip<slice::Iter<'s, S>, slice::Iter<'t, T>>> {
    s.iter().zip(t.iter()).enumerate()
}