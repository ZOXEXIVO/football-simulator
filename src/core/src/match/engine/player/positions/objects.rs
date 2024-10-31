use crate::r#match::{BallFieldData, MatchField, PlayerFieldData};

pub struct MatchObjectsPositions {
    pub ball: BallFieldData,
    pub players: PlayerFieldData,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        MatchObjectsPositions {
            ball: BallFieldData::from(&field.ball),
            players: PlayerFieldData::from(field),
        }
    }
}
