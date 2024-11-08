use crate::r#match::{MatchField, PlayerSide};
use nalgebra::Vector3;

#[derive(Debug)]
pub struct PlayerFieldData {
    pub items: Vec<PlayerFieldMetadata>,
}

#[derive(Debug)]
pub struct PlayerFieldMetadata {
    pub player_id: u32,
    pub side: PlayerSide,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

impl PlayerFieldData {
    pub fn position(&self, player_id: u32) -> Vector3<f32> {
        let pp = self
            .items
            .iter()
            .find(|p| p.player_id == player_id)
            .map(|p| p.position);

        if let Some(p) = pp {
            p
            //.expect(&format!("no position for player = {}", player_id))
        } else {
            let pp: Vec<u32> = self.items.iter().map(|p| p.player_id).collect();

            println!("NOT FOUND: player_id = {}, State={:?}", player_id, pp);

            Vector3::zeros()
        }
    }

    pub fn velocity(&self, player_id: u32) -> Vector3<f32> {
        self.items
            .iter()
            .find(|p| p.player_id == player_id)
            .map(|p| p.velocity)
            .expect(&format!("no velocity for player = {}", player_id))
    }
}

impl From<&MatchField> for PlayerFieldData {
    #[inline]
    fn from(field: &MatchField) -> Self {
        PlayerFieldData {
            items: field
                .players
                .iter()
                .chain(field.substitutes.iter())
                .map(|p| PlayerFieldMetadata {
                    player_id: p.id,
                    side: p
                        .side
                        .expect(&format!("unknown player side, player_id = {}", p.id)),
                    position: p.position,
                    velocity: p.velocity,
                })
                .collect(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayerDistanceFromStartPosition {
    Small,
    Medium,
    Big,
}
