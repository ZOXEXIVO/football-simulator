use std::sync::LazyLock;
use crate::common::NeuralNetwork;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerState,
    PlayerTickContext, StateChangeResult,
};
use crate::r#match::player::events::PlayerUpdateEvent;

static GOALKEEPER_TACKLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_tackling_data.json")));

pub struct GoalkeeperTacklingState {}

impl GoalkeeperTacklingState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let minimal_distance = 30.0;

        let (_, mut nearest_opponents) = tick_context
            .objects_positions
            .player_distances
            .players_within_distance(player, minimal_distance);

        return if !nearest_opponents.is_empty() {
            nearest_opponents.sort_by(|(left_player_id, _), (right_player_id, _)| {
                let left_player = context.players.get(*left_player_id).unwrap();
                let right_player = context.players.get(*right_player_id).unwrap();

                left_player
                    .skills
                    .technical
                    .tackling
                    .partial_cmp(&right_player.skills.technical.tackling)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let (opponent_to_tackle, _) = nearest_opponents.first().unwrap();

            result.push(PlayerUpdateEvent::TacklingBall(*opponent_to_tackle));

            // TODO Own strategy
            StateChangeResult::with_state(PlayerState::Running)
        } else {
            StateChangeResult::none()
        };
    }

    // fn select_opponent_to_tackle<'mp>(
    //     player: &MatchPlayer,
    //     context: &MatchContext,
    //     nearest_opponents: &'mp [(u32, f32)],
    //     objects_positions: &MatchObjectsPositions,
    //     ball_context: &BallContext,
    // ) -> &'mp MatchPlayer {
    //     let opponent_analysis: Vec<_> = nearest_opponents
    //         .iter()
    //         .map(|(opponent_id, _)| {
    //             let opponent = context.players.get(*opponent_id).unwrap();
    //             let analysis = GOALKEEPER_TACKLING_NETWORK.analyze(&TacklingAnalysisInput {
    //                 player: player,
    //                 opponent: opponent,
    //                 objects_positions: objects_positions,
    //                 ball_context: ball_context,
    //             });
    //             (*opponent_id, analysis)
    //         })
    //         .collect();
    //
    //     let (best_opponent_id, _) = opponent_analysis
    //         .iter()
    //         .max_by(|(_, analysis_a), (_, analysis_b)| analysis_a.partial_cmp(analysis_b).unwrap())
    //         .unwrap();
    //
    //     context.players.get(best_opponent_id).unwrap()
    // }
}
