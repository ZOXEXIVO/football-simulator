use crate::common::NeuralNetwork;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

lazy_static! {
    static ref PLAYER_PASSING_STATE_NETWORK: NeuralNetwork = PlayerPassingStateNetLoader::load();
}

pub struct PassingState {}

impl PassingState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions
    ) -> Option<PlayerState> {
        if let Some(teammate_position) =
            objects_positions.find_closest_teammate(player, &context.state.match_state)
        {
            result.push(PlayerUpdateEvent::PassTo(
                teammate_position,
                player.skills.running_speed(),
            ))
        }

        Some(PlayerState::Standing)
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_passing_data.json");

#[derive(Debug)]
pub struct PlayerPassingStateNetLoader;

impl PlayerPassingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}