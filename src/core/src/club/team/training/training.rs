use crate::club::{Player, Staff};
use crate::{PlayerTraining, TeamTrainingResult};

#[derive(Debug)]
pub struct TeamTraining {}

impl TeamTraining {
    pub fn train_players(players: &mut Vec<Player>, coach: &Staff) -> TeamTrainingResult {
        let result = TeamTrainingResult::new();

        for player in players.iter_mut() {
            PlayerTraining::train(player, coach);

            if player.training.has_individual_training {
                PlayerTraining::personal_training(player, coach);
            }
        }

        result
    }
}
