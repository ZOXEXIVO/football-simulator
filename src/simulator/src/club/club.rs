use crate::staff::contract::StaffCollection;
use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::core::SimulationContext;
use crate::player::contract::PlayerCollection;
use crate::player::player::PlayerPosition;
use crate::staff::contract::StaffClubContract;
use crate::utils::IntegerUtils;
use crate::{PlayerEvent, StaffEvent};

#[derive(Debug, Clone)]
pub struct Club {
      pub id: u32,
      pub name: String,
      pub board: ClubBoard,
      pub players: PlayerCollection,
      pub staffs: StaffCollection,
      pub tactics: Option<Tactics>,
}

impl Club {
      pub fn new(
            name: String,
            players: PlayerCollection,
            staffs: StaffCollection,
      ) -> Self {
            Club {
                  id: IntegerUtils::random(0, 1_000_000) as u32,
                  board: ClubBoard::new(),
                  name,
                  players,
                  staffs,
                  tactics: None,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      fn select_tactics(&mut self) {
            
      }

      pub fn get_match_squad(&self) -> Squad {
            self.players.get_match_squad()
      }
      
      pub fn simulate(&mut self, context: &mut SimulationContext) {
            let player_events = self.players.simulate(context);
            self.handle_player_events(player_events);
            
            
            let staff_events = self.staffs.simulate(context);            
            self.handle_staff_events(staff_events);
            
            self.board.simulate(context);
      }
      
      fn handle_player_events(&mut self, events: Vec<PlayerEvent>){
            for player_event in events{
                  match player_event{
                        PlayerEvent::Birthday(age) => {

                        },
                        PlayerEvent::ContractExpired(days) => {

                        }
                  }
            }
      }

      fn handle_staff_events(&mut self, events: Vec<StaffEvent>){
            for player_event in events{
                  match player_event{
                        StaffEvent::Birthday(age) => {

                        },
                        StaffEvent::ContractExpired(days) => {

                        }
                  }
            }
      }
}
