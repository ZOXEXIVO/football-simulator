use crate::common::NeuralNetwork;

use crate::r#match::strategies::goalkeepers::ball_heading_towards_goal;
use crate::r#match::{
    BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    StateChangeResult, SteeringBehavior,
};

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct GoalkeeperStandingState {}

impl GoalkeeperStandingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: BallMetadata,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if !ball_metadata.ball_is_on_player_home_side {
            return StateChangeResult::with_state(PlayerState::Walking);
        }

        if ball_metadata.ball_distance > 100.0 {
            return StateChangeResult::none();
        }

        if ball_metadata.ball_distance < 20.0 {
            return StateChangeResult::with_state(PlayerState::Tackling);
        }

        StateChangeResult::none()
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
