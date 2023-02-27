use crate::PlayerPositionType;

pub const POSITION_POSITIONING: &[(PlayerPositionType, PositionType, PositionType)] = &[
    (
        PlayerPositionType::Goalkeeper,
        PositionType::Home(7, 275),
        PositionType::Away(848, 275),
    ),
    (
        PlayerPositionType::Sweeper,
        PositionType::Home(80, 270),
        PositionType::Away(755, 270),
    ),
    (
        PlayerPositionType::DefenderLeft,
        PositionType::Home(165, 85),
        PositionType::Away(695, 450),
    ),
    (
        PlayerPositionType::DefenderCenterLeft,
        PositionType::Home(165, 210),
        PositionType::Away(695, 330),
    ),
    (
        PlayerPositionType::DefenderCenter,
        PositionType::Home(165, 280),
        PositionType::Away(695, 280),
    ),
    (
        PlayerPositionType::DefenderCenterRight,
        PositionType::Home(165, 330),
        PositionType::Away(695, 210),
    ),
    (
        PlayerPositionType::DefenderRight,
        PositionType::Home(165, 450),
        PositionType::Away(695, 85),
    ),
    (
        PlayerPositionType::DefensiveMidfielder,
        PositionType::Home(45, 50),
        PositionType::Away(105, 50),
    ),
    (
        PlayerPositionType::WingbackLeft,
        PositionType::Home(245, 50),
        PositionType::Away(595, 50),
    ),
    (
        PlayerPositionType::WingbackRight,
        PositionType::Home(245, 40),
        PositionType::Away(595, 50),
    ),
    (
        PlayerPositionType::MidfielderLeft,
        PositionType::Home(297, 85),
        PositionType::Away(560, 450),
    ),
    (
        PlayerPositionType::MidfielderCenterLeft,
        PositionType::Home(297, 210),
        PositionType::Away(560, 330),
    ),
    (
        PlayerPositionType::MidfielderCenter,
        PositionType::Home(297, 280),
        PositionType::Away(560, 280),
    ),
    (
        PlayerPositionType::MidfielderCenterRight,
        PositionType::Home(297, 330),
        PositionType::Away(560, 210),
    ),
    (
        PlayerPositionType::MidfielderRight,
        PositionType::Home(297, 450),
        PositionType::Away(560, 85),
    ),
    (
        PlayerPositionType::AttackingMidfielderLeft,
        PositionType::Home(360, 150),
        PositionType::Away(450, 350),
    ),
    (
        PlayerPositionType::AttackingMidfielderCenter,
        PositionType::Home(360, 260),
        PositionType::Away(450, 260),
    ),
    (
        PlayerPositionType::AttackingMidfielderRight,
        PositionType::Home(360, 350),
        PositionType::Away(450, 150),
    ),
    (
        PlayerPositionType::ForwardLeft,
        PositionType::Home(385, 210),
        PositionType::Away(480, 330),
    ),
    (
        PlayerPositionType::ForwardCenter,
        PositionType::Home(385, 270),
        PositionType::Away(480, 270),
    ),
    (
        PlayerPositionType::ForwardRight,
        PositionType::Home(385, 330),
        PositionType::Away(480, 210),
    ),
    (
        PlayerPositionType::Striker,
        PositionType::Home(385, 270),
        PositionType::Away(445, 270),
    ),
];

pub enum PositionType {
    Home(i16, i16),
    Away(i16, i16),
}
