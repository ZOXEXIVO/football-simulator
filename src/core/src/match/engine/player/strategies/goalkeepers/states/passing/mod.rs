use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::PlayerState::Returning;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult,
};
use std::sync::LazyLock;

static GOALKEEPER_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

pub struct GoalkeeperPassingState {}

impl GoalkeeperPassingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
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
