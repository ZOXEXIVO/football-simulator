use crate::simulator::context::GlobalContext;
use crate::simulator::{SimulationContext, SimulatorData};

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let mut global_ctx = GlobalContext::new(SimulationContext::new(data.date));

        let continent_ctx = global_ctx.with_continent();

        for continent in &mut data.continents {
            continent.simulate(continent_ctx);
        }

        data.next_date();
    }
}
