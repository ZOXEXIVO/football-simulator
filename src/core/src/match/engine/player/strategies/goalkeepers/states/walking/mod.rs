use crate::common::NeuralNetwork;

use crate::r#match::{
    BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    StateChangeResult, SteeringBehavior,
};

use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_WALKING_STATE_NETWORK: NeuralNetwork = PlayerWalkingStateNetLoader::load();
}

pub struct GoalkeeperWalkingState {}

impl GoalkeeperWalkingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: BallMetadata,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if ball_metadata.ball_is_on_player_home_side {
            return StateChangeResult::with_state(PlayerState::Returning);
        }

        if in_state_time % 10 == 0 {
            let wander_velocity = SteeringBehavior::Wander {
                target: objects_positions.ball_position, // TODO random point
                radius: 10.0,
                jitter: 0.0,
                distance: 5.0,
                angle: 40.0,
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

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_walking_data.json");

#[derive(Debug)]
pub struct PlayerWalkingStateNetLoader;

impl PlayerWalkingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
