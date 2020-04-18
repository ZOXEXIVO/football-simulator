use crate::simulator::context::GlobalContext;
use crate::simulator::{SimulationContext, SimulatorData};

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let global_ctx = GlobalContext::new(SimulationContext::new(data.date));

        for continent in &mut data.continents {
            continent.simulate(global_ctx.with_continent(continent.id));
        }

        data.next_date();
    }
}
