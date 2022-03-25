use chrono::NaiveDateTime;
use crate::club::{Player, Staff};
use crate::{PlayerTraining, Team, TeamTrainingResult};

#[derive(Debug)]
pub struct TeamTraining {}

impl TeamTraining {
    pub fn train(team: &mut Team, datetime: NaiveDateTime) -> TeamTrainingResult {
        if team.training_schedule.is_time(datetime) {
            return TeamTrainingResult::empty();
        }
        
        let result = TeamTrainingResult::new();
        
        let coach = team.staffs.training_coach(&team.team_type);

        for player in team.players.players.iter_mut() {
            PlayerTraining::train(player, coach);
        }

        result
    }
}
