use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult, SteeringBehavior};
use std::sync::LazyLock;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_COMINGOUT_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_comingout_data.json")));

pub struct GoalkeeperComingOutState {}

impl GoalkeeperComingOutState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player_context.ball_context.on_own_side {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::ReturningToGoal));
        }

        if in_state_time % 2 == 0 {
            let wander_behaviour = SteeringBehavior::Wander {
                target: tick_context.objects_positions.ball_position,
                radius: 100.0,
                jitter: 1.0,
                distance: 10.0,
                angle: 1.0,
            }.calculate(player);

            return StateChangeResult::with_velocity(wander_behaviour.velocity);
        }

        StateChangeResult::none()
    }
}
