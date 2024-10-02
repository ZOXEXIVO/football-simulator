use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, PlayerDistanceFromStartPosition, PlayerSide, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::defenders::states::DefenderState;

static DEFENDER_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct DefenderRunningState {
}

impl StateProcessingHandler for DefenderRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().on_own_side() {
            let distance_to_ball = ctx.ball().distance();

            if distance_to_ball < 10.0 {
                return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
            }

            if ctx.player.has_ball && distance_to_ball >= 10.0 && distance_to_ball < 20.0 {
                return Some(StateChangeResult::with_defender_state(DefenderState::Clearing));
            }
        }

        // if ctx.player().position_to_distance() == PlayerDistanceFromStartPosition::Big {
        //     return Some(StateChangeResult::with_defender_state(DefenderState::TrackingBack));
        // }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.in_state_time == 0 {
            let target = match ctx.player.side {
                Some(PlayerSide::Left) => ctx.context.goal_positions.left,
                Some(PlayerSide::Right) => ctx.context.goal_positions.right,
                _ => Vector3::new(0.0, 0.0, 0.0)
            };

            Some(SteeringBehavior::Arrive {
                target,
                slowing_distance: 10.0,
            }.calculate(ctx.player).velocity);
        }

        None
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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    use crate::{PersonAttributes, Physical, PlayerAttributes, PlayerPositionType, PlayerSkills};
    use crate::r#match::MatchPlayer;
    use crate::r#match::player::state::PlayerState;
    use crate::r#match::statistics::MatchPlayerStatistics;

    fn create_test_player() -> MatchPlayer {
        MatchPlayer {
            id: 1,
            position: Vector3::new(0.0, 0.0, 0.0),
            start_position: Vector3::new(0.0, 0.0, 0.0),
            attributes: PersonAttributes::default(),
            team_id: 1,
            player_attributes: PlayerAttributes::default(),
            skills: PlayerSkills {
                physical: Physical {
                    stamina: 100.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            tactics_position: PlayerPositionType::DefenderCenter,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            has_ball: false,
            side: None,
            state: PlayerState::Running,
            in_state_time: 0,
            statistics: MatchPlayerStatistics::default(),
        }
    }

    #[test]
    fn test_process_conditions_stationary_player() {
        let mut player = create_test_player();
        let ctx = ConditionContext {
            in_state_time: 0,
            player: &mut player,
        };
        let state = DefenderRunningState::default();

        state.process_conditions(ctx);

        assert_eq!(player.skills.physical.stamina, 99.99);
    }

    #[test]
    fn test_process_conditions_moving_player() {
        let mut player = create_test_player();
        player.velocity = Vector3::new(1.0, 1.0, 0.0);
        let ctx = ConditionContext {
            in_state_time: 0,
            player: &mut player,
        };
        let state = DefenderRunningState::default();

        state.process_conditions(ctx);

        assert!(player.skills.physical.stamina < 99.99);
        assert!(player.skills.physical.stamina > 99.8);
    }

    #[test]
    fn test_process_conditions_sprinting_player() {
        let mut player = create_test_player();
        player.velocity = Vector3::new(5.0, 5.0, 0.0);
        let ctx = ConditionContext {
            in_state_time: 0,
            player: &mut player,
        };
        let state = DefenderRunningState::default();

        state.process_conditions(ctx);

        assert_eq!(player.skills.physical.stamina, 99.9);
    }

    #[test]
    fn test_process_conditions_exhausted_player() {
        let mut player = create_test_player();
        player.skills.physical.stamina = 0.0;
        player.velocity = Vector3::new(1.0, 1.0, 0.0);
        let ctx = ConditionContext {
            in_state_time: 0,
            player: &mut player,
        };
        let state = DefenderRunningState::default();

        state.process_conditions(ctx);

        assert_eq!(player.skills.physical.stamina, 0.0);
    }

    #[test]
    fn test_process_conditions_near_exhaustion() {
        let mut player = create_test_player();
        player.skills.physical.stamina = 0.05;
        player.velocity = Vector3::new(1.0, 1.0, 0.0);
        let ctx = ConditionContext {
            in_state_time: 0,
            player: &mut player,
        };
        let state = DefenderRunningState::default();

        state.process_conditions(ctx);

        assert_eq!(player.skills.physical.stamina, 0.0);
    }
}