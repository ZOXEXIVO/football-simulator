use crate::club::academy::result::ClubAcademyResult;
use crate::context::GlobalContext;
use crate::Player;
use crate::club::academy::settings::AcademySettings;
use log::{debug};

#[derive(Debug)]
pub struct ClubAcademy{
    settings: AcademySettings,
    players: Vec<Player>,
    level: u8,
}

impl ClubAcademy {
    pub fn new(level: u8) -> Self {
        ClubAcademy{
            settings: AcademySettings::default(),
            players: Vec::new(),
            level
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> ClubAcademyResult {
        debug!("start simulating academy");
        
        let result = ClubAcademyResult::new();
        
        if self.players.len() < self.settings.players_count_range.start as usize {
            self.produce_youth_players();
        }

        debug!("end simulating academy");
        
        result
    }
    
    fn produce_youth_players(&mut self){
        
    }
}