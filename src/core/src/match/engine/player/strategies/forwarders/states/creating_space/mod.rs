use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::events::Event::PlayerEvent;
use crate::r#match::forwarders::states::ForwardState;

static FORWARD_CREATING_SPACE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_creating_space_data.json")));

const CREATING_SPACE_THRESHOLD: f32 = 10.0; // Adjust based on your game's scale
const OPPONENT_DISTANCE_THRESHOLD: f32 = 5.0; // Adjust based on your game's scale
const VELOCITY_CHANGE_THRESHOLD: f32 = 2.0; // Adjust based on your game's scale

#[derive(Default)]
pub struct ForwardCreatingSpaceState {}

impl StateProcessingHandler for ForwardCreatingSpaceState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

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
            // If running to the opponent's side is needed, calculate the target position
            let target_position = self.calculate_target_position(ctx);
            //result.events.add_player_event(PlayerEvent::(target_position));
        }

        // If no clear action, continue in the current state
        Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
        // let mut result = StateChangeResult::new();
        //
        // // Use the neural network to determine the best movement direction
        // let input = self.prepare_input(ctx);
        // let output = FORWARD_CREATING_SPACE_STATE_NETWORK.forward(&input);
        // let movement_direction = self.process_output(&output);
        //
        // // Set the player's velocity based on the movement direction
        // result.velocity = Some(movement_direction * ctx.player.skills.max_speed());
        //
        // Some(result)
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

    fn prepare_input(&self, ctx: &StateProcessingContext) -> Vec<f32> {
        // Prepare the input vector for the neural network
        // This could include player position, opponent positions, ball position, etc.
        vec![
            ctx.player.position.x,
            ctx.player.position.y,
            ctx.tick_context.object_positions.ball_position.x,
            ctx.tick_context.object_positions.ball_position.y,
            // Add more relevant input data
        ]
    }

    fn process_output(&self, output: &[f32]) -> Vector3<f32> {
        // Process the output of the neural network to determine the movement direction
        // This could involve interpreting the output as a direction vector
        Vector3::new(output[0], output[1], 0.0).normalize()
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

    fn calculate_target_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let player_position = ctx.player.position;
        let field_half_length = ctx.context.field_size.width as f32 / 2.0;
        let field_width = ctx.context.field_size.width as f32;

        let target_x = field_half_length + (field_half_length - player_position.x) * 0.8;
        let target_y = player_position.y + (field_width / 4.0) * (rand::random::<f32>() - 0.5);

        Vector3::new(target_x, target_y, 0.0)
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