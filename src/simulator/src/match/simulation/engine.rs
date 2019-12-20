use crate::club::squad::Squad;
use crate::player::player::{PlayerPositionType};

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
        
        let mut home_team = self.get_team_for_squad(&self.home_squad);
        let mut away_team = self.get_team_for_squad(&self.away_squad);
        
        

        result
    }
    
    fn get_team_for_squad(&self, squad: &Squad) -> MatchTeam {
        let mut team = MatchTeam::new();
        
        for player in squad.players.iter().map(|p| &p.1) {
            match &player.position() {
                PlayerPositionType::Defender =>{
                    team.defender_skill += player.get_skill();
                },
                PlayerPositionType::Midfielder =>{
                    team.midfielder_skill += player.get_skill();
                },
                PlayerPositionType::Striker =>{
                    team.striker_skill += player.get_skill();
                },
                _ => { }
            }
        }

        team
    }
}

struct MatchTeam {
    pub defender_skill: u32,
    pub midfielder_skill: u32,
    pub striker_skill: u32,
}

impl MatchTeam{
    pub fn new() -> Self{
        MatchTeam{
            defender_skill: 0,
            midfielder_skill: 0,
            striker_skill: 0
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