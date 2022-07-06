use crate::context::GlobalContext;
use crate::utils::Logging;
use crate::{Team, TeamResult, TeamType};

#[derive(Debug)]
pub struct TeamCollection {
    pub teams: Vec<Team>,
}

impl TeamCollection {
    pub fn new(teams: Vec<Team>) -> Self {
        TeamCollection { teams }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> Vec<TeamResult> {
        self.teams
            .iter_mut()
            .map(|team| {
                let message = &format!("simulate team: {}", &team.name);
                Logging::estimate_result(|| team.simulate(ctx.with_team(team.id)), message)
            })
            .collect()
    }

    pub fn main_team_id(&self) -> Option<u32> {
        self.teams
            .iter()
            .find(|t| t.team_type == TeamType::Main)
            .map(|t| t.id)
    }
}
