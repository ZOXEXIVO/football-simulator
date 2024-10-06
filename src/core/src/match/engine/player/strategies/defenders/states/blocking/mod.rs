use std::sync::LazyLock;
use rand::Rng;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::player::events::PlayerUpdateEvent;
use std::f32::consts::PI;

static DEFENDER_BLOCKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_blocking_data.json")));

const BLOCK_DISTANCE_THRESHOLD: f32 = 2.0; // Maximum distance to attempt a block (in meters)
const BLOCK_ANGLE_THRESHOLD: f32 = PI / 6.0; // Maximum angle (30 degrees) between defender and shot/pass direction
const STAMINA_THRESHOLD: f32 = 20.0; // Minimum stamina to attempt a block
const BLOCK_SUCCESS_BASE_CHANCE: f32 = 0.5; // Base chance of successful block

#[derive(Default)]
pub struct DefenderBlockingState {}

impl StateProcessingHandler for DefenderBlockingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check defender's stamina
        let stamina = ctx.player.player_attributes.condition_percentage() as f32;
        if stamina < STAMINA_THRESHOLD {
            // Transition to Resting state if stamina is too low
            return Some(StateChangeResult::with_defender_state(DefenderState::Resting));
        }

        // 2. Check if there is a shot or pass being made
        // For simplicity, let's assume we have access to the ball's recent velocity change
        let ball_velocity = ctx.tick_context.object_positions.ball_velocity;
        if ball_velocity.magnitude() < 0.1 {
            // Ball is not moving significantly; no shot or pass to block
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        // 3. Calculate the defender's position relative to the ball's path
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let defender_position = ctx.player.position;

        // Calculate vector from defender to ball
        let defender_to_ball = ball_position - defender_position;
        let distance_to_ball = defender_to_ball.magnitude();

        if distance_to_ball > BLOCK_DISTANCE_THRESHOLD {
            // Defender is too far to block
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        // 4. Calculate the angle between the defender's position and the ball's movement direction
        let ball_direction = ball_velocity.normalize();
        let defender_direction = defender_to_ball.normalize();
        let angle = defender_direction.angle(&ball_direction);

        if angle > BLOCK_ANGLE_THRESHOLD {
            // Defender is not in the path of the ball
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        // 5. Attempt to block the ball
        let block_success = self.attempt_block(ctx);

        if block_success {
            // Block is successful
            let mut state_change = StateChangeResult::with_defender_state(DefenderState::Standing);

            // Change the ball's velocity to simulate the block (e.g., deflect it)
            // For simplicity, we'll invert the ball's velocity and reduce its speed
            let new_ball_velocity = -ball_velocity * 0.5; // Reduce speed by half

            state_change.events.add(PlayerUpdateEvent::MoveBall(ctx.player.id, new_ball_velocity));

            // Optionally reduce defender's stamina
            // ctx.player.player_attributes.reduce_stamina(block_stamina_cost);

            Some(state_change)
        } else {
            // Block failed; transition to appropriate state
            Some(StateChangeResult::with_defender_state(DefenderState::Standing))
        }
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Defender may need to adjust position slightly to attempt block
        // Calculate minimal movement towards the blocking position
        // For simplicity, we'll assume the defender remains stationary
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}

impl DefenderBlockingState {
    /// Determines if the defender successfully blocks the ball based on skills and random chance.
    fn attempt_block(&self, ctx: &StateProcessingContext) -> bool {
        // Get defender's blocking-related skills
        let bravery = ctx.player.skills.mental.bravery as f32 / 100.0; // Normalize to [0,1]
        let positioning = ctx.player.skills.mental.positioning as f32 / 100.0;
        let tackling = ctx.player.skills.technical.tackling as f32 / 100.0;

        let overall_skill = (bravery + positioning + tackling) / 3.0;

        // Simulate chance of success
        let random_value: f32 = rand::thread_rng().gen(); // Generates a random float between 0.0 and 1.0

        overall_skill > (random_value + (1.0 - BLOCK_SUCCESS_BASE_CHANCE))
    }
}
