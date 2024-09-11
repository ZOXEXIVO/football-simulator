use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::strategies::processor::StateChangeResult;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerDistanceFromStartPosition, PlayerTickContext,
    StateProcessingContext, StateProcessingHandler,
};

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

#[derive(Default)]
pub struct GoalkeeperStandingState {}

impl StateProcessingHandler for GoalkeeperStandingState {
    fn try_fast(&self, context: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl GoalkeeperStandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if in_state_time > 100 {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(
                GoalkeeperState::ComingOut,
            ));
        }

        if !player_context.ball.on_own_side {
            return StateChangeResult::none();
        }

        if player_context.ball.is_heading_towards_player
            && player_context.ball.ball_distance < 100.0
        {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(
                GoalkeeperState::PreparingForSave,
            ));
        }

        if player.player_attributes.condition_percentage() < 60 {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(
                GoalkeeperState::Resting,
            ));
        }

        match player_context.player.distance_to_start_position {
            PlayerDistanceFromStartPosition::Medium | PlayerDistanceFromStartPosition::Big => {
                return StateChangeResult::with_state(PlayerState::Goalkeeper(
                    GoalkeeperState::ReturningToGoal,
                ));
            }
            _ => {}
        }

        StateChangeResult::none()
    }
}
