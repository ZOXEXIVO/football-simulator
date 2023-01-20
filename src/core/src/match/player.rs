use crate::r#match::position::FieldPosition;
use crate::{PersonAttributes, Player, PlayerAttributes, PlayerPositionType, PlayerSkills};

#[derive(Debug, Copy, Clone)]
pub struct MatchPlayer {
    pub player_id: u32,
    pub position: FieldPosition,
    pub attributes: PersonAttributes,
    pub player_attributes: PlayerAttributes,
    pub skills: PlayerSkills,
    pub tactics_position: PlayerPositionType,
    pub speed: i16,
    pub has_ball: bool,
}

impl MatchPlayer {
    pub fn from_player(player: &Player, position: PlayerPositionType) -> Self {
        MatchPlayer {
            player_id: player.id,
            position: FieldPosition::new(0, 0),
            attributes: player.attributes.clone(),
            player_attributes: player.player_attributes.clone(),
            skills: player.skills.clone(),
            tactics_position: position,
            speed: 0,
            has_ball: false,
        }
    }
}
