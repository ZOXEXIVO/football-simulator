use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::midfielders::states::MidfielderState;

static MIDFIELDER_PRESSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pressing_data.json")));

const PRESSING_DISTANCE_THRESHOLD: f32 = 10.0; // Max distance to consider pressing
const STAMINA_THRESHOLD: f32 = 50.0; // Minimum stamina to continue pressing

#[derive(Default)]
pub struct MidfielderPressingState {}

impl StateProcessingHandler for MidfielderPressingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the midfielder has enough stamina to continue pressing
        let stamina = ctx.player.player_attributes.condition_percentage() as f32;
        if stamina < STAMINA_THRESHOLD {
            // Transition to Standing state if stamina is low
            return Some(StateChangeResult::with_midfielder_state(MidfielderState::Standing));
        }

        // 2. Identify the opponent player with the ball
        let players = ctx.context.players.raw_players();
        let opponent_with_ball = players.iter()
            .find(|p| p.team_id != ctx.player.team_id && p.has_ball);

        if let Some(opponent) = opponent_with_ball {
            // 3. Calculate the distance to the opponent
            let distance_to_opponent = (ctx.player.position - opponent.position).magnitude();

            // 4. If the opponent is too far away, stop pressing
            if distance_to_opponent > PRESSING_DISTANCE_THRESHOLD {
                // Transition to Standing state
                return Some(StateChangeResult::with_midfielder_state(MidfielderState::Standing));
            }

            // 5. Continue pressing (no state change)
            None
        } else {
            // No opponent with the ball found (perhaps ball is free)
            // Transition to Standing state
            Some(StateChangeResult::with_midfielder_state(MidfielderState::Standing))
        }
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Move towards the opponent with the ball

        // Identify the opponent player with the ball
        let players = ctx.context.players.raw_players();
        let opponent_with_ball = players
            .iter()
            .find(|p| p.team_id != ctx.player.team_id && p.has_ball);

        if let Some(opponent) = opponent_with_ball {
            // Calculate direction towards the opponent
            let direction = (opponent.position - ctx.player.position).normalize();
            // Set speed based on player's acceleration and pace
            let speed = ctx.player.skills.physical.pace; // Use pace attribute
            Some(direction * speed)
        } else {
            // No opponent with the ball found
            // Remain stationary or move back to position
            Some(Vector3::new(0.0, 0.0, 0.0))
        }
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}