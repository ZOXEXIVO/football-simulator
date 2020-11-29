#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerPositionType {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Debug)]
pub struct PlayerPositions {
    pub positions: Vec<PlayerPosition>
}

impl PlayerPositions {
    pub fn position(&self) -> PlayerPositionType {
        let max_position = self.positions.iter()
            .max_by(|a, b| a.level.cmp(&b.level))
            .unwrap();
        
        max_position.position
    }
}

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: PlayerPositionType,
    pub level: u8,
}
