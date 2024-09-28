use crate::common::NeuralNetwork;
use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{ConditionContext, GameTickContext, MatchContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler};

static GOALKEEPER_SWEEPING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_sweeping_data.json")));

#[derive(Default)]
pub struct GoalkeeperSweepingState {}

impl StateProcessingHandler for GoalkeeperSweepingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl GoalkeeperSweepingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let minimal_distance = 30.0;

        let (_, mut nearest_opponents) = tick_context
            .object_positions
            .player_distances
            .players_within_distance(player, minimal_distance);

        if !nearest_opponents.is_empty() {
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
            StateChangeResult::with(PlayerState::Running)
        } else {
            StateChangeResult::new()
        }
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
