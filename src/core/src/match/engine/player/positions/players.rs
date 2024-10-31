use nalgebra::Vector3;
use crate::r#match::PlayerFieldMetadata;

pub struct PlayerFieldData {
    pub items: Vec<PlayerFieldMetadata>,
}

impl PlayerFieldData {
    pub fn new(players_positions: Vec<PlayerFieldMetadata>) -> Self {
        PlayerFieldData {
            items: players_positions,
        }
    }

    pub fn get_player_position(&self, player_id: u32) -> Option<Vector3<f32>> {
        self.items
            .iter()
            .find(|p| p.player_id == player_id)
            .map(|p| p.position)
    }

    pub fn get_player_velocity(&self, player_id: u32) -> Option<Vector3<f32>> {
        self.items
            .iter()
            .find(|p| p.player_id == player_id)
            .map(|p| p.velocity)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayerDistanceFromStartPosition {
    Small,
    Medium,
    Big,
}