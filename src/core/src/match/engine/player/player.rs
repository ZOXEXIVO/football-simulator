use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    Ball, BallContext, BallState, GameTickContext, MatchBallLogic, MatchContext, PlayerTickContext,
    StateStrategy,
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
    pub team_id: u32,
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
    pub fn from_player(team_id: u32, player: &Player, position: PlayerPositionType) -> Self {
        MatchPlayer {
            player_id: player.id,
            position: Vector3::new(0.0, 0.0, 0.0),
            start_position: Vector3::new(0.0, 0.0, 0.0),
            attributes: player.attributes.clone(),
            team_id,
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            has_ball: false,
            is_home: false,
            state: PlayerState::Standing,
            in_state_time: 0,
        }
    }

    pub fn update(
        &mut self,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
    ) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        let is_ball_home_size = match context.state.ball_state {
            Some(ball_state) => ball_state == BallState::HomeSide,
            None => false,
        };

        let player_context = PlayerTickContext {
            ball_context: BallContext {
                // ball moving towards goal
                is_ball_heading_towards_goal: MatchBallLogic::ball_heading_towards_goal(
                    &tick_context.objects_positions.ball_position,
                    &self.start_position,
                ),
                ball_is_on_player_home_side: self.is_home && is_ball_home_size,
                ball_distance: tick_context
                    .objects_positions
                    .ball_position
                    .distance_to(&self.position),
            },
        };

        self.update_state(context, tick_context, player_context, &mut result);

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
                PlayerUpdateEvent::PassTo(pass_target, pass_power) => {
                    let ball_pass_vector = pass_target - ball.position;

                    ball.velocity = ball_pass_vector.normalize();
                }
                PlayerUpdateEvent::RushOut(_) => {}
                PlayerUpdateEvent::StayInGoal(_) => {}
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
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        let state_result = self.tactics_position.position_group().calculate(
            self.in_state_time,
            self,
            context,
            tick_context,
            player_context,
            result,
        );

        if let Some(state) = state_result.state {
            self.change_state(state);
        } else {
            self.in_state_time += 1;
        }

        if let Some(velocity) = state_result.velocity {
            self.velocity = velocity;
        }
    }

    fn check_collisions(&mut self) {}

    fn move_to(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
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
    PassTo(Vector3<f32>, f64),
    RushOut(u32),
    StayInGoal(u32),
}
