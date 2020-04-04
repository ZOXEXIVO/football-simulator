use crate::club::ClubContext;
use crate::core::context::GlobalContext;
use crate::core::SimulationContext;
use crate::people::StaffClubContract;

#[derive(Debug)]
pub struct ClubBoard {
    pub director: Option<StaffClubContract>,
    pub sport_director: Option<StaffClubContract>,
}

impl ClubBoard {
    pub fn new() -> Self {
        ClubBoard {
            director: None,
            sport_director: None,
        }
    }

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        if self.director.is_none() {
            self.run_director_election(&mut ctx.simulation);
        }

        if self.sport_director.is_none() {
            self.run_sport_director_election(&mut ctx.simulation);
        }

        if ctx.simulation.check_contract_expiration() {
            if self.is_director_contract_expiring(&mut ctx.simulation) {}

            if self.is_sport_director_contract_expiring(&mut ctx.simulation) {}
        }
    }

    fn is_director_contract_expiring(&self, simulation_ctx: &mut SimulationContext) -> bool {
        match &self.director {
            Some(d) => d.is_expired(simulation_ctx),
            None => false,
        }
    }

    fn run_director_election(&mut self, simulation_ctx: &mut SimulationContext) {}

    fn is_sport_director_contract_expiring(&self, simulation_ctx: &mut SimulationContext) -> bool {
        match &self.director {
            Some(d) => d.is_expired(simulation_ctx),
            None => false,
        }
    }

    fn run_sport_director_election(&mut self, context: &mut SimulationContext) {}
}
