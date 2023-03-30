use crate::r#match::{
    DefenderStrategies, ForwardStrategies, GoalkeeperStrategies, MatchContext,
    MatchObjectsPositions, MatchState, MidfielderStrategies, PassingDecisionState, PassingState,
    ReturningState, RunningState, ShootingState, StandingState, TacklingState, WalkingState,
};
use crate::{
    PersonAttributes, Player, PlayerAttributes, PlayerFieldPositionGroup, PlayerPositionType,
    PlayerSkills,
};
use nalgebra::Vector3;

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
        current_time: u64,
        state: &MatchState,
        objects_positions: &MatchObjectsPositions,
    ) -> Vec<PlayerUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        // update state
        let player_state = self.update_state(current_time, &mut result, objects_positions);

        // set velocity
        self.update_velocity(
            current_time,
            &mut result,
            objects_positions,
            state,
            player_state,
        );

        // move
        self.move_to();

        result
    }

    pub fn handle_events(events: Vec<PlayerUpdateEvent>, context: &mut MatchContext) {
        for event in events {
            match event {
                PlayerUpdateEvent::Goal(player_id) => {}
                PlayerUpdateEvent::TacklingBall(player_id) => {}
            }
        }
    }

    fn change_state(&mut self, state: PlayerState) {
        self.in_state_time = 0;
        self.state = state;
    }

    fn update_state(
        &mut self,
        current_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> PlayerState {
        self.in_state_time += 1;

        let changed_state = match self.state {
            PlayerState::Standing => {
                StandingState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Walking => {
                WalkingState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Running => {
                RunningState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Tackling => {
                TacklingState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Shooting => {
                ShootingState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Passing => {
                PassingState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::PassingDecision => {
                PassingDecisionState::process(self.in_state_time, self, result, objects_positions)
            }
            PlayerState::Returning => {
                ReturningState::process(self.in_state_time, self, result, objects_positions)
            }
        };

        if let Some(state) = changed_state {
            self.change_state(state);
        }

        self.state
    }

    // fn calculate_pass_vector(&self, teammate: &MatchPlayer) -> Vector {
    //     // code to calculate pass vector
    // }
    //
    // fn pass_ball(&mut self, pass_vector: Vector) {
    //     // code to pass the ball to the teammate
    // }

    fn check_collisions(&mut self) {}

    fn move_to(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn update_velocity(
        &mut self,
        current_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
        player_state: PlayerState,
    ) {
        if current_time % 100 != 0 {
            return;
        }

        let velocity = match self.tactics_position.position_group() {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::detect_velocity(
                current_time,
                self,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::detect_velocity(
                current_time,
                self,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::detect_velocity(
                current_time,
                self,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::detect_velocity(
                current_time,
                self,
                result,
                objects_positions,
                state,
            ),
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
    PassingDecision,
    Passing,
    Returning,
}

pub enum PlayerUpdateEvent {
    Goal(u32),
    TacklingBall(u32),
}
