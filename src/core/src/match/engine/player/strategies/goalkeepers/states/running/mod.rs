use crate::common::NeuralNetwork;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, StateChangeResult, SteeringBehavior};

lazy_static! {
    static ref PLAYER_RUNNING_STATE_NETWORK: NeuralNetwork = PlayerRunningStateNetLoader::load();
}

pub struct GoalkeeperRunningState {}

impl GoalkeeperRunningState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: BallMetadata,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        Self::check_collision(player, objects_positions, result);

        let to_ball_velocity = SteeringBehavior::Seek {
            target: objects_positions.ball_position,
        }.calculate(player).velocity;

        StateChangeResult::with_velocity(to_ball_velocity)
    }

    fn check_collision(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        if objects_positions
            .ball_position
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
