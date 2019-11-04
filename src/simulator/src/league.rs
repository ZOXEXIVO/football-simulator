use crate::core::SimulationContext;
use crate::club::Club;
use crate::schedule::Schedule;
use crate::chrono::Datelike;

pub struct League {
      pub name: String,
      pub clubs: Vec<Club>,
      pub schedule: Option<Schedule>,
      pub settings: LeagueSettings
}

impl League {
      pub fn items_count(&self) -> usize {
            return self.clubs.iter().map(|club| club.items_count()).sum();
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {            
            if self.settings.is_time_for_new_schedule(context) {
                  self.schedule = Some(Schedule::generate(&self.clubs).unwrap());
            }           

            

            for club in &mut self.clubs {
                  club.simulate(context);
            }       
      }
}

pub struct LeagueSettings{
      pub season_starting: (u8, u8),
      pub season_ending: (u8, u8) 
}

impl LeagueSettings{
   pub fn is_time_for_new_schedule(&self, context: &SimulationContext) -> bool{
        (context.date.day() as u8) == self.season_starting.0 
        && (context.date.month() as u8)  == self.season_starting.1
   }     
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_time_for_new_schedule_is_correct() {
        let mut settings = LeagueSettings{
            season_starting: (1, 3),
            season_ending: (4, 5) 
        };

        let mut context = SimulationContext{
             events: vec![],
             date: NaiveDate::from_ymd(2020, 3, 1)
        };

        let result = settings.is_time_for_new_schedule(&mut context);
        
        assert_eq!(true, result);
    }
}
