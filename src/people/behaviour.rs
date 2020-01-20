#[derive(Debug, Clone)]
pub struct Behaviour {
    pub state: BehaviourState,
}

impl Behaviour {
    pub fn default() -> Self {
        Behaviour {
            state: BehaviourState::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BehaviourState {
    Poor,
    Normal,
    Good,
}
