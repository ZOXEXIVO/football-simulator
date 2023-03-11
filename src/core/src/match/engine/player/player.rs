use crate::r#match::position::FieldPosition;
use crate::r#match::{
    DefenderStrategies, FootballMatchResult, ForwardStrategies, GoalkeeperStrategies, MatchContext,
    MatchObjectsPositions, MatchState, MidfielderStrategies, PassingState, ReturningState,
    RunningState, ShootingState, StandingState, SteeringBehavior, TacklingState, WalkingState,
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
        self.move_to();

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

        let changed_state = match self.state {
            PlayerState::Standing => StandingState::process(self, result, objects_positions),
            PlayerState::Walking => WalkingState::process(self, result, objects_positions),
            PlayerState::Running => RunningState::process(self, result, objects_positions),
            PlayerState::Tackling => TacklingState::process(self, result, objects_positions),
            PlayerState::Shooting => ShootingState::process(self, result, objects_positions),
            PlayerState::Passing => PassingState::process(self, result, objects_positions),
            PlayerState::Returning => ReturningState::process(self, result, objects_positions),
        };

        if let Some(state) = changed_state {
            self.change_state(state);
        }

        //self.update_velocity(result, objects_positions, state, self.state);
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
    Passing,
    Returning,
}

pub enum PlayerUpdateEvent {
    Goal(u32),
}
