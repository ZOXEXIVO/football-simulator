pub struct TeamBehaviourResult {
    players: PlayerBehaviourResult,
    staff: StaffBehaviourResult,
}

impl TeamBehaviourResult {
    pub fn new() -> Self {
        TeamBehaviourResult {
            players: PlayerBehaviourResult::new(),
            staff: StaffBehaviourResult::new(),
        }
    }
}

pub struct PlayerBehaviourResult {}

impl PlayerBehaviourResult {
    pub fn new() -> Self {
        PlayerBehaviourResult {}
    }
}

pub struct StaffBehaviourResult {}

impl StaffBehaviourResult {
    pub fn new() -> Self {
        StaffBehaviourResult {}
    }
}
