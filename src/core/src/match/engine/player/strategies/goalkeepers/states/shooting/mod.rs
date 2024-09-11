use crate::common::NeuralNetwork;
use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, StateChangeResult,
    StateProcessingContext, StateProcessingHandler,
};

static GOALKEEPER_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_shooting_data.json")));

#[derive(Default)]
pub struct GoalkeeperShootingState {}

impl StateProcessingHandler for GoalkeeperShootingState {
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

impl GoalkeeperShootingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()

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
