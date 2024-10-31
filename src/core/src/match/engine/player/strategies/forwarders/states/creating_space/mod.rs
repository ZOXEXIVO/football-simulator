use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_CREATING_SPACE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_creating_space_data.json")));

const CREATING_SPACE_THRESHOLD: f32 = 30.0; // Adjust based on your game's scale
const OPPONENT_DISTANCE_THRESHOLD: f32 = 10.0; // Adjust based on your game's scale
const VELOCITY_CHANGE_THRESHOLD: f32 = 2.0; // Adjust based on your game's scale

#[derive(Default)]
pub struct ForwardCreatingSpaceState {}

impl StateProcessingHandler for ForwardCreatingSpaceState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.team().is_control_ball() {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Running,
            ));
        }

        // Check if the player has created enough space
        if self.has_created_space(ctx) {
            // If space is created, transition to the assisting state
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Assisting,
            ));
        }

        // Check if the player is too close to an opponent
        if self.is_too_close_to_opponent(ctx) {
            // If too close to an opponent, try to dribble away
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Dribbling,
            ));
        }

        // Check if the player should run to the opponent's side between opponents
        if self.should_run_to_opponent_side(ctx) {
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.tick_context.positions.ball.position,
                slowing_distance: 150.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No specific conditions to process
    }
}

impl ForwardCreatingSpaceState {
    fn has_created_space(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players().opponents().exists(CREATING_SPACE_THRESHOLD)
    }

    fn is_too_close_to_opponent(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players().opponents().exists(OPPONENT_DISTANCE_THRESHOLD)
    }

    fn should_run_to_opponent_side(&self, ctx: &StateProcessingContext) -> bool {
        let player_position = ctx.player.position;
        let field_half_length = ctx.context.field_size.width as f32 / 2.0;

        player_position.x < field_half_length && self.has_space_between_opponents(ctx)
    }

    fn has_space_between_opponents(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.players();
        let opponents = players.opponents();

        let mut opponents_all = opponents.all();

        if let Some(first) = opponents_all.next() {
            if let Some(second) = opponents_all.next() {
                let opponent1_position = first.position;
                let opponent2_position = second.position;

                let distance_between_opponents =
                    (opponent1_position - opponent2_position).magnitude();

                return distance_between_opponents > CREATING_SPACE_THRESHOLD;
            }
        }

        false
    }
}
