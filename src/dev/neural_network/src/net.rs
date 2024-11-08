use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
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
                    _ => idx - 1
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

    pub fn save_json(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize")
    }
    
    pub fn run(&self, inputs: &[f64]) -> Vec<f64> {
        let mut results = self.run_internal(inputs);

        results.pop().expect("failed to pop results from network")
    }

    pub(crate) fn run_internal(&self, inputs: &[f64]) -> Vec<Vec<f64>> {
        let mut results = Vec::with_capacity(self.layers.len() + 1);
        results.push(inputs.to_vec());

        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let current_result = &results[layer_idx];
            let mut layer_results = vec![0.0; layer.neurons.len()];

            layer.neurons.iter().enumerate().for_each(|(i, neuron)| {
                let mut total = neuron.weights[0];

                for (&weight, &value) in neuron.weights.iter().skip(1).zip(current_result) {
                    total += weight * value;
                }

                layer_results[i] = self.activate(total);
            });

            results.push(layer_results);
        }

        results
    }

    #[inline]
    fn activate(&self, x: f64) -> f64 {
        match self.activation_func {
            ActivationFunction::Sigmoid => sigmoid(x),
            ActivationFunction::Tanh => tanh(x),
            ActivationFunction::Relu => relu(x),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: u32, inputs: u32) -> Layer {
         Layer {
            // neurons
            neurons: (0..neurons).map(|n| {
                Neuron::new(inputs)
            }).collect()
        }
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

#[derive(Serialize, Deserialize)]
pub struct Neuron {
    pub weights: Vec<f64>
}

impl Neuron {
    pub fn new(inputs: u32) -> Self {
        Neuron {
            weights: (0..=inputs).map(|w| random_f64()).collect()
        }
    }
}

fn random_f64() -> f64 {
    2f64 * rand::random::<f64>() - 1f64
}

// Activations
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ActivationFunction {
    Sigmoid,
    Tanh,
    Relu
}

#[inline]
pub fn sigmoid(x: f64) -> f64 {
    1f64 / (1f64 + (-x).exp())
}

#[inline]
pub fn sigmoid_derive(x: f64) -> f64 {
    sigmoid(x) * (1.0 - sigmoid(x))
}

#[inline]
pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

#[inline]
pub fn tanh_derive(x: f64) -> f64 {
    1.0 - x.tanh().powi(2)
}

#[inline]
pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

#[inline]
pub fn relu_derive(x: f64) -> f64 {
    if x <= 0.0 { 0.0 } else { 1.0 }
}