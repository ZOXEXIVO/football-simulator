use crate::r#match::{BallState, GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent, SteeringBehavior, SteeringOutput};
use nalgebra::Vector3;
use crate::common::NeuralNetwork;
use crate::FloatUtils;

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn detect_velocity(
        context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32> {
        let behavior = match context.state.ball_state {
            Some(ball_state) => match ball_state {
                BallState::HomeSide => {
                    if player.is_home {
                        DefenderBehavior::Defend
                    } else {
                        DefenderBehavior::Support
                    }
                }
                BallState::AwaySide => {
                    if player.is_home {
                        DefenderBehavior::Support
                    } else {
                        DefenderBehavior::Defend
                    }
                }
            },
            None => DefenderBehavior::Idle,
        };

        let steering_output = match behavior {
            DefenderBehavior::Defend => {
                if DefenderStrategies::is_on_defending_half(player, &context.state) {
                    SteeringBehavior::Seek {
                        target: objects_positions.ball_positions,
                    }
                    .calculate(player)
                } else {
                    SteeringBehavior::Arrive {
                        target: Vector3::new(FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3)),
                        slowing_distance: 2.0,
                    }
                    .calculate(player)
                }
            }
            DefenderBehavior::Support => SteeringBehavior::Arrive {
                target: player.start_position,
                slowing_distance: 2.0,
            }
            .calculate(player),
            DefenderBehavior::Idle => SteeringOutput {
                velocity: Vector3::new(FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3)),
                rotation: 0.0,
            },
        };

        Vector3::new(steering_output.velocity.x, steering_output.velocity.y, 0.0)
    }

    fn is_on_defending_half(player: &MatchPlayer, state: &GameState) -> bool {
        match state.ball_state {
            Some(ball_state) => ball_state == BallState::HomeSide && player.is_home,
            None => false,
        }
    }
}

enum DefenderBehavior {
    Defend,
    Support,
    Idle,
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct DefendersNetLoader;

impl DefendersNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}