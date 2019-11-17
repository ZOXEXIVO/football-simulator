use crate::club::squad::Squad;
use crate::player::player::Player;

pub struct FootballEngine {
    home_squad: Squad,
    away_squad: Squad
}

const TOTAL_HALF_SECONDS: u16 = 45 * 60;

impl FootballEngine {  
    pub fn new(home_squad: Squad, away_squad: Squad) -> Self {
        FootballEngine {
            home_squad: home_squad,
            away_squad: away_squad
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

        self.play_half(&mut result);
        self.wait_next_half(&mut result);
        self.play_half(&mut result);

        result
    }

    fn wait_next_half(&mut self, result: &mut FootballMatchDetails) {
        
    }

    fn play_half(&mut self, result: &mut FootballMatchDetails) {
        for tick in 0..TOTAL_HALF_SECONDS {
            for player in &self.home_squad.players{
                
            }

            for player in &self.away_squad.players{
            
            }
        }       
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