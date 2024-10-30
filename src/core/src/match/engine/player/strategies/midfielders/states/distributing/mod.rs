use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, MatchPlayerLite, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use rand::prelude::IteratorRandom;
use std::sync::LazyLock;

static MIDFIELDER_DISTRIBUTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_distributing_data.json")));

#[derive(Default)]
pub struct MidfielderDistributingState {}

impl StateProcessingHandler for MidfielderDistributingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Find the best passing option
        if let Some(teammate) = self.find_best_pass_option(ctx) {
            let teammate_player_position = ctx.tick_context.player_position(teammate.id);

            let pass_power = self.calculate_pass_power(teammate.id, ctx);

            return Some(StateChangeResult::with_midfielder_state_and_event(
                MidfielderState::Returning,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    ctx.player.id,
                    teammate_player_position,
                    pass_power,
                )),
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

impl MidfielderDistributingState {
    fn find_best_pass_option<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Option<MatchPlayerLite> {
        let players = ctx.players();

        if let Some(player) = players
            .teammates()
            .nearby(300.0)
            .choose(&mut rand::thread_rng())
        {
            return Some(player);
        }

        None
    }

    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> f64 {
        let distance = ctx.player().distance_to_player(teammate_id);

        let pass_skill = ctx.player.skills.technical.passing;

        (distance / pass_skill as f32 * 10.0) as f64
    }
}
