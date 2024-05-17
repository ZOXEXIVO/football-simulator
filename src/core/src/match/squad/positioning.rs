use crate::PlayerPositionType;

pub const POSITION_POSITIONING: &[(PlayerPositionType, PositionType, PositionType)] = &[
    (
        PlayerPositionType::Goalkeeper,
        PositionType::Home(0, 275),
        PositionType::Away(840, 275),
    ),
    (
        PlayerPositionType::Sweeper,
        PositionType::Home(80, 275),
        PositionType::Away(755, 275),
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
        PositionType::Home(165, 275),
        PositionType::Away(695, 275),
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
        PositionType::Home(230, 275),
        PositionType::Away(630, 275),
    ),
    (
        PlayerPositionType::WingbackLeft,
        PositionType::Home(235, 50),
        PositionType::Away(625, 50),
    ),
    (
        PlayerPositionType::WingbackRight,
        PositionType::Home(235, 480),
        PositionType::Away(625, 480),
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
        PositionType::Home(297, 275),
        PositionType::Away(560, 275),
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
        PositionType::Away(485, 385),
    ),
    (
        PlayerPositionType::AttackingMidfielderCenter,
        PositionType::Home(360, 275),
        PositionType::Away(485, 275),
    ),
    (
        PlayerPositionType::AttackingMidfielderRight,
        PositionType::Home(360, 385),
        PositionType::Away(485, 150),
    ),
    (
        PlayerPositionType::ForwardLeft,
        PositionType::Home(395, 210),
        PositionType::Away(480, 330),
    ),
    (
        PlayerPositionType::ForwardCenter,
        PositionType::Home(395, 275),
        PositionType::Away(480, 275),
    ),
    (
        PlayerPositionType::ForwardRight,
        PositionType::Home(395, 330),
        PositionType::Away(480, 210),
    ),
    (
        PlayerPositionType::Striker,
        PositionType::Home(405, 275),
        PositionType::Away(435, 275),
    ),
];

pub enum PositionType {
    Home(i16, i16),
    Away(i16, i16),
}
