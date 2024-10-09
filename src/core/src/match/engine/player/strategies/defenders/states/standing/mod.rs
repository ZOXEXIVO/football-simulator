use std::sync::LazyLock;

use nalgebra::Vector3;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::IntegerUtils;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{ConditionContext, MatchPlayer, PlayerDistanceFromStartPosition, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior, VectorExtensions};

static DEFENDER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

const INTERCEPTION_DISTANCE: f32 = 100.0;
const CLEARING_DISTANCE: f32 = 50.0;
const TIRED_THRESHOLD: f32 = 30.0;
const STANDING_TIME_LIMIT: u64 = 300;
const WALK_DISTANCE_THRESHOLD: f32 = 15.0;
const MARKING_DISTANCE: f32 = 15.0;
const PRESSING_DISTANCE: f32 = 150.0;

#[derive(Default)]
pub struct DefenderStandingState {}

impl StateProcessingHandler for DefenderStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let ball_ops = ctx.ball();
        let player_ops = ctx.player();

        if ball_ops.on_own_side() {
            // Ball is on the defender's side
            if ball_ops.is_towards_player() {
                if ball_ops.distance() < INTERCEPTION_DISTANCE {
                    // Move to intercept only if ball is moving slowly or player is close
                    if ball_ops.speed() < 20.0 || player_ops.distance_from_start_position() < 10.0 {
                        return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
                    }
                }

                // Consider teammates and opponents more carefully before switching to marking or clearing
                let (teammates_count, opponents_count) = player_ops.distances();
                if opponents_count > teammates_count && ctx.player.has_ball && ball_ops.on_own_third() {
                    // Only clear if outnumbered, has ball, and in defensive third
                    return Some(StateChangeResult::with_defender_state(DefenderState::Clearing));
                } else if opponents_count > 1 && ball_ops.distance() < MARKING_DISTANCE {
                    // Mark if multiple opponents nearby and ball is close
                    return Some(StateChangeResult::with_defender_state(DefenderState::Marking));
                }

                // Track back if far from position and ball moving fast
                if player_ops.distance_from_start_position() > 20.0 && ball_ops.speed() > 20.0 {
                    return Some(StateChangeResult::with_defender_state(DefenderState::TrackingBack));
                }
            } else {
                // Ball is not towards the player
                if let Some(opponent) = self.find_nearby_opponent(ctx) {
                    if opponent.has_ball && opponent.position.distance_to(&ctx.player.position) < PRESSING_DISTANCE {
                        // Only press if opponent has ball and is very close
                        return Some(StateChangeResult::with_defender_state(DefenderState::Pressing));
                    } else if opponent.position.distance_to(&ctx.player.position) < MARKING_DISTANCE {
                        // Mark nearby opponents
                        return Some(StateChangeResult::with_defender_state(DefenderState::Marking));
                    }
                }
            }
        }

        // On attacking side
        else {
            // Walk or hold line more readily on attacking side
            if self.should_transition_to_walking(ctx) {
                return Some(StateChangeResult::with_defender_state(DefenderState::Walking));
            }

            if self.should_hold_line(ctx) {
                return Some(StateChangeResult::with_defender_state(DefenderState::HoldingLine));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        None
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl DefenderStandingState {
    fn prepare_network_input(&self, ctx: &StateProcessingContext) -> Vec<f64> {
        vec![
            ctx.ball().distance() as f64,
            ctx.ball().speed() as f64,
            if ctx.ball().on_own_side() { 1.0 } else { 0.0 },
            if ctx.ball().is_towards_player() { 1.0 } else { 0.0 },
            ctx.player().distance_from_start_position() as f64,
            ctx.player.skills.physical.stamina as f64,
            ctx.player.skills.mental.positioning as f64,
            ctx.player.skills.mental.decisions as f64,
            ctx.context.time.time as f64,
            ctx.in_state_time as f64,
            ctx.player.skills.physical.acceleration as f64,
            ctx.player.skills.technical.tackling as f64,
            ctx.player.skills.technical.marking as f64
        ]
    }

    fn should_transition_to_walking(&self, ctx: &StateProcessingContext) -> bool {
        let player_ops = ctx.player();
        let ball_ops = ctx.ball();

        // Check if player is tired
        let is_tired = player_ops.is_tired();

        // Check if player has been standing for too long
        let standing_too_long = ctx.in_state_time > STANDING_TIME_LIMIT;

        // Check if the ball is far away
        let ball_far_away = ball_ops.distance() > INTERCEPTION_DISTANCE * 2.0;

        // Check if there's no immediate threat
        let no_immediate_threat = self.find_nearby_opponent(ctx)
            .map_or(true, |opponent| opponent.position.distance_to(&ctx.player.position) > CLEARING_DISTANCE);

        // Check if player is close to their optimal position
        let close_to_optimal_position = player_ops.distance_from_start_position() < WALK_DISTANCE_THRESHOLD;

        // Check team's tactical situation
        let team_in_control = self.is_team_in_control(ctx);

        // Combine all factors to make the decision
        (is_tired || standing_too_long) &&
            (ball_far_away || close_to_optimal_position) &&
            no_immediate_threat &&
            team_in_control
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

    fn should_hold_line(&self, ctx: &StateProcessingContext) -> bool {
        // Implement logic to decide if the defender should hold the defensive line
        // This could be based on team tactics, other defenders' positions, etc.
        let player_ops = ctx.player();
        let ball_ops = ctx.ball();

        let defenders = player_ops.defenders();
        let avg_defender_x = defenders.iter().map(|d| d.position.x).sum::<f32>() / defenders.len() as f32;

        (ctx.player.position.x - avg_defender_x).abs() < 5.0 && ball_ops.distance() > INTERCEPTION_DISTANCE
    }

    fn is_team_in_control(&self, ctx: &StateProcessingContext) -> bool {
        // Implement logic to determine if the defender's team is in control
        // This could be based on ball possession, field position, etc.
        let teammates_with_ball = ctx.context.players.get_by_team(ctx.player.team_id);
        !teammates_with_ball.is_empty()
    }
}