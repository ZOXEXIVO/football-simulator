use std::sync::LazyLock;
use crate::common::NeuralNetwork;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer,
    PlayerTickContext,  StateChangeResult,
};
use crate::r#match::player::events::PlayerUpdateEvent;

static DEFENDER_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_shooting_data.json"))
});

pub struct DefenderShootingState {}

impl DefenderShootingState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()
        //
        // let mut res_vec = Vec::new();
        //
        // res_vec.push(objects_positions.ball_position.x as f64);
        // res_vec.push(objects_positions.ball_position.y as f64);
        //
        // res_vec.push(objects_positions.ball_velocity.x as f64);
        // res_vec.push(objects_positions.ball_velocity.y as f64);
        //
        // let res = PLAYER_SHOOTING_STATE_NETWORK.run(&res_vec);
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
    }
}
