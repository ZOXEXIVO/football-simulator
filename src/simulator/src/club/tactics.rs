use crate::{Behaviour, BehaviourState, Club, Staff};

#[derive(Debug, Clone)]
pub struct Tactics {
    pub positioning: TacticsPositioning,
}

impl Tactics {
    pub fn new(positioning: TacticsPositioning) -> Self {
        Tactics { positioning }
    }
}

#[derive(Debug, Clone)]
pub enum TacticsPositioning {
    T235,
    T442,
    T451,
    T433,
    T442Diamond,
    T442DiamondWide,
    T442Narrow,
    T352,
    T4231,
    T4141,
    T4411,
    T343,
    T1333,
    T4312,
    T4222,
}

pub struct TacticsSelector;

impl TacticsSelector {
    pub fn select(club: &Club, staff: &Staff) -> Tactics {
        match staff.behaviour.state {
            BehaviourState::Normal => Tactics::new(TacticsPositioning::T343),
            BehaviourState::Poor => Tactics::new(TacticsPositioning::T451),
            _ => Tactics::new(TacticsPositioning::T442),
        }
    }
}
