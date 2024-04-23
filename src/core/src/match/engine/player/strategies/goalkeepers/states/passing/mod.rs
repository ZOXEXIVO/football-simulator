use crate::common::NeuralNetwork;
use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::PlayerState::Returning;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, PlayerUpdateEvent,
    StateChangeResult,
};

lazy_static! {
    static ref GOALKEEPER_PASSING_STATE_NETWORK: NeuralNetwork =
        DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json"));
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
                let max_distance: f32 = 30.0;

                let (mut teammates, opponents) = tick_context
                    .objects_positions
                    .player_distances
                    .players_within_distance(player, max_distance);

                if !teammates.is_empty() {
                    // Find the closest teammate

                    teammates.sort_by(|(_, distance_left), (_, distance_right)| {
                        distance_left.partial_cmp(distance_right).unwrap()
                    });

                    if let Some((closest_teammate_id, _)) = teammates.first() {
                        if let Some(teammate) = context.players.get(*closest_teammate_id) {
                            // Example logic: If the closest teammate is open, pass the ball to them
                            if teammate.has_ball == false {
                                if let Some(target) = context.players.get(*closest_teammate_id) {
                                    result.push(PlayerUpdateEvent::PassTo(target.position, 100.0));
                                    return StateChangeResult::none(); // No state change since we passed the ball
                                }
                            }
                        }
                    }
                }

                return StateChangeResult::with_state(Returning);
            }
        }

        StateChangeResult::none()
    }
}
