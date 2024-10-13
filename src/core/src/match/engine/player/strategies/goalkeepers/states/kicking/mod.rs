use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::EventCollection;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_KICKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_kicking_data.json")));

const KICK_DISTANCE_THRESHOLD: f32 = 30.0; // Maximum distance to consider for kicking
const KICK_POWER_MULTIPLIER: f32 = 1.5; // Multiplier for kick power calculation

#[derive(Default)]
pub struct GoalkeeperKickingState {}

impl StateProcessingHandler for GoalkeeperKickingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the goalkeeper has the ball
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Standing,
            ));
        }

        // 2. Find the best teammate to kick the ball to
        let teammates = ctx.context.players.get_by_team(ctx.player.team_id);

        let best_teammate = teammates
            .iter()
            .filter(|teammate| {
                let distance = (teammate.position - ctx.player.position).magnitude();
                distance >= KICK_DISTANCE_THRESHOLD
            })
            .max_by(|a, b| {
                let dist_a = (a.position - ctx.player.position).magnitude();
                let dist_b = (b.position - ctx.player.position).magnitude();
                dist_a
                    .partial_cmp(&dist_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        if let Some(teammate) = best_teammate {
            // 3. Calculate the kick power based on the distance to the teammate
            let distance_to_teammate = (ctx.player.position - teammate.position).magnitude();
            let kick_power =
                distance_to_teammate / ctx.player.skills.technical.free_kicks * KICK_POWER_MULTIPLIER;

            // 4. Kick the ball to the teammate
            let mut events = EventCollection::new();

            events.add_player_event(PlayerEvent::PassTo(
                teammate.id,
                teammate.position,
                kick_power as f64,
            ));
            events.add_player_event(PlayerEvent::UnClaimBall(ctx.player.id));

            return Some(StateChangeResult::with_events(events));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Remain stationary while kicking the ball
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}
