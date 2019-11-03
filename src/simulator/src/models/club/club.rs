use crate::models::club::board::ClubBoard;
use crate::core::SimulationContext;
use crate::models::player::contract::PlayerClubContract;
use crate::models::staff::contract::StaffClubContract;
use crate::utils::IntegerUtils;

pub struct Club {
      pub id: u32,
      pub name: String,
      pub board: ClubBoard,
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
                  id: IntegerUtils::random(0, 1000000) as u32,
                  board: ClubBoard::new(),
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
            
            for staff in &mut self.staffs {
                  staff.simulate(context);
            }
      }
}
