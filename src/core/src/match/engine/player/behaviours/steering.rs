use std::f32::NAN;
use crate::r#match::position::VectorExtensions;
use crate::r#match::MatchPlayer;
use nalgebra::Vector3;

pub enum SteeringBehavior {
    Seek {
        target: Vector3<f32>,
    },
    Arrive {
        target: Vector3<f32>,
        slowing_distance: f32,
    },
    Pursuit {
        target: Vector3<f32>,
        velocity: Vector3<f32>,
    },
    Evade {
        target: Vector3<f32>,
        velocity: Vector3<f32>,
    },
    Wander {
        target: Vector3<f32>,
        radius: f32,
        jitter: f32,
        distance: f32,
        angle: f32,
    },
    Flee {
        target: Vector3<f32>,
    },
}

impl SteeringBehavior {
    pub fn calculate(&self, player: &MatchPlayer) -> SteeringOutput {
        match self {
            SteeringBehavior::Seek { target } => {
                let desired_velocity = (*target - player.position).normalize();

                let steering = desired_velocity - player.velocity;

                let max_force = player.skills.physical.acceleration / 20.0;
                let steering = Self::limit_magnitude(steering, max_force);

                println!("TARGET: ({}, {})", target.x, target.y);
                //println!("Steering: {:?}", steering);

                SteeringOutput {
                    velocity: steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Arrive {
                target,
                slowing_distance,
            } => {
                let distance = (*target - player.position).length();
                let desired_velocity = (*target - player.position).normalize()
                    * (distance / *slowing_distance * player.skills.max_speed());

                let steering = desired_velocity - player.velocity;
                let max_acceleration = player.skills.max_speed();
                let steering_length = steering.norm();

                let steering_ratio = max_acceleration / steering_length;
                let mut limited_steering = steering * steering_ratio;

                if limited_steering.x == f32::NAN || limited_steering.x == f32::NAN {
                    limited_steering = Vector3::zeros();
                }

                SteeringOutput {
                    velocity: limited_steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Pursuit { target, velocity } => {
                let distance = (target - player.position).length();
                let prediction = distance / player.skills.max_speed();
                let target_position = target + (velocity * prediction);
                let desired_velocity =
                    (target_position - player.position).normalize() * player.skills.max_speed();
                let mut steering = desired_velocity - player.velocity;

                if steering.x == NAN || steering.x == NAN {
                    steering = Vector3::zeros();
                }

                SteeringOutput {
                    velocity: steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Evade { target, velocity } => {
                let distance = (target - player.position).length();
                let prediction = distance / player.skills.max_speed();
                let target_position = target + velocity * prediction;
                let desired_velocity =
                    (player.position - target_position).normalize() * player.skills.max_speed();
                let mut steering = desired_velocity - player.velocity;

                if steering.x == NAN || steering.x == NAN {
                    steering = Vector3::zeros();
                }

                SteeringOutput {
                    velocity: steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Wander {
                target,
                radius,
                jitter,
                distance,
                angle: _,
            } => {
                let rand_vec = Vector3::random_in_unit_circle().normalize() * *jitter;

                let target_position = *target + rand_vec;

                let target_offset = target_position - player.position;

                let adjusted_offset = target_offset.normalize() * *distance;
                let steering = adjusted_offset.add_scalar(player.heading() * *radius);

                SteeringOutput {
                    velocity: steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Flee { target } => {
                let target_direction = (player.position - *target).normalize();

                let desired_velocity = target_direction * player.skills.max_speed();

                let steering = desired_velocity - player.velocity;

                SteeringOutput {
                    velocity: steering,
                    rotation: 0.0,
                }
            }
        }
    }

    fn limit_magnitude(v: Vector3<f32>, max_magnitude: f32) -> Vector3<f32> {
        let current_magnitude = v.norm();
        if current_magnitude > max_magnitude {
            let ratio = max_magnitude / current_magnitude;
            v * ratio
        } else {
            v
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SteeringOutput {
    pub velocity: Vector3<f32>,
    pub rotation: f32,
}

impl SteeringOutput {
    pub fn new(velocity: Vector3<f32>, rotation: f32) -> Self {
        SteeringOutput { velocity, rotation }
    }
}
