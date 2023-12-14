use crate::common::NeuralNetwork;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

use crate::r#match::position::VectorExtensions;
use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_SHOOTING_STATE_NETWORK: NeuralNetwork = PlayerShootingStateNetLoader::load();
}

pub struct ShootingState {}

impl ShootingState {
    pub fn process(
        _in_state_time: u64,
        _player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        let mut res_vec = Vec::new();

        res_vec.push(objects_positions.ball_positions.x as f64);
        res_vec.push(objects_positions.ball_positions.y as f64);

        res_vec.push(objects_positions.ball_velocity.x as f64);
        res_vec.push(objects_positions.ball_velocity.y as f64);

        let res = PLAYER_SHOOTING_STATE_NETWORK.run(&res_vec);

        if res[0] > 0.6 {
            return Some(PlayerState::Standing);
        }
        if res[1] > 0.6 {
            return Some(PlayerState::Walking);
        }
        if res[2] > 0.6 {
            return Some(PlayerState::Running);
        }
        if res[3] > 0.6 {
            return Some(PlayerState::Tackling);
        }
        if res[4] > 0.6 {
            return Some(PlayerState::Shooting);
        }
        if res[5] > 0.6 {
            return Some(PlayerState::Passing);
        }


        // write code for processing shoot state

        //         player.velocity = player.skills.running_speed();
        //         // let distance_to_goal = (self.position.x - self.field.width as i16 / 2).abs();
        //         // if distance_to_goal < 50 {
        //         //     let mut rng = thread_rng();
        //     let shot_success = rng.gen_range(0, 100);
        //
        //     let shooting_skill = self.skills.technical.finishing;
        //
        //     if shot_success < shooting_skill {
        //         if self.position.x < self.field.width as i16 / 2 {
        //             self.field.home_goals += 1;
        //         } else {
        //             self.field.away_goals += 1;
        //         }
        //     }
        // }

        None
    }
}


const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_shooting_data.json");

#[derive(Debug)]
pub struct PlayerShootingStateNetLoader;

impl PlayerShootingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}