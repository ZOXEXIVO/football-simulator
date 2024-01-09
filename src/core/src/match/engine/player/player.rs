use crate::r#match::{
    Ball, MatchContext, MatchObjectsPositions, PlayerStateStrategy, VelocityStrategy,
};
use crate::{PersonAttributes, Player, PlayerAttributes, PlayerPositionType, PlayerSkills};
use nalgebra::Vector3;
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct MatchPlayer {
    pub player_id: u32,
    pub position: Vector3<f32>,
    pub start_position: Vector3<f32>,
    pub attributes: PersonAttributes,
    pub player_attributes: PlayerAttributes,
    pub skills: PlayerSkills,
    pub tactics_position: PlayerPositionType,
    pub velocity: Vector3<f32>,
    pub has_ball: bool,
    pub is_home: bool,
    pub state: PlayerState,
    pub in_state_time: u64,
}

impl MatchPlayer {
    pub fn from_player(player: &Player, position: PlayerPositionType) -> Self {
        MatchPlayer {
            player_id: player.id,
            position: Vector3::new(0.0, 0.0, 0.0),
            start_position: Vector3::new(0.0, 0.0, 0.0),
            attributes: player.attributes.clone(),
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            velocity: Vector3::new(1.0, 1.0, 0.0),
            has_ball: false,
            is_home: false,
            state: PlayerState::Standing,
            in_state_time: 0,
        }
    }

    pub fn update(
        &mut self,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
    ) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        let changed_state = self.update_state(context, &mut result, objects_positions);

        self.update_velocity(changed_state.is_some(), context, &mut result, objects_positions);

        self.move_to();

        result
    }

    pub fn handle_events(
        events: Vec<PlayerUpdateEvent>,
        ball: &mut Ball,
        _context: &mut MatchContext,
    ) {
        for event in events {
            match event {
                PlayerUpdateEvent::Goal(_player_id) => {}
                PlayerUpdateEvent::TacklingBall(_player_id) => {}
                PlayerUpdateEvent::PassTo(_pass_target, _player_running_speed) => {
                    //let ball_pass_vector = _pass_target - ball.position;
                    //ball.velocity = ball_pass_vector.normalize(); //* player_running_speed;
                }
            }
        }
    }

    fn change_state(&mut self, state: PlayerState) {
        self.in_state_time = 0;
        self.state = state;
    }

    fn update_state(
        &mut self,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        match self.process_state(self.in_state_time, context, result, objects_positions) {
            Some(state) => {
                self.change_state(state);
                Some(state)
            },
            None => None
        }
    }

    fn check_collisions(&mut self) {}

    fn move_to(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn update_velocity(
        &mut self,
        state_changed: bool,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) {
        if state_changed {
            if let Some(changed_velocity) = self.tactics_position.position_group().calculate_velocity(
                context,
                &self,
                result,
                objects_positions,
            ) {
                self.velocity = changed_velocity;
            }
        }
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
    Passing,
    Returning,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlayerState::Standing => write!(f, "Standing"),
            PlayerState::Walking => write!(f, "Walking"),
            PlayerState::Running => write!(f, "Running"),
            PlayerState::Tackling => write!(f, "Tackling"),
            PlayerState::Shooting => write!(f, "Shooting"),
            PlayerState::Passing => write!(f, "Passing"),
            PlayerState::Returning => write!(f, "Returning"),
        }
    }
}

pub enum PlayerUpdateEvent {
    Goal(u32),
    TacklingBall(u32),
    PassTo(Vector3<f32>, Vector3<f32>),
}
