use crate::academy::AcademyPlayer;
use crate::club::academy::result::ClubAcademyResult;
use crate::club::academy::settings::AcademySettings;
use crate::context::GlobalContext;
use crate::utils::IntegerUtils;

#[derive(Debug)]
pub struct ClubAcademy {
    settings: AcademySettings,
    players: Vec<AcademyPlayer>,
    _level: u8,
}

impl ClubAcademy {
    pub fn new(level: u8) -> Self {
        ClubAcademy {
            settings: AcademySettings::default(),
            players: Vec::new(),
            _level: level,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ClubAcademyResult {
        let result = ClubAcademyResult::new();

        if self.players.len() < self.settings.players_count_range.start as usize {
            self.produce_youth_players(ctx);
        }

        result
    }

    fn produce_youth_players(&mut self, _ctx: GlobalContext<'_>) {
        for _ in 0..IntegerUtils::random(5, 15) {
            // let generated_player =
            //     PlayerGenerator::generate(country_id, ctx.simulation.date.date());
            //
            // self.players
            //     .push(AcademyPlayer::from_player(generated_player))
        }
    }
}
