use crate::core::SimulationContext;
use crate::models::player::contract::PlayerClubContract;
use crate::models::staff::contract::StaffClubContract;

pub struct Club {
      pub name: String,
      pub players: Vec<PlayerClubContract>,
      pub staffs: Vec<StaffClubContract>,
}

impl Club {
      pub fn new(
            name: String,
            players: Vec<PlayerClubContract>,
            staffs: Vec<StaffClubContract>,
      ) -> Self {
            Club {
                  name: name,
                  players: players,
                  staffs: staffs,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for player in &mut self.players {
                  player.simulate(context);
            }
      }
}
