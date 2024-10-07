use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerUpdateEvent;

static GOALKEEPER_DIVING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_diving_data.json")));

const DIVE_DURATION: f32 = 1.0; // Duration of dive in seconds
const RECOVERY_TIME: f32 = 1.5; // Time to recover after dive

#[derive(Default)]
pub struct GoalkeeperDivingState {}

impl StateProcessingHandler for GoalkeeperDivingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        let elapsed_time = ctx.in_state_time as f32 / 1000.0; // Convert to seconds

        if elapsed_time > DIVE_DURATION + RECOVERY_TIME {
            // Dive and recovery completed, signal state change
            //result.events.add(PlayerUpdateEvent::DiveCompleted(ctx.player.id));
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::Standing));
        }

        if elapsed_time <= DIVE_DURATION {
            // Still diving
            let dive_direction = self.calculate_dive_direction(ctx);
            let dive_position = ctx.player.position + dive_direction * self.calculate_dive_distance(ctx);
            result.events.add(PlayerUpdateEvent::MovePlayer(ctx.player.id, dive_position));
        } else {
            // In recovery phase
            if self.is_ball_caught(ctx) {
                result.events.add(PlayerUpdateEvent::CaughtBall(ctx.player.id));
            } else if self.is_ball_nearby(ctx) {
                result.events.add(PlayerUpdateEvent::ClaimBall(ctx.player.id));
            }
        }

        Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let elapsed_time = ctx.in_state_time as f32 / 1000.0; // Convert to seconds

        if elapsed_time <= DIVE_DURATION {
            // During dive, return dive velocity
            let dive_direction = self.calculate_dive_direction(ctx);
            let dive_speed = self.calculate_dive_speed(ctx);
            Some(dive_direction * dive_speed)
        } else {
            // After dive, no velocity (goalkeeper is on the ground or getting up)
            Some(Vector3::new(0.0, 0.0, 0.0))
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl GoalkeeperDivingState {
    fn calculate_dive_direction(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let to_ball = ball_position - ctx.player.position;
        to_ball.normalize()
    }

    fn calculate_dive_distance(&self, ctx: &StateProcessingContext) -> f32 {
        // Base the dive distance on the goalkeeper's jumping skill
        ctx.player.skills.physical.jumping * 0.1 // Adjust this multiplier as needed
    }

    fn calculate_dive_speed(&self, ctx: &StateProcessingContext) -> f32 {
        // Base the dive speed on the goalkeeper's reflexes and agility
        (ctx.player.skills.physical.acceleration + ctx.player.skills.physical.agility) * 0.05 // Adjust this multiplier as needed
    }

    fn is_ball_caught(&self, ctx: &StateProcessingContext) -> bool {
        let ball_distance = ctx.ball().distance();
        let catch_probability = ctx.player.skills.technical.first_touch / 100.0; // Using first_touch as a proxy for catching skill

        ball_distance < 1.0 && rand::random::<f32>() < catch_probability
    }

    fn is_ball_nearby(&self, ctx: &StateProcessingContext) -> bool {
        ctx.ball().distance() < 2.0 // Adjust this distance as needed
    }
}