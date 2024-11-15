use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::events::{Event, EventCollection};
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use rand::prelude::IteratorRandom;
use std::sync::LazyLock;

static DEFENDER_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

#[derive(Default)]
pub struct DefenderPassingState {}

impl StateProcessingHandler for DefenderPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.player.has_ball(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Standing,
            ));
        }

        if let Some(teammate_id) = self.find_best_pass_option(ctx) {
            let teammate_player_position = ctx.tick_context.positions.players.position(teammate_id);

            let pass_power = self.calculate_pass_power(teammate_id, ctx);

            return Some(StateChangeResult::with_defender_state_and_event(
                DefenderState::Returning,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    ctx.player.id,
                    teammate_player_position,
                    pass_power,
                )),
            ));
        }
        let mut best_player_id = None;
        let mut highest_score = 0.0;

        for (player_id, teammate_distance) in ctx.players().teammates().nearby_ids(30.0) {
            let score = 1.0 / (teammate_distance + 1.0);
            if score > highest_score {
                highest_score = score;
                best_player_id = Some(player_id);
            }
        }

        if let Some(teammate_id) = best_player_id {
             let events = EventCollection::with_event(Event::PlayerEvent(PlayerEvent::PassTo(
                ctx.player.id,
                ctx.tick_context.positions.players.position(teammate_id),
                1.0,
            )));

            return Some(StateChangeResult::with_events(events));
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

impl DefenderPassingState {
    fn find_best_pass_option<'a>(&'a self, ctx: &'a StateProcessingContext<'a>) -> Option<u32> {
        if let Some((teammate_id, _)) = ctx
            .players()
            .teammates()
            .nearby_ids(250.0)
            .choose(&mut rand::thread_rng())
        {
            return Some(teammate_id);
        }

        None
    }

    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> f64 {
        let distance = ctx
            .tick_context
            .distances
            .get(ctx.player.id, teammate_id);

        let pass_skill = ctx.player.skills.technical.passing;

        (distance / pass_skill as f32 * 10.0) as f64
    }
}
