use crate::utils::IntegerUtils;
use crate::{PersonBehaviourState, Player, PlayerTrainingResult, Staff};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PlayerTraining {}

impl PlayerTraining {
    pub fn new() -> Self {
        PlayerTraining {}
    }

    pub fn train(player: &Player, coach: &Staff, now: NaiveDateTime) -> PlayerTrainingResult {
        let mut result = PlayerTrainingResult::new();

        let training_history = &player.training_history;

        result.set_mental(player.skills.mental.train(&player, training_history));

        result.set_technical(player.skills.technical.train(&player, training_history));

        result.set_physical(player.skills.physical.train(&player, training_history));

        match coach.behaviour.state {
            PersonBehaviourState::Good => {}
            PersonBehaviourState::Normal => {}
            PersonBehaviourState::Poor => {}
        }

        result
    }
}
