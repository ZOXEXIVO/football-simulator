use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    ConditionContext, MatchPlayer, PlayerSide, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use rand::prelude::IteratorRandom;

static FORWARD_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

#[derive(Default)]
pub struct ForwardPassingState {}

impl StateProcessingHandler for ForwardPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the player has the ball
        if !ctx.player.has_ball {
            // Transition to Running state if the player doesn't have the ball
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if the player is under pressure
        // if player_ops.is_under_pressure() {
        //     // Transition to Dribbling state if under pressure
        //     return Some(StateChangeResult::with_forward_state(
        //         ForwardState::Dribbling,
        //     ));
        // }

        // Find the best passing option
        if let Some(teammate) = self.find_best_pass_option(ctx) {
            if let Some(teammate_player_position) = ctx
                .tick_context
                .object_positions
                .players_positions
                .get_player_position(teammate.id)
            {
                let pass_power = self.calculate_pass_power(teammate.id, ctx);

                return Some(StateChangeResult::with_forward_state_and_event(
                    ForwardState::Running,
                    Event::PlayerEvent(PlayerEvent::PassTo(
                        teammate.id,
                        teammate_player_position,
                        pass_power,
                    )),
                ));
            }
        }

        // Check if there's space to dribble forward
        if self.space_to_dribble(ctx) {
            // Transition to Dribbling state if there's space to dribble
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Dribbling,
            ));
        }

        // Check if there's an opportunity to shoot
        if self.can_shoot(ctx) {
            // Transition to Shooting state if there's an opportunity to shoot
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Shooting,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl ForwardPassingState {
    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> f64 {
        let distance = ctx
            .tick_context
            .object_positions
            .player_distances
            .get(ctx.player.id, teammate_id)
            .unwrap();

        let pass_skill = ctx.player.skills.technical.passing;

        (distance / pass_skill as f32 * 10.0) as f64
    }

    fn find_best_pass_option<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Option<&'a MatchPlayer> {
        let players = ctx.players();

        if let Some((teammate_id, _)) = players.teammates().nearby_raw(100.0).choose(&mut rand::thread_rng()) {
            return Some(ctx.context.players.get(teammate_id)?);
        }

        None
    }

    fn is_open_for_pass(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        let max_distance = 20.0; // Adjust based on your game's scale
        let players = ctx.players();
        let opponents = players.opponents();

        // Check if the teammate is within a reasonable distance
        if ctx.player.position.distance_to(&teammate.position) > max_distance {
            return false;
        }

        // Check if there are no opponents close to the teammate
        opponents
            .all()
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
        let angle_to_goal = self.angle_to_goal(ctx, teammate);

        // Calculate the scoring chance based on distance and angle to the goal
        (1.0 - distance_to_goal / ctx.context.field_size.width as f32) * angle_to_goal
    }

    fn angle_to_goal(&self, ctx: &StateProcessingContext, player: &MatchPlayer) -> f32 {
        let goal_position = match player.side {
            Some(PlayerSide::Left) => ctx.context.goal_positions.right,
            Some(PlayerSide::Right) => ctx.context.goal_positions.left,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        let player_to_goal = (goal_position - player.position).normalize();
        let player_velocity = player.velocity.normalize();

        player_velocity.dot(&player_to_goal).acos()
    }

    fn space_to_dribble(&self, ctx: &StateProcessingContext) -> bool {
        let dribble_distance = 10.0; // Adjust based on your game's scale
        let players = ctx.players();
        let opponents = players.opponents();

        // Check if there are no opponents within the dribble distance
        opponents
            .all()
            .iter()
            .all(|opponent| ctx.player.position.distance_to(&opponent.position) > dribble_distance)
    }

    fn can_shoot(&self, ctx: &StateProcessingContext) -> bool {
        let shot_distance = 25.0; // Adjust based on your game's scale

        // Check if the player is within shooting distance and has a clear shot
        ctx.ball().distance_to_opponent_goal() < shot_distance && self.has_clear_shot(ctx)
    }

    fn has_clear_shot(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.players();
        let opponents = players.opponents();

        let opponent_goal_position = match ctx.player.side {
            // swap for opponents
            Some(PlayerSide::Left) => ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        // Check if there are no opponents blocking the shot
        opponents.all().iter().all(|opponent| {
            let opponent_to_goal = (opponent_goal_position - opponent.position).normalize();
            let player_to_goal = (opponent_goal_position - ctx.player.position).normalize();
            opponent_to_goal.dot(&player_to_goal) < 0.9
        })
    }
}
