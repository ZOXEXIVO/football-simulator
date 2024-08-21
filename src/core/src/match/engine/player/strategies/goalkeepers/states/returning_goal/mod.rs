use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerDistanceFromStartPosition, PlayerTickContext, StateChangeResult, SteeringBehavior};
use std::sync::LazyLock;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_RETURNING_GOAL_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_returning_goal_data.json")));

pub struct GoalkeeperReturningGoalState {}

impl GoalkeeperReturningGoalState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player_context.player_context.distance_to_start_position == PlayerDistanceFromStartPosition::Small{
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::UnderPressure));
        }

        if in_state_time % 2 == 0 {
            let wander_behaviour = SteeringBehavior::Arrive {
                target: player.start_position,
                slowing_distance: 5.0,
            }.calculate(player);

            return StateChangeResult::with_velocity(wander_behaviour.velocity);
        }

        StateChangeResult::none()
    }
}
