use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use rand::prelude::{IteratorRandom};
use std::sync::LazyLock;

static MIDFIELDER_DISTRIBUTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_distributing_data.json")));

#[derive(Default)]
pub struct MidfielderDistributingState {}

impl StateProcessingHandler for MidfielderDistributingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Find the best passing option
        if let Some(teammate) = self.find_best_pass_option(ctx) {
            if let Some(teammate_player_position) = ctx
                .tick_context
                .object_positions
                .players_positions
                .get_player_position(teammate.id)
            {
                let pass_power = self.calculate_pass_power(teammate.id, ctx);

                return Some(StateChangeResult::with_midfielder_state_and_event(
                    MidfielderState::Returning,
                    Event::PlayerEvent(PlayerEvent::PassTo(
                        teammate.id,
                        teammate_player_position,
                        pass_power,
                    )),
                ));
            }
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

impl MidfielderDistributingState {
    fn find_best_pass_option<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Option<&'a MatchPlayer> {
        let players = ctx.players();

        if let Some((teammate_id, _)) = players.teammates().nearby_raw(300.0).choose(&mut rand::thread_rng()) {
            return Some(ctx.context.players.get(teammate_id)?);
        }

        None
    }

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
}
