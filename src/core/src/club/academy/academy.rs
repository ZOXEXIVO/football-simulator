use crate::club::academy::result::ClubAcademyResult;
use crate::club::academy::settings::AcademySettings;
use crate::context::GlobalContext;
use crate::{PlayerCollection, StaffCollection};
use crate::utils::IntegerUtils;
    use crate::academy::result::ProduceYouthPlayersResult;


#[derive(Debug)]
pub struct ClubAcademy {
    settings: AcademySettings,
    players: PlayerCollection,
    staff: StaffCollection,
    _level: u8,
}

impl ClubAcademy {
    pub fn new(level: u8) -> Self {
        ClubAcademy {
            settings: AcademySettings::default(),
            players: PlayerCollection::new(Vec::new()),
            staff: StaffCollection::new(Vec::new()),
            _level: level,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ClubAcademyResult {
        let mut result = ClubAcademyResult::new(
            self.players.simulate(ctx.with_player(None)),
            self.produce_youth_players(ctx)
        );

        result
    }

    fn produce_youth_players(&mut self, _ctx: GlobalContext<'_>) -> ProduceYouthPlayersResult {
        let result = ProduceYouthPlayersResult::new(Vec::new());

        if self.players.players.len() < self.settings.players_count_range.start as usize {

        }

        for _ in 0..IntegerUtils::random(5, 15) {
            // let generated_player =
            //     PlayerGenerator::generate(country_id, ctx.simulation.date.date());
            //
            // self.players
            //     .push(AcademyPlayer::from_player(generated_player))
        }

        result
    }
}
