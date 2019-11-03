use crate::core::context::SimulationContext;
use crate::models::{Staff, StaffClubContract};

pub struct ClubBoard {
    pub director: Option<StaffClubContract>, 
    pub sport_director: Option<StaffClubContract>
}

impl ClubBoard{
    pub fn new() -> Self{
        ClubBoard{
            director: None,
            sport_director: None
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if self.director.is_none() || self.is_director_contract_expiring(context){
            self.run_director_election(context);
        }

        if self.sport_director.is_none() || self.is_sport_director_contract_expiring(context){
            self.run_sport_director_election(context);
        }
    }

    fn is_director_contract_expiring(&self, context: &mut SimulationContext) -> bool{
        self.director.as_ref().unwrap().is_expired(context)      
    }

    fn run_director_election (&mut self, context: &mut SimulationContext){

    }

    fn is_sport_director_contract_expiring(&self, context: &mut SimulationContext) -> bool{
        self.director.as_ref().unwrap().is_expired(context)      
    }

    fn run_sport_director_election (&mut self, context: &mut SimulationContext){

    }
}