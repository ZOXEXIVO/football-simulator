use crate::club::academy::ClubAcademy;
use crate::club::board::ClubBoard;
use crate::club::{ClubFinances, ClubMood, ClubResult};
use crate::context::GlobalContext;
use crate::shared::Location;
use crate::utils::Logging;
use crate::{Team, TeamType};

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,

    pub location: Location,

    pub mood: ClubMood,
    pub board: ClubBoard,

    pub finance: ClubFinances,

    pub academy: ClubAcademy,

    pub teams: Vec<Team>,
}

impl Club {
    pub fn new(
        id: u32,
        name: String,
        location: Location,
        finance: ClubFinances,
        teams: Vec<Team>,
    ) -> Self {
        Club {
            id,
            name,
            location,
            finance,
            academy: ClubAcademy::new(10),
            mood: ClubMood::default(),
            board: ClubBoard::new(),
            teams,
        }
    }

    pub fn main_team_id(&self) -> Option<u32> {
        self.teams
            .iter()
            .find(|t| t.team_type == TeamType::Main)
            .map(|t| t.id)
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ClubResult {
        let team_results = self
            .teams
            .iter_mut()
            .map(|team| {
                let message = &format!("simulate team: {}", &team.name);
                Logging::estimate_result(|| team.simulate(ctx.with_team(team.id, &team.name.clone())), message)
            })
            .collect();

        let result = ClubResult::new(
            self.finance.simulate(ctx.with_finance()),
            team_results,
            self.board.simulate(ctx.with_board()),
            self.academy.simulate(ctx.clone()),
        );

        if ctx.simulation.is_week_beginning() {
            for team in &self.teams {
                let weekly_salary = team.get_week_salary();
                self.finance.push_salary(ctx.club.as_ref().unwrap().name, weekly_salary as i32);
            }
        }

        result
    }
}
