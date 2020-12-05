use crate::club::{Player, Staff};

#[derive(Debug)]
pub struct Training{
    
}

impl Training{
    pub fn train_players(players: &mut Vec<Player>, coach: &Staff){
        for player in players {
            player.train(coach);
        }      
    }
}