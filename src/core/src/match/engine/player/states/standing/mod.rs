use crate::common::NeuralNetwork;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

use crate::r#match::position::VectorExtensions;
use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct StandingState {}

impl StandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if in_state_time > 20 {
            return Some(PlayerState::Walking);
        }

        let mut res_vec = Vec::new();

        res_vec.push(objects_positions.ball_positions.x as f64);
        res_vec.push(objects_positions.ball_positions.y as f64);

        res_vec.push(objects_positions.ball_velocity.x as f64);
        res_vec.push(objects_positions.ball_velocity.y as f64);

        let res = PLAYER_STANDING_STATE_NETWORK.run(&res_vec);

        if res[0] > 0.6 {
            return Some(PlayerState::Standing);
        }
        if res[1] > 0.6 {
            return Some(PlayerState::Walking);
        }
        if res[2] > 0.6 {
            return Some(PlayerState::Running);
        }
        if res[3] > 0.6 {
            return Some(PlayerState::Tackling);
        }
        if res[4] > 0.6 {
            return Some(PlayerState::Shooting);
        }
        if res[5] > 0.6 {
            return Some(PlayerState::Passing);
        }

        None
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_standing_data.json");

#[derive(Debug)]
pub struct PlayerStandingStateNetLoader;

impl PlayerStandingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}