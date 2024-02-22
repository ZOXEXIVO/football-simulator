use crate::common::NeuralNetwork;
use crate::r#match::strategies::common::MatchPlayerLogic;
use crate::r#match::PlayerState::Returning;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, PlayerUpdateEvent, StateChangeResult};

lazy_static! {
    static ref PLAYER_PASSING_STATE_NETWORK: NeuralNetwork = PlayerPassingStateNetLoader::load();
}

pub struct GoalkeeperPassingState {}

impl GoalkeeperPassingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player.skills.mental.decisions > 10.0 {
        } else {
            if in_state_time > 3 {
                if let Some(closest_teammate) = MatchPlayerLogic::closest_teammate(
                    &tick_context.objects_positions.players_positions,
                    player,
                    &context.state.match_state,
                ) {
                    let pass_modifier = if player.skills.technical.passing > 10.0 {
                        1.0
                    } else {
                        0.5
                    };

                    let pass_power = 100.0 * pass_modifier;

                    result.push(PlayerUpdateEvent::PassTo(closest_teammate, pass_power))
                }

                return StateChangeResult::with_state(Returning);
            }
        }

        StateChangeResult::none()

        // let mut res_vec = Vec::new();
        //
        // res_vec.push(objects_positions.ball_position.x as f64);
        // res_vec.push(objects_positions.ball_position.y as f64);
        //
        // res_vec.push(objects_positions.ball_velocity.x as f64);
        // res_vec.push(objects_positions.ball_velocity.y as f64);
        //
        // let res = PLAYER_PASSING_STATE_NETWORK.run(&res_vec);
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

        // if let Some(teammate_position) =
        //     objects_positions.find_closest_teammate(player, &context.state.match_state)
        // {
        //     result.push(PlayerUpdateEvent::PassTo(
        //         teammate_position,
        //         player.skills.running_speed(),
        //     ))
        // }
        //
        // Some(PlayerState::Standing)
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
