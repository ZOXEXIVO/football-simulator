use crate::PlayerPositionType;

pub const POSITION_POSITIONING: &[(PlayerPositionType, PositionType, PositionType)] = &[
    (
        PlayerPositionType::Goalkeeper,
        PositionType::Home(4, 270),
        PositionType::Away(831, 270),
    ),
    (
        PlayerPositionType::Sweeper,
        PositionType::Home(80, 270),
        PositionType::Away(755, 270),
    ),
    (
        PlayerPositionType::DefenderLeft,
        PositionType::Home(165, 85),
        PositionType::Away(670, 400),
    ),
    (
        PlayerPositionType::DefenderCenterLeft,
        PositionType::Home(165, 200),
        PositionType::Away(670, 300),
    ),
    (
        PlayerPositionType::DefenderCenter,
        PositionType::Home(185, 270),
        PositionType::Away(655, 270),
    ),
    (
        PlayerPositionType::DefenderCenterRight,
        PositionType::Home(165, 300),
        PositionType::Away(670, 200),
    ),
    (
        PlayerPositionType::DefenderRight,
        PositionType::Home(165, 400),
        PositionType::Away(670, 85),
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
        PositionType::Home(280, 85),
        PositionType::Away(560, 400),
    ),
    (
        PlayerPositionType::MidfielderCenterLeft,
        PositionType::Home(280, 200),
        PositionType::Away(560, 300),
    ),
    (
        PlayerPositionType::MidfielderCenter,
        PositionType::Home(280, 270),
        PositionType::Away(560, 270),
    ),
    (
        PlayerPositionType::MidfielderCenterLeft,
        PositionType::Home(280, 300),
        PositionType::Away(560, 200),
    ),
    (
        PlayerPositionType::MidfielderRight,
        PositionType::Home(280, 400),
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
        PositionType::Away(460, 330),
    ),
    (
        PlayerPositionType::ForwardCenter,
        PositionType::Home(385, 270),
        PositionType::Away(460, 270),
    ),
    (
        PlayerPositionType::ForwardRight,
        PositionType::Home(385, 330),
        PositionType::Away(460, 210),
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
