use crate::PlayerPositionType;

pub struct MatchTacticalPosition {
    pub position: PlayerPositionType,
    pub path: Vec<(f32, f32)>,
}