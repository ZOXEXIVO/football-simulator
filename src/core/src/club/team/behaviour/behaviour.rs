use crate::club::team::behaviour::TeamBehaviourResult;
use crate::{PlayerCollection, StaffCollection};

pub struct TeamBehaviour {}

impl TeamBehaviour {
    pub fn simulate(_: &PlayerCollection, _: &StaffCollection) -> TeamBehaviourResult {
        let result = TeamBehaviourResult::new();

        result
    }
}
