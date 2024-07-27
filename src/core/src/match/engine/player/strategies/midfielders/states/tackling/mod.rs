use crate::common::NeuralNetwork;
use std::sync::LazyLock;

use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult,
};

static MIDFIELDER_TACKLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_tackling_data.json")));

pub struct MidfielderTacklingState {}

impl MidfielderTacklingState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()
        // let mut res_vec = Vec::new();
        //
        // res_vec.push(objects_positions.ball_positions.x as f64);
        // res_vec.push(objects_positions.ball_positions.y as f64);
        //
        // res_vec.push(objects_positions.ball_velocity.x as f64);
        // res_vec.push(objects_positions.ball_velocity.y as f64);
        //
        // let res = PLAYER_TACKLING_STATE_NETWORK.run(&res_vec);
        //
        // let index_of_max_element = res
        //     .iter()
        //     .enumerate()
        //     .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        //     .unwrap()
        //     .0;
        //
        // //println!("RES = {:?}", res);
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

        //Check for transition to standing or walking state
        // let tackling_success = player.skills.tackling() * player.player_attributes.condition;
        // if tackling_success > 50.0 {
        //     player.has_ball = true;
        // }
        // // Check for transition to standing state
        // if player.player_attributes.condition < 20 {
        //     return Some(PlayerState::Standing);
        // }
    }
}
