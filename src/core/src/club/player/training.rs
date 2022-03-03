use crate::utils::IntegerUtils;
use crate::{PersonBehaviourState, Player, Staff};

#[derive(Debug)]
pub struct PlayerTraining {
    pub has_individual_training: bool,
    pub history: PlayerTrainingHistory,
}

#[derive(Debug)]
pub struct PlayerTrainingHistory {}

impl PlayerTraining {
    pub fn new() -> Self {
        PlayerTraining {
            has_individual_training: false,
            history: PlayerTrainingHistory::new(),
        }
    }

    pub fn assign_individual_training(&mut self) {
        self.has_individual_training = true;
    }

    pub fn personal_training(player: &mut Player, coach: &Staff) {
        match coach.behaviour.state {
            PersonBehaviourState::Good => {
                player.skills.mental.train(1);
                player.skills.technical.train(1)
            }
            PersonBehaviourState::Normal => player.skills.train(1),
            PersonBehaviourState::Poor => player.skills.physical.train(2),
        }
    }

    pub fn train(player: &mut Player, coach: &Staff) {
        match coach.behaviour.state {
            PersonBehaviourState::Good => {
                player
                    .skills
                    .mental
                    .train(IntegerUtils::random(-1, 2) as i8);
                player
                    .skills
                    .technical
                    .train(IntegerUtils::random(-1, 2) as i8)
            }
            PersonBehaviourState::Normal => player.skills.train(IntegerUtils::random(-1, 2) as i8),
            PersonBehaviourState::Poor => player
                .skills
                .physical
                .train(IntegerUtils::random(-1, 1) as i8),
        }
    }
}

impl PlayerTrainingHistory {
    pub fn new() -> Self {
        PlayerTrainingHistory {}
    }
}
