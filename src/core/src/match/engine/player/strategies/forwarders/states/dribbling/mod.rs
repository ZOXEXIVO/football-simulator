use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    ConditionContext, MatchPlayer, PlayerSide, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::player::events::PlayerEvent;

static FORWARD_DRIBBLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_dribbling_data.json")));

#[derive(Default)]
pub struct ForwardDribblingState {}

impl StateProcessingHandler for ForwardDribblingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();
        let player_ops = ctx.player();

        // Check if the player has the ball
        if !ctx.player.has_ball {
            // Transition to Running state if the player doesn't have the ball
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if the player is under pressure
        if player_ops.is_under_pressure(ctx) {
            // Transition to Passing state if under pressure
            return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
        }

        // Check if there's space to dribble forward
        if !self.has_space_to_dribble(ctx) {
            // Transition to HoldingUpPlay state if there's no space to dribble
            return Some(StateChangeResult::with_forward_state(
                ForwardState::HoldingUpPlay,
            ));
        }

        // Check if there's an opportunity to pass to a teammate
        if let Some(teammate_id) = self.find_best_pass_option(ctx) {
            let teammate = &ctx.context.players.get(teammate_id)?;

            // Perform the pass
            result
                .events
                .add_player_event(PlayerEvent::RequestPass(ctx.player.id, teammate.id));

            // Transition to Running state after making the pass
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if there's an opportunity to shoot
        if self.can_shoot(ctx) {
            // Transition to Shooting state if there's an opportunity to shoot
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Shooting,
            ));
        }

        // Dribble towards the opponent's goal
        let direction = ctx.ball().direction_to_opponent_goal();

        result.velocity = Some(direction * ctx.player.skills.physical.acceleration);

        Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl ForwardDribblingState {
    fn has_space_to_dribble(&self, ctx: &StateProcessingContext) -> bool {
        let dribble_distance = 10.0; // Adjust based on your game's scale
        let players = ctx.player();
        let opponents = players.opponents();

        // Check if there are no opponents within the dribble distance
        opponents
            .iter()
            .all(|opponent| ctx.tick_context.object_positions.player_distances.get(ctx.player.id, opponent.id).unwrap() > dribble_distance)
    }

    fn find_best_pass_option(&self, ctx: &StateProcessingContext) -> Option<u32> {
        let teammates = ctx.context.players.get_by_team(ctx.player.team_id);

        teammates
            .iter()
            .enumerate()
            .filter(|(_, teammate)| {
                // Check if the teammate is in a good position to receive a pass
                let is_open = self.is_open_for_pass(ctx, teammate);
                let is_in_passing_lane = self.in_passing_lane(ctx, teammate);
                is_open && is_in_passing_lane
            })
            .max_by(|(_, a), (_, b)| {
                // Find the teammate with the highest scoring chance
                let score_a = self.scoring_chance(ctx, a);
                let score_b = self.scoring_chance(ctx, b);
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, player)| player.id)
    }

    fn is_open_for_pass(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        let max_distance = 20.0; // Adjust based on your game's scale

        // Check if the teammate is within a reasonable distance
        if ctx.tick_context.object_positions.player_distances.get(ctx.player.id, teammate.id).unwrap() > max_distance {
            return false;
        }

        let players = ctx.player();
        let opponents = players.opponents();

        // Check if there are no opponents close to the teammate
        opponents
            .iter()
            .all(|opponent| opponent.position.distance_to(&teammate.position) > 5.0)
    }

    fn in_passing_lane(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let player_to_ball = (ball_position - ctx.player.position).normalize();
        let player_to_teammate = (teammate.position - ctx.player.position).normalize();

        // Check if the teammate is in the passing lane
        player_to_ball.dot(&player_to_teammate) > 0.8
    }

    fn scoring_chance(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> f32 {
        let goal_position = match teammate.side {
            Some(PlayerSide::Left) => ctx.context.goal_positions.right,
            Some(PlayerSide::Right) => ctx.context.goal_positions.left,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        let distance_to_goal = teammate.position.distance_to(&goal_position);

        // Calculate the scoring chance based on distance to the goal
        1.0 - distance_to_goal / ctx.context.field_size.width as f32
    }

    fn can_shoot(&self, ctx: &StateProcessingContext) -> bool {
        let shot_distance = 25.0; // Adjust based on your game's scale

        let distance_to_goal = ctx.ball().distance_to_opponent_goal();

        // Check if the player is within shooting distance and has a clear shot
        distance_to_goal < shot_distance && self.has_clear_shot(ctx)
    }

    fn has_clear_shot(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.player();
        let opponents = players.opponents();

        let opponent_goal_position = match ctx.player.side {
            // swap for opponents
            Some(PlayerSide::Left) => ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        // Check if there are no opponents blocking the shot
        opponents.iter().all(|opponent| {
            let opponent_to_goal = (opponent_goal_position - opponent.position).normalize();
            let player_to_goal = (opponent_goal_position - ctx.player.position).normalize();
            opponent_to_goal.dot(&player_to_goal) < 0.9
        })
    }
}
