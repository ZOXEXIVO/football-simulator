use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

const MAX_SHOOTING_DISTANCE: f32 = 300.0; // Maximum distance to attempt a shot
const MIN_SHOOTING_DISTANCE: f32 = 20.0; // Minimum distance to attempt a shot (e.g., edge of penalty area)

static DEFENDER_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct DefenderRunningState {}

impl StateProcessingHandler for DefenderRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball {
            if self.is_in_shooting_range(ctx) {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Shooting,
                ));
            }

            if self.should_pass(ctx) {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Passing,
                ));
            }
        } else {
            let distance_to_ball = ctx.ball().distance();

            if !ctx.player.has_ball && distance_to_ball < 30.0 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Intercepting,
                ));
            }

            if ctx.player.has_ball && distance_to_ball >= 10.0 && distance_to_ball < 20.0 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Clearing,
                ));
            }
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.player.has_ball {
            Some(
                SteeringBehavior::Arrive {
                    target: ctx.ball().direction_to_opponent_goal(),
                    slowing_distance: 150.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        } else {
            Some(
                SteeringBehavior::Arrive {
                    target: ctx.ball().direction_to_opponent_goal(),
                    slowing_distance: 100.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {
        if ctx.player.skills.physical.stamina == 0.0 {
            return;
        }

        let stamina_reduction = |velocity: f32| -> f32 {
            let base_reduction = 0.01;
            let velocity_factor = 0.1;
            let max_reduction = 0.1;

            (base_reduction + velocity * velocity_factor).min(max_reduction)
        };

        let stamina_reduction = stamina_reduction(ctx.player.velocity.magnitude());

        ctx.player.skills.physical.stamina -= stamina_reduction;

        if ctx.player.skills.physical.stamina < 0.0 {
            ctx.player.skills.physical.stamina = 0.0;
        }
    }
}

impl DefenderRunningState {
    pub fn should_pass(&self, ctx: &StateProcessingContext) -> bool {
        let wait_ticks = match ctx.player.skills.mental.decisions {
            0.0..5.0 => 1000,
            5.0..10.0 => 800,
            10.0..13.0 => 500,
            14.0..17.0 => 100,
            17.0..20.0 => 10,
            _ => 1000,
        };

        ctx.in_state_time > wait_ticks
    }

    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal <= MAX_SHOOTING_DISTANCE && distance_to_goal >= MIN_SHOOTING_DISTANCE
    }
}
