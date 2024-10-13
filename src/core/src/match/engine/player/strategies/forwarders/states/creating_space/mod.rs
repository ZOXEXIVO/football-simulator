use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::events::Event::PlayerEvent;
use crate::r#match::forwarders::states::ForwardState;

static FORWARD_CREATING_SPACE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_creating_space_data.json")));

const CREATING_SPACE_THRESHOLD: f32 = 50.0; // Adjust based on your game's scale
const OPPONENT_DISTANCE_THRESHOLD: f32 = 5.0; // Adjust based on your game's scale
const VELOCITY_CHANGE_THRESHOLD: f32 = 2.0; // Adjust based on your game's scale

#[derive(Default)]
pub struct ForwardCreatingSpaceState {}

impl StateProcessingHandler for ForwardCreatingSpaceState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the player has created enough space
        if self.has_created_space(ctx) {
            // If space is created, transition to the assisting state
            return Some(StateChangeResult::with_forward_state(ForwardState::Assisting));
        }

        // Check if the player is too close to an opponent
        if self.is_too_close_to_opponent(ctx) {
            // If too close to an opponent, try to dribble away
            return Some(StateChangeResult::with_forward_state(ForwardState::Dribbling));
        }

        // Check if the player should run to the opponent's side between opponents
        if self.should_run_to_opponent_side(ctx) {
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Check if the player should change velocity
        if self.should_change_velocity(ctx) {
            // If velocity change is needed, calculate the new velocity
            Some(self.calculate_new_velocity(ctx))
        } else {
            None
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {
        // No specific conditions to process
    }
}

impl ForwardCreatingSpaceState {
    fn has_created_space(&self, ctx: &StateProcessingContext) -> bool {
        let nearest_opponent = ctx.tick_context.object_positions.player_distances
            .find_closest_opponent(ctx.player);

        if let Some((_, distance)) = nearest_opponent {
            distance > CREATING_SPACE_THRESHOLD
        } else {
            false
        }
    }

    fn is_too_close_to_opponent(&self, ctx: &StateProcessingContext) -> bool {
        let nearest_opponent = ctx.tick_context.object_positions.player_distances
            .find_closest_opponent(ctx.player);

        if let Some((_, distance)) = nearest_opponent {
            distance < OPPONENT_DISTANCE_THRESHOLD
        } else {
            false
        }
    }

    fn should_change_velocity(&self, ctx: &StateProcessingContext) -> bool {
        let player_velocity = ctx.player.velocity;
        let nearest_opponent = ctx.tick_context.object_positions.player_distances
            .find_closest_opponent(ctx.player);

        if let Some((_, distance)) = nearest_opponent {
            distance < VELOCITY_CHANGE_THRESHOLD
        } else {
            false
        }
    }

    fn calculate_new_velocity(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let player_position = ctx.player.position;
        let nearest_opponent = ctx.tick_context.object_positions.player_distances
            .find_closest_opponent(ctx.player);

        if let Some((opponent_id, _)) = nearest_opponent {
            let opponent_position = ctx.tick_context.object_positions.players_positions
                .get_player_position(opponent_id).unwrap();

            let direction_away_from_opponent = (player_position - opponent_position).normalize();
            direction_away_from_opponent * ctx.player.skills.max_speed()
        } else {
            ctx.player.velocity
        }
    }

    fn should_run_to_opponent_side(&self, ctx: &StateProcessingContext) -> bool {
        let player_position = ctx.player.position;
        let field_half_length = ctx.context.field_size.width as f32 / 2.0;

        player_position.x < field_half_length && self.has_space_between_opponents(ctx)
    }

    fn has_space_between_opponents(&self, ctx: &StateProcessingContext) -> bool {
        let nearest_opponents = ctx.tick_context.object_positions.player_distances
            .find_closest_opponents(ctx.player);

        if let Some(opponents) = nearest_opponents {
            if opponents.len() >= 2 {
                let opponent1_position = ctx.context.players.get(opponents[0].0).unwrap().position;
                let opponent2_position = ctx.context.players.get(opponents[1].0).unwrap().position;

                let distance_between_opponents = (opponent1_position - opponent2_position).magnitude();
                distance_between_opponents > CREATING_SPACE_THRESHOLD
            } else {
                false
            }
        } else {
            false
        }
    }
}