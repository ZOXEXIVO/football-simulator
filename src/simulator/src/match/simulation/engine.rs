use crate::club::squad::Squad;

pub struct FootballEngine {
    home_squad: Squad,
    away_squad: Squad
}

const MATCH_DURATION_SECS: u16 = 90 * 60;


impl FootballEngine {
    const MATCH_ACTIONS: u16 = 100;
    
    pub fn new(home_squad: Squad, away_squad: Squad) -> Self {
        FootballEngine {
            home_squad,
            away_squad
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails{
        let mut result = FootballMatchDetails{
            score: Score{
                home: 0,
                away: 0
            },
            player_changes: vec![],
        };
        
                
        


        result
    }
}

pub struct FootballMatchDetails{
    pub score: Score,
    pub player_changes: Vec<PlayerChanges>
}

pub struct Score{
    pub home: u8,
    pub away: u8
}

pub struct PlayerChanges{
     
}