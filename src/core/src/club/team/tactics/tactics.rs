use crate::club::{PersonBehaviourState, PlayerPositionType, Staff};
use crate::Team;

#[derive(Debug, Clone)]
pub struct Tactics {
    pub tactic_type: MatchTacticType,
}

impl Tactics {
    pub fn new(tactic_type: MatchTacticType) -> Self {
        Tactics { tactic_type }
    }

    pub fn positions(&self) -> &[PlayerPositionType; 11] {
        let (_, positions) = TACTICS_POSITIONS
            .iter()
            .find(|(positioning, _)| *positioning == self.tactic_type)
            .unwrap();

        positions
    }
}

pub const TACTICS_POSITIONS: &[(MatchTacticType, [PlayerPositionType; 11])] = &[(
    MatchTacticType::T442,
    [
        PlayerPositionType::Goalkeeper,
        PlayerPositionType::DefenderLeft,
        PlayerPositionType::DefenderCenterLeft,
        PlayerPositionType::DefenderCenterRight,
        PlayerPositionType::DefenderRight,
        PlayerPositionType::MidfielderLeft,
        PlayerPositionType::MidfielderCenterLeft,
        PlayerPositionType::MidfielderCenterRight,
        PlayerPositionType::MidfielderRight,
        PlayerPositionType::ForwardLeft,
        PlayerPositionType::ForwardRight,
    ],
)];

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub enum MatchTacticType {
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
            PersonBehaviourState::Poor => Tactics::new(MatchTacticType::T442),
            PersonBehaviourState::Normal => Tactics::new(MatchTacticType::T442),
            PersonBehaviourState::Good => Tactics::new(MatchTacticType::T442),
        }
    }
}
