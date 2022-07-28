use crate::club::team::behaviour::TeamBehaviourResult;
use crate::{PlayerCollection, StaffCollection};

pub struct TeamBehaviour {}

impl TeamBehaviour {
    pub fn simulate(
        players: &mut PlayerCollection,
        staffs: &mut StaffCollection,
    ) -> TeamBehaviourResult {
        let result = TeamBehaviourResult::new();

        result
    }
}
