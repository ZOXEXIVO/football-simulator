use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;

static GOALKEEPER_PRESAVE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_presave_data.json")));

#[derive(Default)]
pub struct GoalkeeperPreSaveState {}

impl StateProcessingHandler for GoalkeeperPreSaveState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().distance() > 150.0 {
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::Walking));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let player_acceleration = ctx.player.skills.physical.acceleration;

        // Get current positions
        let player_position = ctx.player.position;
        let ball_position = ctx.tick_context.objects_positions.ball_position;

        // Calculate the direction vector towards the ball
        let direction_to_ball = (ball_position - player_position).normalize();
        let player_velocity = (direction_to_ball * player_acceleration).normalize();

        Some(
            SteeringBehavior::Pursuit {
                target: ctx.tick_context.objects_positions.ball_position,
                velocity: player_velocity,
            }
                .calculate(ctx.player)
                .velocity,
        )
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}
