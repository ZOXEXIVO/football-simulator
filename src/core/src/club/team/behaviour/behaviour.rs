use crate::club::team::behaviour::TeamBehaviourResult;
use crate::{PlayerCollection, StaffCollection, Team};

pub struct TeamBehaviour {}

impl TeamBehaviour {
    pub fn simulate(players: &PlayerCollection, staff: &StaffCollection) -> TeamBehaviourResult {
        let result = TeamBehaviourResult::new();

        result
    }
}
