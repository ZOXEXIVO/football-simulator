use crate::common::NeuralNetwork;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct DefenderStandingState {}

impl DefenderStandingState {
    pub fn process(
        in_state_time: u64,
        _player: &mut MatchPlayer,
        _context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if in_state_time > 20 {
            return Some(PlayerState::Walking);
        }

        let mut res_vec = Vec::new();

        res_vec.push(objects_positions.ball_position.x as f64);
        res_vec.push(objects_positions.ball_position.y as f64);

        res_vec.push(objects_positions.ball_velocity.x as f64);
        res_vec.push(objects_positions.ball_velocity.y as f64);

        let res = PLAYER_STANDING_STATE_NETWORK.run(&res_vec);

        let index_of_max_element = res
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        match index_of_max_element {
            0 => Some(PlayerState::Standing),
            1 => Some(PlayerState::Walking),
            2 => Some(PlayerState::Running),
            3 => Some(PlayerState::Tackling),
            4 => Some(PlayerState::Shooting),
            5 => Some(PlayerState::Passing),
            6 => Some(PlayerState::Returning),
            _ => None,
        }
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
