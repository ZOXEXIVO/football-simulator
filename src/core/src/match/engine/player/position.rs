use crate::PlayerPositionType;

#[derive(Debug, Clone)]
pub struct MatchTacticalPosition {
    pub position: PlayerPositionType,
    pub path: Vec<(f32, f32)>,
}