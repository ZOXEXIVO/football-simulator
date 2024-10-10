use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static DEFENDER_PUSHINGUP_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pushingup_data.json")));

const TACKLING_DISTANCE_THRESHOLD: f32 = 2.0; // Distance within which the defender can tackle
const PRESSING_DISTANCE_THRESHOLD: f32 = 20.0; // Max distance to consider pressing
const STAMINA_THRESHOLD: f32 = 30.0; // Minimum stamina to continue pressing
const FIELD_THIRD_THRESHOLD: f32 = 0.33; // One-third of the field width

#[derive(Default)]
pub struct DefenderPushingUpState {}

impl StateProcessingHandler for DefenderPushingUpState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let ball_ops = ctx.ball();

        // Check if the ball has moved back to the defensive half
        if ball_ops.on_own_side() {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::TrackingBack,
            ));
        }

        // Check if the team has lost possession
        if !self.is_team_in_control(ctx) {
            if let Some(opponent) = self.find_nearby_opponent(ctx) {
                let distance_to_opponent = ctx
                    .tick_context
                    .object_positions
                    .player_distances
                    .get(opponent.id, ctx.player.id)
                    .unwrap();
                // If very close, attempt to tackle
                if distance_to_opponent <= TACKLING_DISTANCE_THRESHOLD {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Tackling,
                    ));
                }

                // If within pressing distance and enough stamina, start pressing
                if distance_to_opponent <= PRESSING_DISTANCE_THRESHOLD
                    && ctx.player.skills.physical.stamina > STAMINA_THRESHOLD
                {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Pressing,
                    ));
                }
            }

            // If no immediate threat, transition to covering space
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Covering,
            ));
        }

        // Check if the defender has pushed up too far
        if self.is_too_advanced(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::HoldingLine,
            ));
        }

        // Stay in the pushing up state
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let optimal_position = self.calculate_optimal_pushing_up_position(ctx);
        let movement_vector = optimal_position - ctx.player.position;

        if movement_vector.magnitude() > 1.0 {
            Some(movement_vector.normalize() * ctx.player.skills.physical.acceleration)
        } else {
            None
        }
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}

impl DefenderPushingUpState {
    fn is_team_in_control(&self, ctx: &StateProcessingContext) -> bool {
        let teammates_with_ball = ctx.context.players.get_by_team(ctx.player.team_id);
        !teammates_with_ball.is_empty()
    }

    fn find_nearby_opponent<'a>(&self, ctx: &'a StateProcessingContext) -> Option<&'a MatchPlayer> {
        if let Some((opponent_id, _)) = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponent(ctx.player)
        {
            return ctx.context.players.get(opponent_id);
        }

        None
    }

    fn is_too_advanced(&self, ctx: &StateProcessingContext) -> bool {
        let field_width = ctx.context.field_size.width as f32;
        let attacking_third_boundary = field_width * (1.0 - FIELD_THIRD_THRESHOLD);

        ctx.player.position.x > attacking_third_boundary && !self.is_last_defender(ctx)
    }

    fn is_last_defender(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.player();
        let defenders = players.defenders();

        defenders
            .iter()
            .all(|d| d.position.x <= ctx.player.position.x)
    }

    fn calculate_optimal_pushing_up_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let player_position = ctx.player.position;
        let field_width = ctx.context.field_size.width as f32;
        let field_height = ctx.context.field_size.height as f32;

        // Calculate the center of the attacking third
        let attacking_third_center = Vector3::new(
            field_width * (1.0 - FIELD_THIRD_THRESHOLD / 2.0),
            field_height * 0.5,
            0.0,
        );

        // Find the average position of attacking teammates
        let attacking_teammates = ctx
            .context
            .players
            .get_by_team(ctx.player.team_id)
            .into_iter()
            .filter(|p| p.position.x > field_width * 0.5)
            .collect::<Vec<_>>();

        let avg_attacking_position = if !attacking_teammates.is_empty() {
            attacking_teammates
                .iter()
                .fold(Vector3::zeros(), |acc, p| acc + p.position)
                / attacking_teammates.len() as f32
        } else {
            attacking_third_center
        };

        // Calculate a position that supports the attack but doesn't push too far forward
        let support_position = (ball_position + avg_attacking_position) * 0.5;

        // Blend the support position with the attacking third center and the player's current position
        let optimal_position =
            (support_position * 0.6 + attacking_third_center * 0.3 + player_position * 0.1)
                .cap_magnitude(field_width * 0.8); // Limit how far forward the defender can go

        // Ensure the optimal position is within the field boundaries
        Vector3::new(
            optimal_position.x.clamp(field_width * 0.5, field_width),
            optimal_position.y.clamp(0.0, field_height),
            0.0,
        )
    }
}
