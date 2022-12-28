use crate::common::NeuralNetwork;

const NEURAL_NETWORK_DATA: &'static str = include_str!("neural_data.json");

#[derive(Debug)]
pub struct TrainingNetLoader;

impl TrainingNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
