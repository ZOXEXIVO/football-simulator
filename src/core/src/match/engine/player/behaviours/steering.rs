use crate::r#match::position::VectorExtensions;
use crate::r#match::MatchPlayer;
use nalgebra::Vector3;
use rand::Rng;

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

                if limited_steering.x.is_nan() || limited_steering.y.is_nan() {
                    limited_steering = Vector3::zeros();
                }

                SteeringOutput {
                    velocity: limited_steering,
                    rotation: 0.0,
                }
            }
            SteeringBehavior::Pursuit { target, velocity } => {
                let to_target = target - player.position;
                let distance = to_target.length();

                // Define a slowing radius
                let slowing_radius = 5.0; // Adjust this value as needed

                let target_speed = if distance > slowing_radius {
                    player.skills.max_speed()
                } else {
                    player.skills.max_speed() * (distance / slowing_radius)
                };

                let desired_velocity = to_target.normalize() * target_speed;
                let steering = desired_velocity - player.velocity;

                // Apply a maximum force to the steering
                let max_force: f32 = 10.0; // Adjust this value as needed
                let steering = steering.normalize() * max_force.min(steering.length());

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

                if steering.x.is_nan() || steering.y.is_nan() {
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
                angle: f32,
            } => {
                let mut rng = rand::thread_rng();

                // Generate a random angle each time
                let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;

                // Create a displacement based on the random angle
                let displacement = Vector3::new(
                    angle.cos() * *radius,
                    angle.sin() * *radius,
                    0.0
                );

                // Add some randomness to the displacement
                let jitter_offset = Vector3::new(
                    rng.gen::<f32>() * *jitter - *jitter * 0.5,
                    rng.gen::<f32>() * *jitter - *jitter * 0.5,
                    0.0
                );

                // Calculate the wander target
                let wander_target = *target + displacement + jitter_offset;

                // Calculate the wandering force
                let wandering_force = wander_target - player.position;

                // Calculate the steering force
                let steering_force = wandering_force - player.velocity;

                // Limit the magnitude of the steering force
                let max_force = 1.0; // Adjust this value as needed
                let steering_force = if steering_force.magnitude() > max_force {
                    steering_force.normalize() * max_force
                } else {
                    steering_force
                };

                // Calculate the new velocity
                let new_velocity = (player.velocity + steering_force).normalize() * *distance;

                SteeringOutput {
                    velocity: new_velocity,
                    rotation: new_velocity.y.atan2(new_velocity.x),
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
