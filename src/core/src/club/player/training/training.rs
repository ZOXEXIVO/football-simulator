use crate::common::NeuralNetwork;
use crate::utils::IntegerUtils;
use crate::{PersonBehaviourState, Player, PlayerTrainingResult, Staff, TrainingNetLoader};
use chrono::NaiveDateTime;

lazy_static! {
    pub static ref TRAINING_NET: NeuralNetwork = TrainingNetLoader::load();
}

#[derive(Debug)]
pub struct PlayerTraining;

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

        let vec = vec![
            player.skills.mental.anticipation as f64,
            player.skills.mental.decisions as f64,
        ];

        let run_results = TRAINING_NET.run(&vec);

        result.mental.diff = run_results[0];

        println!("diff = {}", result.mental.diff);

        match coach.behaviour.state {
            PersonBehaviourState::Good => {}
            PersonBehaviourState::Normal => {}
            PersonBehaviourState::Poor => {}
        }

        result
    }
}
