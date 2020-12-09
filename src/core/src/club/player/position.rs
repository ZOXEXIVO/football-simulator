use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd)]
pub enum PlayerPositionType {
    Goalkeeper,
    Sweeper,
    DefenderLeft,
    DefenderCenter,
    DefenderRight,
    DefensiveMidfielder,
    MidfielderLeft,
    MidfielderCenter,
    MidfielderRight,
    AttackingMidfielderLeft,
    AttackingMidfielderCenter,
    AttackingMidfielderRight,
    Striker,
    WingbackLeft,
    WingbackRight
}

impl Display for PlayerPositionType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
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
