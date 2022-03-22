use crate::club::{Player, Staff};
use crate::{PlayerTraining, Team, TeamTrainingResult};

#[derive(Debug)]
pub struct TeamTraining {}

impl TeamTraining {
    pub fn train(team: &mut Team) -> TeamTrainingResult {
        let result = TeamTrainingResult::new();

        let coach = team.staffs.training_coach(&team.team_type);

        for player in team.players.players.iter_mut() {
            PlayerTraining::train(player, coach);

            if player.training.has_individual_training {
                PlayerTraining::personal_training(player, coach);
            }
        }

        result
    }
}
