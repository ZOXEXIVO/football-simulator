use std::sync::LazyLock;
use nalgebra::Vector3;
use rand::prelude::SliceRandom;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::events::{Event, EventCollection};
use crate::r#match::player::events::PlayerEvent;

static DEFENDER_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

#[derive(Default)]
pub struct DefenderPassingState {}

impl StateProcessingHandler for DefenderPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        if let Some(teammate) = self.find_best_pass_option(ctx) {
            if let Some(teammate_player_position) = ctx.tick_context.object_positions.players_positions.get_player_position(teammate.id) {
                let pass_power = self.calculate_pass_power(teammate.id, ctx);

                return Some(StateChangeResult::with_defender_state_and_event(DefenderState::Returning, Event::PlayerEvent(
                    PlayerEvent::PassTo(ctx.player.id, teammate_player_position, pass_power)
                )));
            }
        }

        let (nearest_teammates, opponents) = ctx.tick_context
            .object_positions
            .player_distances
            .players_within_distance(ctx.player, 30.0);

        if ctx.player.has_ball && opponents.len() > 1  {
            return Some(StateChangeResult::with_defender_state(DefenderState::Clearing));
        }

        let mut best_player_id = None;
        let mut highest_score = 0.0;

        for (player_id, teammate_distance) in nearest_teammates {
            let score = 1.0 / (teammate_distance + 1.0);
            if score > highest_score {
                highest_score = score;
                best_player_id = Some(player_id);
            }
        }

        if let Some(player_id) = best_player_id {
            if let Some(teammate_player_position) = ctx.tick_context.object_positions.players_positions.get_player_position(player_id) {
                let events = EventCollection::with_event(Event::PlayerEvent(
                    PlayerEvent::PassTo(ctx.player.id, teammate_player_position, 1.0)
                ));

                return Some(StateChangeResult::with_events(events));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl DefenderPassingState {
    fn find_best_pass_option<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Option<&'a MatchPlayer> {
        let teammates = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_teammates(ctx.player);

        if let Some(teammates_result) = teammates {
            if let Some((teammate_id, _)) = teammates_result.choose(&mut rand::thread_rng()) {
                return Some(ctx.context.players.get(*teammate_id)?);
            }
        }

        None
    }

    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> f64 {
        let distance = ctx.tick_context.object_positions.
            player_distances.get(ctx.player.id, teammate_id)
            .unwrap();

        let pass_skill = ctx.player.skills.technical.passing;

        (distance / pass_skill as f32 * 10.0) as f64
    }
}