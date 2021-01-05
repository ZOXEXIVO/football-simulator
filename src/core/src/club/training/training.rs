use crate::club::{Player, Staff};

#[derive(Debug)]
pub struct Training{
    
}

impl Training{
    pub fn train_players(players: &mut Vec<Player>, coach: &Staff){
        for (_, player) in players.iter_mut().filter(|p| p.training.has_individual_training).enumerate() {
            player.personal_training(coach);
        }
        
        
    }
}