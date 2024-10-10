use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler, VectorExtensions};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::{PlayerUpdateEvent, PlayerUpdateEventCollection};

static GOALKEEPER_DISTRIBUTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_distributing_data.json")));

const DISTRIBUTION_DISTANCE_THRESHOLD: f32 = 50.0;

#[derive(Default)]
pub struct GoalkeeperDistributingState {}

impl StateProcessingHandler for GoalkeeperDistributingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the goalkeeper has the ball
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::Standing));
        }

        // If not under immediate pressure, look for assist opportunities
        if let Some(teammate_id) = self.find_best_teammate_to_distribute(ctx) {
            // 3. Calculate the pass power based on the distance to the teammate
            let (pass_power, position) = self.calculate_pass_power(teammate_id, ctx);

            // 4. Distribute the ball to the teammate
            let events = PlayerUpdateEventCollection::with_event(
                PlayerUpdateEvent::PassTo(
                    teammate_id,
                    position,
                    pass_power,
                ),
            );

            return Some(StateChangeResult::with_events(events));
        }

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

impl GoalkeeperDistributingState {
    fn find_best_teammate_to_distribute(&self, ctx: &StateProcessingContext) -> Option<u32> {
        ctx.tick_context
            .object_positions
            .player_distances
            .find_closest_teammates(ctx.player)
            .and_then(|teammates| {
                teammates.iter()
                    .filter(|(id, _)| self.is_in_good_scoring_position(ctx, *id))
                    .min_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(id, _)| *id)
            })
    }

    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> (f64, Vector3<f32>) {
        let position = ctx.tick_context.object_positions.players_positions.get_player_position(teammate_id).unwrap();

        let distance = ctx.tick_context.object_positions.
            player_distances.get(ctx.player.id, teammate_id)
            .unwrap();

        let pass_skill = ctx.player.skills.technical.passing;
        ((distance / pass_skill as f32 * 10.0) as f64, position)
    }

    fn is_in_good_scoring_position(&self, ctx: &StateProcessingContext, player_id: u32) -> bool {
        if let Some(player) = ctx.context.players.get(player_id) {
            let distance_to_goal = ctx.ball().distance_to_opponent_goal();
            distance_to_goal < 20.0 // Adjust based on your game's scale
        } else {
            false
        }
    }
}
