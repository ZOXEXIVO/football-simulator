use crate::r#match::{
    BallSide, MatchField, MatchObjectsPositions, PlayerDistanceClosure, Space, SphereCollider,
};
use nalgebra::Vector3;

pub struct GameTickContext {
    pub positions: MatchObjectsPositions,
    pub distances: PlayerDistanceClosure,
    pub ball: BallMetadata,
    pub space: Space,
}

impl GameTickContext {
    pub fn new(field: &MatchField) -> Self {
        GameTickContext {
            ball: BallMetadata::from(field),
            positions: MatchObjectsPositions::from(field),
            distances: PlayerDistanceClosure::from(field),
            space: Space::from(field),
        }
    }
}

pub struct BallMetadata {
    pub is_owned: bool,
    pub current_owner: Option<u32>,
    pub last_owner: Option<u32>,
}

impl From<&MatchField> for BallMetadata {
    fn from(field: &MatchField) -> Self {
        BallMetadata {
            is_owned: field.ball.current_owner.is_some(),
            current_owner: field.ball.current_owner,
            last_owner: field.ball.previous_owner,
        }
    }
}

