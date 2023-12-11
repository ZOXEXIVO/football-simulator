use crate::common::NeuralNetwork;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};
use crate::r#match::position::VectorExtensions;
use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_RUNNING_STATE_NETWORK: NeuralNetwork = PlayerRunningStateNetLoader::load();
}

pub struct RunningState {}

impl RunningState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        Self::check_collision(player, objects_positions, result);

        if in_state_time > 100 {

        }

        None
    }

    fn check_collision(player: &mut MatchPlayer, objects_positions: &MatchObjectsPositions, result: &mut Vec<PlayerUpdateEvent>) {
        if objects_positions
            .ball_positions
            .distance_to(&player.position)
            < 5.0
        {
            result.push(PlayerUpdateEvent::TacklingBall(player.player_id))
        }
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct PlayerRunningStateNetLoader;

impl PlayerRunningStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}