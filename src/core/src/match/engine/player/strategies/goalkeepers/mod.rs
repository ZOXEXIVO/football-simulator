mod states;

use crate::common::NeuralNetwork;
use crate::r#match::position::VectorExtensions;
use crate::r#match::strategies::goalkeepers::states::{
    GoalkeeperPassingState, GoalkeeperReturningState, GoalkeeperRunningState,
    GoalkeeperShootingState, GoalkeeperStandingState, GoalkeeperTacklingState,
    GoalkeeperWalkingState,
};
use crate::r#match::{
    BallMetadata, BallState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState,
    PlayerUpdateEvent, StateChangeResult,
};
use nalgebra::Vector3;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn calculate(
        in_state_time: u64,
        context: &mut MatchContext,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        let is_ball_home_size = match context.state.ball_state {
            Some(ball_state) => ball_state == BallState::HomeSide,
            None => false,
        };

        let ball_metadata = BallMetadata {
            // ball moving towards goal
            is_ball_heading_towards_goal: ball_heading_towards_goal(
                objects_positions.ball_position,
                player.start_position,
            ),
            // distance to ball
            ball_distance: objects_positions
                .ball_position
                .distance_to(&player.position),
            // is ball on the home side?
            ball_is_on_player_home_side: player.is_home && is_ball_home_size,
        };

        match player.state {
            PlayerState::Standing => GoalkeeperStandingState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Walking => GoalkeeperWalkingState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Running => GoalkeeperRunningState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Tackling => GoalkeeperTacklingState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Shooting => GoalkeeperShootingState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Passing => GoalkeeperPassingState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
            PlayerState::Returning => GoalkeeperReturningState::process(
                player,
                context,
                objects_positions,
                ball_metadata,
                in_state_time,
                result,
            ),
        }
    }
}

fn ball_heading_towards_goal(ball_position: Vector3<f32>, goal_position: Vector3<f32>) -> bool {
    let ball_to_goal = goal_position - ball_position;

    let ball_forward = Vector3::new(1.0, 0.0, 0.0);

    let dot_product = ball_to_goal.normalize().dot(&ball_forward);

    dot_product > 0.8
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct GoalkeepersNetLoader;

impl GoalkeepersNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
