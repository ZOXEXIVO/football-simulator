use crate::common::NeuralNetwork;

#[derive(Debug)]
pub struct DefaultNeuralNetworkLoader;

impl DefaultNeuralNetworkLoader {
    pub fn load(nn_data: &'static str) -> NeuralNetwork {
        NeuralNetwork::load_json(nn_data)
    }
}
