use crate::common::NeuralNetwork;
use crate::r#match::position::VectorExtensions;
use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    BallContext, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState,
    PlayerTickContext, PlayerUpdateEvent, StateChangeResult, SteeringBehavior,
};

lazy_static! {
    static ref GOALKEEPER_RUNNING_STATE_NETWORK: NeuralNetwork =
        DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json"));
}

pub struct GoalkeeperRunningState {}

impl GoalkeeperRunningState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        Self::check_collision(player, &tick_context.objects_positions, result);

        let to_ball_velocity = SteeringBehavior::Seek {
            target: tick_context.objects_positions.ball_position,
        }
        .calculate(player)
        .velocity;

        StateChangeResult::with_velocity(to_ball_velocity)
    }

    fn check_collision(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        if objects_positions
            .ball_position
            .distance_to(&player.position)
            < 5.0
        {
            result.push(PlayerUpdateEvent::TacklingBall(player.player_id))
        }
    }
}
