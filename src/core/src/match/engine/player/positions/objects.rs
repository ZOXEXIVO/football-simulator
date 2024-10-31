use crate::r#match::{BallFieldData, MatchField, PlayerFieldData, PlayerFieldMetadata};

pub struct MatchObjectsPositions {
    pub ball: BallFieldData,
    pub players: PlayerFieldData,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        let positions: Vec<PlayerFieldMetadata> = field
            .players
            .iter()
            .map(|p| PlayerFieldMetadata {
                player_id: p.id,
                side: p.side.expect("unknown player side"),
                position: p.position,
                velocity: p.velocity,
            })
            .collect();

        MatchObjectsPositions {
            ball: BallFieldData::from(&field.ball),
            players: PlayerFieldData::new(positions),
        }
    }
}
