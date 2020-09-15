use crate::simulator::context::GlobalContext;
use crate::simulator::{SimulationContext, SimulatorData};
use crate::continent::ContinentResult;

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let ctx = GlobalContext::new(SimulationContext::new(data.date));

        let results: Vec<ContinentResult> = data.continents.iter_mut()
            .map(|continent| continent.simulate(ctx.with_continent(continent.id)))
            .collect();

        for result in results {
            result.process(data);
        }
        
        data.next_date();
    }
}
