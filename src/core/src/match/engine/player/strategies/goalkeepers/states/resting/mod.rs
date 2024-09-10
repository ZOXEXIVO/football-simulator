use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext};
use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::CONDITION_MAX_VALUE;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_RESTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_resting_data.json")));

#[derive(Default)]
pub struct GoalkeeperRestingState {}

impl StateProcessingHandler for GoalkeeperRestingState {
    fn try_fast(
        &self, context: &mut StateProcessingContext
    ) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(
        &self, context: &mut StateProcessingContext
    ) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl GoalkeeperRestingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if in_state_time < 5 {
            StateChangeResult::none();
        }

        let player_skills = player.skills;
        let player_attributes = player.player_attributes;

        let max_rest_duration = Self::calculate_max_rest_duration(
            player_skills.physical.stamina,
            player_skills.physical.natural_fitness,
            player_skills.physical.match_readiness,
            player_attributes.condition,
        );

        if in_state_time >= max_rest_duration {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::Standing));
        }

        StateChangeResult::none()
    }

    fn calculate_max_rest_duration(
        stamina: f32,
        natural_fitness: f32,
        match_readiness: f32,
        condition: i16,
    ) -> u64 {
        let base_duration = 1000.0 + (stamina * 100.0) + (natural_fitness * 50.0);

        let readiness_factor = (match_readiness / 100.0) * 0.5;
        let condition_factor = (condition as f32 / CONDITION_MAX_VALUE as f32) * 0.5;

        (base_duration * readiness_factor * condition_factor) as u64
    }
}
