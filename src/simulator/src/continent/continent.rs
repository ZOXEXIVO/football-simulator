use crate::{SimulationContext, Country};

pub use rayon::prelude::*;

pub struct Continent{
    pub name: String,
    pub countries: Vec<Country>
}

impl Continent{
    pub fn items_count(&self) -> usize {
        self.countries.iter().map(|country| country.items_count()).sum()
    }
    
    pub fn simulate(&mut self, context: &mut SimulationContext) {
        self.countries.par_iter_mut().for_each(|country|{
            country.simulate(&mut context.clone());
        });
        
        
    }
}