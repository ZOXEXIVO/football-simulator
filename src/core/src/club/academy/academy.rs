use crate::club::academy::result::ClubAcademyResult;
use crate::context::GlobalContext;
use crate::{Player, PlayerGenerator};
use crate::club::academy::settings::AcademySettings;
use crate::utils::IntegerUtils;

#[derive(Debug)]
pub struct ClubAcademy {
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

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ClubAcademyResult {
        let result = ClubAcademyResult::new();
        
        if self.players.len() < self.settings.players_count_range.start as usize {
            self.produce_youth_players(ctx);
        }
        
        result
    }
    
    fn produce_youth_players(&mut self, ctx: GlobalContext<'_>){
        //TODO Country
        let country_id = 0;
        
        for _ in 0..IntegerUtils::random(5, 15) {
            self.players.push(PlayerGenerator::generate_young_player(country_id, ctx.simulation.date.date()))
        }
    }
}