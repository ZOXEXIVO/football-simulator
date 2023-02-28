use crate::r#match::position::FieldPosition;
use crate::r#match::FootballMatchDetails;
use crate::{PersonAttributes, Player, PlayerAttributes, PlayerPositionType, PlayerSkills};
use nalgebra::Vector2;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct MatchPlayer {
    pub player_id: u32,
    pub position: FieldPosition,
    pub start_position: FieldPosition,
    pub attributes: PersonAttributes,
    pub player_attributes: PlayerAttributes,
    pub skills: PlayerSkills,
    pub tactics_position: PlayerPositionType,
    pub velocity: Vector2<f32>,
    pub has_ball: bool,
    pub state: PlayerState,
    pub in_state_time: u32,
}

impl MatchPlayer {
    pub fn from_player(player: &Player, position: PlayerPositionType) -> Self {
        MatchPlayer {
            player_id: player.id,
            position: FieldPosition::new(0.0, 0.0),
            start_position: FieldPosition::new(0.0, 0.0),
            attributes: player.attributes.clone(),
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            velocity: Vector2::new(1.0, 1.0),
            has_ball: false,
            state: PlayerState::Standing,
            in_state_time: 0,
        }
    }

    pub fn update(
        &mut self,
        ball_position: &FieldPosition,
        players_positions: &Vec<FieldPosition>,
    ) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_state(&mut result, ball_position, players_positions);
        self.update_condition(&mut result);
        self.update_velocity(&mut result);
        self.move_to(&mut result, ball_position, players_positions);

        result
    }

    fn is_collision(ball_position: &FieldPosition, player_position: &FieldPosition) -> bool {
        const COLLISION_RADIUS: f32 = 2.0;

        let x_diff = (ball_position.x - player_position.x).abs();
        let y_diff = (ball_position.y - player_position.y).abs();

        x_diff <= COLLISION_RADIUS && y_diff <= COLLISION_RADIUS
    }

    pub fn handle_events(
        events: &Vec<PlayerUpdateEvent>,
        match_details: &mut FootballMatchDetails,
    ) {
        for event in events {}
    }

    fn change_state(&mut self, state: PlayerState) {
        self.in_state_time = 0;
        self.state = state;
    }

    fn update_state(
        &mut self,
        result: &mut Vec<PlayerUpdateEvent>,
        ball_position: &FieldPosition,
        players_positions: &Vec<FieldPosition>,
    ) {
        self.in_state_time += 1;

        match self.state {
            PlayerState::Standing => {
                self.velocity = Vector2::new(1.0, 1.0);
                // Check for transition to walking or running state

                if self.in_state_time > 10 {
                    self.change_state(PlayerState::Walking);
                }
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
            PlayerState::Returning => {
                let start_position = self.start_position;
                let distance_to_start = self.position.distance_to(&start_position);

                if distance_to_start > 0.0 {
                    // Calculate a velocity vector that moves the player towards their starting position
                    let direction_to_start = (start_position - self.position).normalize();
                    self.velocity = Vector2::new(direction_to_start.x, direction_to_start.y);
                } else {
                    // Player has returned to their starting position, reset velocity and state
                    self.velocity = Vector2::new(0.0, 0.0);
                    self.state = PlayerState::Standing;
                }
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

    fn move_to(
        &mut self,
        result: &mut Vec<PlayerUpdateEvent>,
        ball_position: &FieldPosition,
        players_positions: &Vec<FieldPosition>,
    ) {
        let mut rng = thread_rng();

        self.position.x = rng.gen_range(0.0..400.0);
        self.position.y = rng.gen_range(0.0..300.0);
    }

    fn update_velocity(&mut self, result: &mut Vec<PlayerUpdateEvent>) {
        let mut rng = thread_rng();

        let random_x_val: f32 = rng.gen_range(-1.0..1.0);
        let random_y_val: f32 = rng.gen_range(-1.0..1.0);

        self.velocity = Vector2::new(random_x_val, random_y_val);

        // let mut rng = thread_rng();
        //
        // let condition = self.player_attributes.condition as f32;
        // let max_speed = self.skills.max_speed();
        //
        // let speed = max_speed * (condition / 100.0);
        //
        // let random_x_val: f32 = 1.0;
        // ///rng.gen_range(-1.0..1.0);
        // let random_y_val: f32 = 1.0; //rng.gen_range(-1.0..1.0);
        //
        // self.velocity = Vector2::new(speed * random_x_val, speed * random_y_val);
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
    Returning,
}

pub enum PlayerUpdateEvent {
    Goal(u32),
}
