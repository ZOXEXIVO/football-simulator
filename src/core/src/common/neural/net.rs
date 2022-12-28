use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
    activation_func: ActivationFunction,
}

impl NeuralNetwork {
    pub fn new(layers_configurations: &[u32], activation_func: ActivationFunction) -> Self {
        let layers_len = layers_configurations.len();

        let mut layers = Vec::with_capacity(layers_len);

        // enumerate by neurons per layers count
        // current layer imputs is equal to previous layer neurons count
        for idx in 0..layers_len {
            let current_neurons = layers_configurations[idx];
            let current_inputs = layers_configurations[{
                // case for first layer (no previous)
                match idx {
                    0 => idx,
                    _ => idx - 1,
                }
            }];

            layers.push(Layer::new(current_neurons, current_inputs));
        }

        NeuralNetwork {
            layers,
            activation_func,
        }
    }

    pub fn load_json(json: &str) -> NeuralNetwork {
        serde_json::from_str(json).unwrap()
    }

    pub fn run(&self, inputs: &[f64]) -> Vec<f64> {
        // run network
        let mut results = self.run_internal(inputs);

        // latest output will be at the end of vec
        results.pop().unwrap()
    }

    fn run_internal(&self, inputs: &[f64]) -> Vec<Vec<f64>> {
        let mut results = Vec::with_capacity(self.layers.len());

        // Fill first layer
        results.push(inputs.to_vec());

        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let mut layer_results = Vec::with_capacity(layer.neurons.len());

            // calculate weight * input
            for neuron in &layer.neurons {
                let mut total: f64 = neuron.weights[0];

                let current_result = &results[layer_idx];

                for (&weight, &value) in neuron.weights.iter().skip(1).zip(current_result) {
                    total += weight * value;
                }

                // write activated result
                layer_results.push(self.activate(total));
            }

            results.push(layer_results);
        }

        results
    }

    #[inline]
    fn activate(&self, x: f64) -> f64 {
        match self.activation_func {
            ActivationFunction::Sigmoid => sigmoid(x),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: u32, inputs: u32) -> Layer {
        let mut layer = Layer {
            // neurons
            neurons: (0..neurons).map(|n| Neuron::new(inputs)).collect(),
        };

        layer
    }
}

impl Index<u32> for Layer {
    type Output = Neuron;

    fn index(&self, index: u32) -> &Self::Output {
        &self.neurons[index as usize]
    }
}

impl IndexMut<u32> for Layer {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.neurons[index as usize]
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Neuron {
    pub weights: Vec<f64>,
}

impl Neuron {
    pub fn new(inputs: u32) -> Self {
        Neuron {
            weights: (0..=inputs).map(|w| random_f64()).collect(),
        }
    }
}

fn random_f64() -> f64 {
    2f64 * rand::random::<f64>() - 1f64
}

// Activations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivationFunction {
    Sigmoid,
}

#[inline]
pub fn sigmoid(x: f64) -> f64 {
    1f64 / (1f64 + (-x).exp())
}
