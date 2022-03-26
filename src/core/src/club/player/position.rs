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
    WingbackLeft,
    WingbackRight,
    Striker,
}

impl Display for PlayerPositionType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl PlayerPositionType {
    pub fn get_short_name(&self) -> &'static str {
        match *self {
            PlayerPositionType::Goalkeeper => "GK",
            PlayerPositionType::Sweeper => "SW",
            PlayerPositionType::DefenderLeft => "DL",
            PlayerPositionType::DefenderCenter => "DC",
            PlayerPositionType::DefenderRight => "DR",
            PlayerPositionType::DefensiveMidfielder => "DM",
            PlayerPositionType::MidfielderLeft => "ML",
            PlayerPositionType::MidfielderCenter => "MC",
            PlayerPositionType::MidfielderRight => "MR",
            PlayerPositionType::AttackingMidfielderLeft => "AML",
            PlayerPositionType::AttackingMidfielderCenter => "AMC",
            PlayerPositionType::AttackingMidfielderRight => "AMR",
            PlayerPositionType::Striker => "ST",
            PlayerPositionType::WingbackLeft => "WL",
            PlayerPositionType::WingbackRight => "WR",
        }
    }
}

#[derive(Debug)]
pub struct PlayerPositions {
    pub positions: Vec<PlayerPosition>,
}

const REQUIRED_POSITION_LEVEL: u8 = 15;

impl PlayerPositions {
    pub fn positions(&self) -> Vec<PlayerPositionType> {
        self.positions
            .iter()
            .filter(|p| p.level >= REQUIRED_POSITION_LEVEL)
            .map(|p| p.position)
            .collect()
    }

    pub fn display_positions(&self) -> Vec<&str> {
        self.positions()
            .iter()
            .map(|p| p.get_short_name())
            .collect()
    }
}

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: PlayerPositionType,
    pub level: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_position_names_is_correct() {
        assert_eq!("GK", PlayerPositionType::Goalkeeper.get_short_name());
        assert_eq!("SW", PlayerPositionType::Sweeper.get_short_name());
        assert_eq!("DL", PlayerPositionType::DefenderLeft.get_short_name());
        assert_eq!("DC", PlayerPositionType::DefenderCenter.get_short_name());
        assert_eq!("DR", PlayerPositionType::DefenderRight.get_short_name());
        assert_eq!(
            "DM",
            PlayerPositionType::DefensiveMidfielder.get_short_name()
        );
        assert_eq!("ML", PlayerPositionType::MidfielderLeft.get_short_name());
        assert_eq!("MC", PlayerPositionType::MidfielderCenter.get_short_name());
        assert_eq!("MR", PlayerPositionType::MidfielderRight.get_short_name());
        assert_eq!(
            "AML",
            PlayerPositionType::AttackingMidfielderLeft.get_short_name()
        );
        assert_eq!(
            "AMC",
            PlayerPositionType::AttackingMidfielderCenter.get_short_name()
        );
        assert_eq!(
            "AMR",
            PlayerPositionType::AttackingMidfielderRight.get_short_name()
        );
        assert_eq!("ST", PlayerPositionType::Striker.get_short_name());
        assert_eq!("WL", PlayerPositionType::WingbackLeft.get_short_name());
        assert_eq!("WR", PlayerPositionType::WingbackRight.get_short_name());
    }

    #[test]
    fn display_positions_return_with_over_15_level() {
        let positions = PlayerPositions {
            positions: vec![
                PlayerPosition {
                    position: PlayerPositionType::Goalkeeper,
                    level: 1,
                },
                PlayerPosition {
                    position: PlayerPositionType::Sweeper,
                    level: 10,
                },
                PlayerPosition {
                    position: PlayerPositionType::Striker,
                    level: 14,
                },
                PlayerPosition {
                    position: PlayerPositionType::WingbackLeft,
                    level: 15,
                },
                PlayerPosition {
                    position: PlayerPositionType::WingbackRight,
                    level: 20,
                },
            ],
        };

        let display_positions = positions.display_positions().join(",");

        assert_eq!("WL,WR", display_positions);
    }
}
