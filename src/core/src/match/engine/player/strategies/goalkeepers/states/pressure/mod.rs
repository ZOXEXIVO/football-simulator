use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_PRESSURE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pressure_data.json")));

#[derive(Default)]
pub struct GoalkeeperPressureState {}

impl StateProcessingHandler for GoalkeeperPressureState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl GoalkeeperPressureState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player.position.distance_to(&player.start_position) < 10.0 {
            return StateChangeResult::with(
                PlayerState::Goalkeeper(GoalkeeperState::Standing)
            );
        }

        Self::check_collision(player, &tick_context.objects_positions, result);

        let to_start_position = SteeringBehavior::Seek {
            target: player.start_position,
        }
        .calculate(player)
        .velocity;

        StateChangeResult::with_velocity(to_start_position)
    }

    fn check_collision(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        if objects_positions
            .ball_position
            .distance_to(&player.position)
            < 10.0
        {
            result.push(PlayerUpdateEvent::TacklingBall(player.player_id))
        }
    }
}
