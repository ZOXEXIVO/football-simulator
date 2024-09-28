use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

const STAMINA_THRESHOLD: u32 = 20; // Minimum stamina percentage before resting

static MIDFIELDER_RETURNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_returning_data.json")));

#[derive(Default)]
pub struct MidfielderReturningState {}

impl StateProcessingHandler for MidfielderReturningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the midfielder has reached their starting position
        if ctx.player().distance_from_start_position() < 10.0 {
            // Transition to Standing state
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Standing,
            ));
        }

        // 2. Check if the ball is moving towards the player and is close
        if ctx.ball().is_towards_player() && ctx.ball().distance() < 50.0 {
            // Transition to Tackling state to attempt to regain possession
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Tackling,
            ));
        }

        // 3. Check if the team is losing and time is running out
        if ctx.player().is_team_loosing() && ctx.context.time.is_running_out() {
            // Transition to SupportingAttack to push forward
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::SupportingAttack,
            ));
        }

        // 4. Check if the player's stamina is low
        if ctx.player.player_attributes.condition_percentage() < STAMINA_THRESHOLD {
            // Transition to Resting state
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Resting,
            ));
        }

        // 5. Continue returning to position
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.player.start_position,
                slowing_distance: 10.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}
