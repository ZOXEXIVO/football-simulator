use crate::club::{PersonBehaviourState, PlayerPositionType, Staff};
use crate::Team;

#[derive(Debug)]
pub struct Tactics {
    pub positioning: TacticsPositioning,
}

impl Tactics {
    pub fn new(positioning: TacticsPositioning) -> Self {
        Tactics { positioning }
    }

    pub fn positions(&self) -> &[PlayerPositionType; 11] {
        let (_, positions) = TACTICS_POSITIONS
            .iter()
            .find(|(positioning, _)| *positioning == self.positioning)
            .unwrap();

        positions
    }
}

const TACTICS_POSITIONS: &[(TacticsPositioning, [PlayerPositionType; 11])] = &[(
    TacticsPositioning::T442,
    [
        PlayerPositionType::Goalkeeper,
        PlayerPositionType::DefenderLeft,
        PlayerPositionType::DefenderCenter,
        PlayerPositionType::DefenderCenter,
        PlayerPositionType::DefenderRight,
        PlayerPositionType::MidfielderLeft,
        PlayerPositionType::MidfielderCenter,
        PlayerPositionType::MidfielderCenter,
        PlayerPositionType::MidfielderRight,
        PlayerPositionType::Striker,
        PlayerPositionType::Striker,
    ],
)];

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum TacticsPositioning {
    T442,
    // T235,
    // T451,
    // T433,
    // T442Diamond,
    // T442DiamondWide,
    // T442Narrow,
    // T352,
    // T4231,
    // T4141,
    // T4411,
    // T343,
    // T1333,
    // T4312,
    // T4222,
}

pub struct TacticsSelector;

impl TacticsSelector {
    pub fn select(_: &Team, coach: &Staff) -> Tactics {
        match coach.behaviour.state {
            PersonBehaviourState::Poor => Tactics::new(TacticsPositioning::T442),
            PersonBehaviourState::Normal => Tactics::new(TacticsPositioning::T442),
            PersonBehaviourState::Good => Tactics::new(TacticsPositioning::T442),
        }
    }    
}
