use std::sync::LazyLock;
use crate::common::NeuralNetwork;

use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerState, PlayerTickContext,
    StateChangeResult, SteeringBehavior,
};
use crate::r#match::player::events::PlayerUpdateEvent;

static GOALKEEPER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

pub struct GoalkeeperWalkingState {}

impl GoalkeeperWalkingState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player_tick_context.ball_context.on_own_side {
            return StateChangeResult::with(PlayerState::Returning, player.skills.running_speed());
        }

        if in_state_time > 100 {
            let wander_velocity = SteeringBehavior::Wander {
                target: player.start_position,
                radius: 40.0,
                jitter: 0.0,
                distance: 25.0,
                angle: 10.0,
            }
            .calculate(player)
            .velocity;

            return StateChangeResult::with_velocity(wander_velocity);
        }

        StateChangeResult::none()

        // if context.time.time % 1000 == 0 {
        //     let direction = SteeringBehavior::Seek {
        //         target: objects_positions.ball_position,
        //     }
        //     .calculate(player);
        //
        //     StateChangeResult::with_velocity(Vector3::new(
        //         direction.velocity.x,
        //         direction.velocity.y,
        //         0.0,
        //     ))
        //
        //     // if player.skills.physical.acceleration > 15.0 {
        //     //     player.state = PlayerState::Running;
        //     // }
        // }

        // let mut res_vec = Vec::new();
        //
        // res_vec.push(objects_positions.ball_position.x as f64);
        // res_vec.push(objects_positions.ball_position.y as f64);
        //
        // res_vec.push(objects_positions.ball_velocity.x as f64);
        // res_vec.push(objects_positions.ball_velocity.y as f64);
        //
        // let res = PLAYER_WALKING_STATE_NETWORK.run(&res_vec);
        //
        // let index_of_max_element = res
        //     .iter()
        //     .enumerate()
        //     .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        //     .unwrap()
        //     .0;
        //
        // match index_of_max_element {
        //     0 => Some(PlayerState::Standing),
        //     1 => Some(PlayerState::Walking),
        //     2 => Some(PlayerState::Running),
        //     3 => Some(PlayerState::Tackling),
        //     4 => Some(PlayerState::Shooting),
        //     5 => Some(PlayerState::Passing),
        //     6 => Some(PlayerState::Returning),
        //     _ => None,
        // }
    }
}
