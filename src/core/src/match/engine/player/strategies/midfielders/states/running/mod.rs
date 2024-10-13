use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;

static MIDFIELDER_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct MidfielderRunningState {}

impl StateProcessingHandler for MidfielderRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball {
            // If the player has the ball, consider shooting or passing
            if self.is_in_shooting_position(ctx) {
                return Some(StateChangeResult::with_midfielder_state(MidfielderState::DistanceShooting));
            }

            if let Some(teammate_id) = self.find_open_teammate(ctx) {
                return Some(StateChangeResult::with_midfielder_state_and_event(
                    MidfielderState::ShortPassing,
                    Event::PlayerEvent(PlayerEvent::RequestPass(teammate_id)),
                ));
            }

            // If no shooting or passing options, continue running with the ball
            return None;
        }

        // If the player doesn't have the ball, check if they can gain possession
        if self.can_gain_possession(ctx) {
            return Some(StateChangeResult::with_midfielder_state_and_event(
                MidfielderState::Tackling,
                Event::PlayerEvent(PlayerEvent::TacklingBall(ctx.player.id)),
            ));
        }

        // If the player can't gain possession, check if they should press or return
        if self.should_press(ctx) {
            return Some(StateChangeResult::with_midfielder_state(MidfielderState::Pressing));
        }

        if self.should_return_to_position(ctx) {
            return Some(StateChangeResult::with_midfielder_state(MidfielderState::Returning));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.team().is_control_ball() {
            Some(SteeringBehavior::Arrive {
                target: ctx.ball().direction_to_opponent_goal(),
                slowing_distance: 200.0
            }.calculate(ctx.player).velocity)
        }else {
            Some(SteeringBehavior::Arrive {
                target: ctx.ball().direction_to_own_goal(),
                slowing_distance: 200.0
            }.calculate(ctx.player).velocity)
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl MidfielderRunningState {
    fn is_in_shooting_position(&self, ctx: &StateProcessingContext) -> bool {
        let shooting_range = 25.0; // Distance from goal to consider shooting
        let player_position = ctx.player.position;
        let goal_position = ctx.ball().direction_to_opponent_goal();

        let distance_to_goal = (player_position - goal_position).magnitude();

        distance_to_goal <= shooting_range
    }

    fn find_open_teammate<'a>(&self, ctx: &StateProcessingContext<'a>) -> Option<u32> {
        // Find an open teammate to pass to
        let teammates = ctx.context.players.get_by_team(ctx.player.team_id);
        let open_teammates= teammates
            .iter()
            .filter(|teammate| {
                // Check if the teammate is open (not closely marked by an opponent)
                let opponent_distance = ctx
                    .tick_context
                    .object_positions
                    .player_distances
                    .find_closest_opponent(teammate)
                    .map(|(_, distance)| distance)
                    .unwrap_or(f32::MAX);

                opponent_distance > 5.0 // Adjust the threshold as needed
            })
            .min_by(|a, b| {
                // Prefer teammates closer to the opponent's goal
                let a_distance = (a.position - ctx.ball().direction_to_opponent_goal()).magnitude();
                let b_distance = (b.position - ctx.ball().direction_to_opponent_goal()).magnitude();
                a_distance.partial_cmp(&b_distance).unwrap()
            }).map(|p| p.id);

        open_teammates
    }

    fn should_press(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player should press the opponent with the ball
        let ball_distance = ctx.ball().distance();
        let pressing_distance = 20.0; // Adjust the threshold as needed

        !ctx.team().is_control_ball() && ball_distance < pressing_distance
    }

    fn should_return_to_position(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player should return to their starting position
        let distance_from_start = ctx.player().distance_from_start_position();
        let return_distance = 30.0; // Adjust the threshold as needed

        distance_from_start > return_distance
    }

    fn can_gain_possession(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player is close enough to the ball to attempt to gain possession
        let ball_distance = ctx.ball().distance();
        let tackling_distance = 2.0; // Adjust the threshold as needed

        !ctx.player.has_ball && ball_distance < tackling_distance
    }
}
