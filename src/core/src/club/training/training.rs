use crate::club::{Player, Staff};
use crate::PlayerTraining;

#[derive(Debug)]
pub struct Training{
    
}

impl Training{
    pub fn train_players(players: &mut Vec<Player>, coach: &Staff) {
        for player in players.iter_mut() {
            PlayerTraining::train(player, coach);
            
            if player.training.has_individual_training {
                PlayerTraining::personal_training(player, coach);
            }
        }
    }
}