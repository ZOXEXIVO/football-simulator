use std::sync::LazyLock;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::position::VectorExtensions;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    PlayerTickContext, StateChangeResult, SteeringBehavior,
};
use nalgebra::Vector3;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

pub struct GoalkeeperRunningState {}

impl GoalkeeperRunningState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player.position.distance_to(&player.start_position) < 10.0 {
            return StateChangeResult::with(PlayerState::Standing, Vector3::zeros());
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
