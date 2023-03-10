﻿use crate::r#match::position::FieldPosition;
use crate::r#match::{
    DefenderStrategies, FootballMatchResult, ForwardStrategies, GoalkeeperStrategies, MatchContext,
    MatchObjectsPositions, MatchState, MidfielderStrategies, SteeringBehavior,
};
use crate::{
    PersonAttributes, Player, PlayerAttributes, PlayerFieldPositionGroup, PlayerPositionType,
    PlayerSkills, PlayerStatusType,
};
use nalgebra::Vector2;

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
    pub is_home: bool,
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
            is_home: false,
            state: PlayerState::Standing,
            in_state_time: 0,
        }
    }

    pub fn update(
        &mut self,
        state: &MatchState,
        objects_positions: &MatchObjectsPositions,
    ) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_state(&mut result, objects_positions);
        self.update_velocity(&mut result, objects_positions, state, self.state);

        self.move_to();

        self.update_condition(&mut result, objects_positions);

        result
    }

    pub fn handle_events(events: Vec<PlayerUpdateEvent>, context: &mut MatchContext) {
        for event in events {}
    }

    fn change_state(&mut self, state: PlayerState) {
        self.in_state_time = 0;
        self.state = state;
    }

    fn update_state(
        &mut self,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
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
                let direction = SteeringBehavior::Seek {
                    target: FieldPosition::new(848.0, 275.0),
                }
                .calculate(self);

                self.velocity = Vector2::new(direction.velocity.x, direction.velocity.y);
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
            PlayerState::PassDecision => {
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
                if self.position.distance_to(&self.start_position) < 10.0 {
                    self.change_state(PlayerState::Standing);
                } else {
                    if self.in_state_time == 0 {
                        let calculated_steering = SteeringBehavior::Seek {
                            target: self.start_position,
                        }
                        .calculate(self);

                        // return Vector2::new(
                        //     calculated_steering.velocity.x,
                        //     calculated_steering.velocity.y,
                        // );
                    }
                }
            }
        }
    }

    // fn find_closest_teammate(&self, state: &MatchState) -> Option<&MatchPlayer> {
    //     let max_pass_distance = 20.0; // Maximum distance a player can pass the ball
    //     let mut closest_teammate = None;
    //     let mut closest_distance = std::f32::MAX;
    //
    //     for player in state.players.iter() {
    //         if player.is_home == self.is_home && player != self && !player.attributes.is_marked {
    //             let distance = self.position.distance_to(&player.position);
    //             if distance < closest_distance && distance < max_pass_distance {
    //                 closest_teammate = Some(player);
    //                 closest_distance = distance;
    //             }
    //         }
    //     }
    //
    //     closest_teammate
    // }

    // fn calculate_pass_vector(&self, teammate: &MatchPlayer) -> Vector {
    //     // code to calculate pass vector
    // }
    //
    // fn pass_ball(&mut self, pass_vector: Vector) {
    //     // code to pass the ball to the teammate
    // }

    fn check_ball_collision(&mut self) {}

    fn update_condition(
        &mut self,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) {
        // self.player_attributes.condition
    }

    fn move_to(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn update_velocity(
        &mut self,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
        player_state: PlayerState,
    ) {
        let velocity = match self.tactics_position.position_group() {
            PlayerFieldPositionGroup::Goalkeeper => {
                GoalkeeperStrategies::detect_velocity(self, result, objects_positions, state)
            }
            PlayerFieldPositionGroup::Defender => {
                DefenderStrategies::detect_velocity(self, result, objects_positions, state)
            }
            PlayerFieldPositionGroup::Midfielder => {
                MidfielderStrategies::detect_velocity(self, result, objects_positions, state)
            }
            PlayerFieldPositionGroup::Forward => {
                ForwardStrategies::detect_velocity(self, result, objects_positions, state)
            }
        };

        self.velocity = velocity;
    }

    pub fn heading(&self) -> f32 {
        self.velocity.y.atan2(self.velocity.x)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Standing,
    Walking,
    Running,
    Tackling,
    Shooting,
    PassDecision,
    Passing,
    Returning,
}

pub enum PlayerUpdateEvent {
    Goal(u32),
}