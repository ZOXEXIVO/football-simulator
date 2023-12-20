use crate::common::NeuralNetwork;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

lazy_static! {
    static ref PLAYER_RETURNING_STATE_NETWORK: NeuralNetwork =
        PlayerReturningStateNetLoader::load();
}

pub struct ReturningState {}

impl ReturningState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        let mut res_vec = Vec::new();

        res_vec.push(objects_positions.ball_positions.x as f64);
        res_vec.push(objects_positions.ball_positions.y as f64);

        res_vec.push(objects_positions.ball_velocity.x as f64);
        res_vec.push(objects_positions.ball_velocity.y as f64);

        let res = PLAYER_RETURNING_STATE_NETWORK.run(&res_vec);

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

        // if player.position.distance_to(&player.start_position) < 10.0 {
        //     return Some(PlayerState::Standing);
        // }
        //
        // None
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_returning_data.json");

#[derive(Debug)]
pub struct PlayerReturningStateNetLoader;

impl PlayerReturningStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
