use std::f32::NAN;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::state::{PlayerMatchState, PlayerState};
use crate::r#match::player::statistics::MatchPlayerStatistics;
use crate::r#match::{GameTickContext, MatchContext};
use crate::{
    PersonAttributes, Player, PlayerAttributes, PlayerFieldPositionGroup, PlayerPositionType,
    PlayerSkills,
};
use nalgebra::Vector3;
use std::fmt::*;
use log::info;
use crate::r#match::events::EventCollection;

#[derive(Debug, Clone)]
pub struct MatchPlayer {
    pub id: u32,
    pub position: Vector3<f32>,
    pub start_position: Vector3<f32>,
    pub attributes: PersonAttributes,
    pub team_id: u32,
    pub player_attributes: PlayerAttributes,
    pub skills: PlayerSkills,
    pub tactics_position: PlayerPositionType,
    pub velocity: Vector3<f32>,
    pub has_ball: bool,
    pub side: Option<PlayerSide>,
    pub state: PlayerState,
    pub in_state_time: u64,
    pub statistics: MatchPlayerStatistics,
    pub use_extended_state_logging: bool
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerSide {
    Left,
    Right,
}

impl MatchPlayer {
    pub fn from_player(team_id: u32, player: &Player, position: PlayerPositionType, use_extended_state_logging: bool) -> Self {
        MatchPlayer {
            id: player.id,
            position: Vector3::new(0.0, 0.0, 0.0),
            start_position: Vector3::new(0.0, 0.0, 0.0),
            attributes: player.attributes.clone(),
            team_id,
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            has_ball: false,
            side: None,
            state: match position.position_group() {
                PlayerFieldPositionGroup::Goalkeeper => {
                    PlayerState::Goalkeeper(GoalkeeperState::Standing)
                }
                PlayerFieldPositionGroup::Defender => {
                    PlayerState::Defender(DefenderState::Standing)
                }
                PlayerFieldPositionGroup::Midfielder => {
                    PlayerState::Midfielder(MidfielderState::Standing)
                }
                PlayerFieldPositionGroup::Forward => PlayerState::Forward(ForwardState::Standing),
            },
            in_state_time: 0,
            statistics: MatchPlayerStatistics::new(),
            use_extended_state_logging
        }
    }

    pub fn update(
        &mut self,
        context: &MatchContext,
        tick_context: &GameTickContext,
        events: &mut EventCollection
    ) {
        let player_events = PlayerMatchState::process(self, context, tick_context);

        events.add_from_collection(player_events);

        self.check_boundary_collision(context);
        self.move_to();
    }

    fn check_boundary_collision(&mut self, context: &MatchContext) {
        let field_width = context.field_size.width as f32 + 1.0;
        let field_height = context.field_size.height as f32 + 1.0;

        // Check if ball hits the boundary and reverse its velocity if it does
        if self.position.x <= 0.0 || self.position.x >= field_width {
            self.velocity.x = 0.0;
        }

        if self.position.y <= 0.0 || self.position.y >= field_height {
            self.velocity.y = 0.0;
        }
    }

    fn change_state(&mut self, state: PlayerState) {
        self.in_state_time = 0;
        self.state = state;
    }

    fn move_to(&mut self) {
        if !self.velocity.x.is_nan() {
            self.position.x += self.velocity.x;
        }

        if !self.velocity.y.is_nan() {
            self.position.y += self.velocity.y;
        }
    }

    pub fn heading(&self) -> f32 {
        self.velocity.y.atan2(self.velocity.x)
    }
}
