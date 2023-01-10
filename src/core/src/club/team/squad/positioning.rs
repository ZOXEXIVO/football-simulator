use crate::PlayerPositionType;

pub const POSITION_POSITIONING: &[(PlayerPositionType, PositionType, PositionType)] = &[
    (
        PlayerPositionType::Goalkeeper,
        PositionType::Home(0, 50),
        PositionType::Away(150, 50),
    ),
    (
        PlayerPositionType::Sweeper,
        PositionType::Home(10, 50),
        PositionType::Away(140, 50),
    ),
    (
        PlayerPositionType::DefenderLeft,
        PositionType::Home(15, 40),
        PositionType::Away(135, 40),
    ),
    (
        PlayerPositionType::DefenderCenter,
        PositionType::Home(25, 35),
        PositionType::Away(125, 35),
    ),
    (
        PlayerPositionType::DefenderRight,
        PositionType::Home(35, 40),
        PositionType::Away(115, 40),
    ),
    (
        PlayerPositionType::DefensiveMidfielder,
        PositionType::Home(45, 50),
        PositionType::Away(105, 50),
    ),
    (
        PlayerPositionType::MidfielderLeft,
        PositionType::Home(55, 40),
        PositionType::Away(95, 40),
    ),
    (
        PlayerPositionType::MidfielderCenter,
        PositionType::Home(65, 35),
        PositionType::Away(85, 35),
    ),
    (
        PlayerPositionType::MidfielderRight,
        PositionType::Home(75, 40),
        PositionType::Away(75, 40),
    ),
    (
        PlayerPositionType::AttackingMidfielderLeft,
        PositionType::Home(85, 50),
        PositionType::Away(65, 50),
    ),
    (
        PlayerPositionType::AttackingMidfielderCenter,
        PositionType::Home(95, 50),
        PositionType::Away(55, 50),
    ),
    (
        PlayerPositionType::AttackingMidfielderRight,
        PositionType::Home(105, 50),
        PositionType::Away(45, 50),
    ),
    (
        PlayerPositionType::Striker,
        PositionType::Home(115, 50),
        PositionType::Away(35, 50),
    ),
    (
        PlayerPositionType::WingbackLeft,
        PositionType::Home(125, 40),
        PositionType::Away(25, 40),
    ),
    (
        PlayerPositionType::WingbackRight,
        PositionType::Home(135, 40),
        PositionType::Away(15, 40),
    ),
    (
        PlayerPositionType::MidfielderLeft,
        PositionType::Home(5, 20),
        PositionType::Away(145, 20),
    ),
    (
        PlayerPositionType::MidfielderCenter,
        PositionType::Home(25, 15),
        PositionType::Away(125, 15),
    ),
    (
        PlayerPositionType::MidfielderRight,
        PositionType::Home(45, 20),
        PositionType::Away(105, 20),
    ),
    (
        PlayerPositionType::AttackingMidfielderLeft,
        PositionType::Home(65, 30),
        PositionType::Away(85, 30),
    ),
    (
        PlayerPositionType::AttackingMidfielderCenter,
        PositionType::Home(85, 30),
        PositionType::Away(65, 30),
    ),
    (
        PlayerPositionType::AttackingMidfielderRight,
        PositionType::Home(105, 30),
        PositionType::Away(45, 30),
    ),
    (
        PlayerPositionType::Striker,
        PositionType::Home(25, 5),
        PositionType::Away(125, 5),
    ),
    (
        PlayerPositionType::WingbackLeft,
        PositionType::Home(15, 40),
        PositionType::Away(135, 40),
    ),
    (
        PlayerPositionType::WingbackRight,
        PositionType::Home(35, 40),
        PositionType::Away(115, 40),
    ),
];

pub enum PositionType {
    Home(u16, u16),
    Away(u16, u16),
}
