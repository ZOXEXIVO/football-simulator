use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::events::Event;
use crate::r#match::player::events::PlayerEvent;

static GOALKEEPER_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

#[derive(Default)]
pub struct GoalkeeperPassingState {}

impl StateProcessingHandler for GoalkeeperPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        // 1. Check if the goalkeeper has the ball
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Standing,
            ));
        }

        // 2. Find the best teammate to pass to
        let (nearest_teammates, opponents) = ctx
            .tick_context
            .object_positions
            .player_distances
            .players_within_distance(ctx.player, 30.0);

        if let Some((teammate_id, teammate_distance)) = nearest_teammates.first() {
            let pass_skill = ctx.player.skills.technical.passing;

            let pass_power = (teammate_distance / pass_skill as f32 * 10.0) as f64;

            result
                .events
                .add_player_event(PlayerEvent::UnClaimBall(ctx.player.id));

            if let Some(teammate_positions) = ctx
                .tick_context
                .object_positions
                .players_positions
                .get_player_position(*teammate_id)
            {
                result.events.add_player_event(PlayerEvent::PassTo(
                    *teammate_id,
                    teammate_positions,
                    pass_power,
                ));
            }
            return Some(result);
        }

        if ctx.in_state_time > 50 {
            let (nearest_teammates, opponents) = ctx.tick_context
                .object_positions
                .player_distances
                .players_within_distance(ctx.player, 300.0);

            let (teammate_id, distance) = nearest_teammates.first().unwrap();

            let teammaste_position = ctx.tick_context.object_positions.players_positions.get_player_position(*teammate_id).unwrap();

            let pass_power = (distance / ctx.player.skills.technical.passing as f32 * 10.0) as f64;

            return Some(StateChangeResult::with_goalkeeper_state_and_event(
                GoalkeeperState::Standing,
                Event::PlayerEvent(PlayerEvent::PassTo(*teammate_id, teammaste_position, pass_power))));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}
