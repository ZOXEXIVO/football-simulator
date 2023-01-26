use crate::r#match::position::FieldPosition;
use crate::{PersonAttributes, Player, PlayerAttributes, PlayerPositionType, PlayerSkills};
use nalgebra::Vector2;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct MatchPlayer {
    pub player_id: u32,
    pub position: FieldPosition,
    pub attributes: PersonAttributes,
    pub player_attributes: PlayerAttributes,
    pub skills: PlayerSkills,
    pub tactics_position: PlayerPositionType,
    pub velocity: Vector2<f32>,
    pub has_ball: bool,
    pub state: PlayerState,
}

impl MatchPlayer {
    pub fn from_player(player: &Player, position: PlayerPositionType) -> Self {
        MatchPlayer {
            player_id: player.id,
            position: FieldPosition::new(0.0, 0.0),
            attributes: player.attributes.clone(),
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            velocity: Vector2::new(0.0, 0.0),
            has_ball: false,
            state: PlayerState::Standing,
        }
    }

    pub fn update(&mut self) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_state(&mut result);
        self.update_condition(&mut result);
        self.update_velocity(&mut result);
        self.move_to(&mut result);

        result
    }

    fn update_state(&mut self, result: &mut Vec<PlayerUpdateEvent>) {
        match self.state {
            PlayerState::Standing => {
                self.velocity = Vector2::new(0.0, 0.0);
                // Check for transition to walking or running state
            }
            PlayerState::Walking => {
                self.velocity = self.skills.walking_speed();
                // Check for transition to standing or running state
            }
            PlayerState::Running => {
                self.velocity = self.skills.running_speed();
                // Check for transition to standing or walking state
            }
            PlayerState::Tackling => {
                // let tackling_success = self.skills.tackling() * self.player_attributes.condition;
                // if tackling_success > 50.0 {
                //     self.has_ball = true;
                // }
                // // Check for transition to standing state
                // if self.player_attributes.condition < 20.0 {
                //     self.state = PlayerState::Standing;
                // }
            }
            PlayerState::Shooting => {
                // let distance_to_goal = (self.position.x - self.field.width as i16 / 2).abs();
                // if distance_to_goal < 50 {
                //     let mut rng = thread_rng();
                //     let shot_success = rng.gen_range(0, 100);
                //
                //     let shooting_skill = self.skills.technical.finishing;
                //
                //     if shot_success < shooting_skill {
                //         if self.position.x < self.field.width as i16 / 2 {
                //             self.field.home_goals += 1;
                //         } else {
                //             self.field.away_goals += 1;
                //         }
                //     }
                // }

                self.state = PlayerState::Standing;
            }
            PlayerState::Passing => {
                // if self.has_ball {
                //     // find closest teammate
                //     let closest_teammate = self.find_closest_teammate();
                //     // calculate pass vector
                //     let pass_vector = self.calculate_pass_vector(&closest_teammate);
                //     // pass the ball to the teammate
                //     self.pass_ball(pass_vector);
                //     // transition to standing state
                //     self.state = PlayerState::Standing;
                // }
            }
        }
    }

    fn find_closest_teammate(&self) -> Option<MatchPlayer> {
        None
        // let mut closest_teammate = None;
        // let mut closest_distance = std::f32::MAX;
        //
        // for teammate in team {
        //     if player.player_id != teammate.player_id {
        //         let distance = (teammate.position.x - player.position.x).powi(2)
        //             + (teammate.position.y - player.position.y).powi(2);
        //         if distance < closest_distance {
        //             closest_distance = distance;
        //             closest_teammate = Some(teammate.clone());
        //         }
        //     }
        // }
        //
        // closest_teammate
    }

    // fn calculate_pass_vector(&self, teammate: &MatchPlayer) -> Vector {
    //     // code to calculate pass vector
    // }
    //
    // fn pass_ball(&mut self, pass_vector: Vector) {
    //     // code to pass the ball to the teammate
    // }

    fn check_ball_collision(&mut self) {}

    fn update_condition(&mut self, result: &mut Vec<PlayerUpdateEvent>) {
        // self.player_attributes.condition
    }

    fn move_to(&mut self, result: &mut Vec<PlayerUpdateEvent>) {
        self.position.x += self.velocity.x;
        if self.position.x > 140.0 {
            self.position.x = 140.0;
        }

        self.position.y += self.velocity.y;
        if self.position.y > 90.0 {
            self.position.y = 90.0;
        }
    }

    fn update_velocity(&mut self, result: &mut Vec<PlayerUpdateEvent>) {
        let condition = self.player_attributes.condition as f32;
        let max_speed = self.skills.max_speed();

        let speed = max_speed * (condition / 100.0);

        let mut rng = thread_rng();

        let random_x_val: f32 = rng.gen_range(-0.1..0.1);
        let random_y_val: f32 = rng.gen_range(-0.1..0.1);

        self.velocity = Vector2::new(speed * random_x_val, speed * random_y_val);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Standing,
    Walking,
    Running,
    Tackling,
    Shooting,
    Passing,
}

pub enum PlayerUpdateEvent {
    Goal(u32),
}
