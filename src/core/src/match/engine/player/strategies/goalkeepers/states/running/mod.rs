use crate::common::NeuralNetwork;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    StateChangeResult,
};

lazy_static! {
    static ref PLAYER_RUNNING_STATE_NETWORK: NeuralNetwork = PlayerRunningStateNetLoader::load();
}

pub struct GoalkeeperRunningState {}

impl GoalkeeperRunningState {
    pub fn process(
        _in_state_time: u64,
        ball_metadata: BallMetadata,
        player: &MatchPlayer,
        _context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        StateChangeResult::none()

        // Self::check_collision(player, objects_positions, result);
        //
        // let mut res_vec = Vec::new();
        //
        // res_vec.push(objects_positions.ball_position.x as f64);
        // res_vec.push(objects_positions.ball_position.y as f64);
        //
        // res_vec.push(objects_positions.ball_velocity.x as f64);
        // res_vec.push(objects_positions.ball_velocity.y as f64);
        //
        // let res = PLAYER_RUNNING_STATE_NETWORK.run(&res_vec);
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

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct PlayerRunningStateNetLoader;

impl PlayerRunningStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
