use crate::common::NeuralNetwork;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};
use crate::r#match::position::VectorExtensions;

lazy_static! {
    static ref PLAYER_RETURNING_STATE_NETWORK: NeuralNetwork = PlayerReturningStateNetLoader::load();
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


        if player.position.distance_to(&player.start_position) < 10.0 {
            return Some(PlayerState::Standing);
        }

        None
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