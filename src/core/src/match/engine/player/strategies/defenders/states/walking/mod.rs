use crate::common::NeuralNetwork;
use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, PlayerUpdateEvent,
    StateChangeResult,
};

lazy_static! {
    static ref DEFENDER_WALKING_STATE_NETWORK: NeuralNetwork =
        DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json"));
}

pub struct DefenderWalkingState {}

impl DefenderWalkingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()
        // if context.time.time % 1000 == 0 {
        //     let direction = SteeringBehavior::Seek {
        //         target: objects_positions.ball_position,
        //     }
        //     .calculate(player);
        //
        //     player.velocity = Vector3::new(direction.velocity.x, direction.velocity.y, 0.0);
        //
        //     if in_state_time > 30 {
        //         player.state = PlayerState::Running;
        //     }
        //
        //     // if player.skills.physical.acceleration > 15.0 {
        //     //     player.state = PlayerState::Running;
        //     // }
        // }
        //
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
