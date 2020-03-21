#[derive(Debug)]
pub struct Behaviour {
    pub state: BehaviourState,
}

impl Behaviour {
    pub fn default() -> Self {
        Behaviour {
            state: BehaviourState::Normal,
        }
    }

    pub fn try_increase(&mut self) {
        match self.state {
            BehaviourState::Poor => {
                self.state = BehaviourState::Normal;
            }
            BehaviourState::Normal => {
                self.state = BehaviourState::Good;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum BehaviourState {
    Poor,
    Normal,
    Good,
}
