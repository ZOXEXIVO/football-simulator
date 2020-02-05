use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{SquadPlayer, TacticsSelector};
use crate::core::SimulationContext;
use crate::people::{
    Player, PlayerCollection, PlayerEventHandlers, PlayerSelector, StaffCollection,
    StaffEventHandlers,
};
use crate::utils::IntegerUtils;

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
    pub fn new(name: String, players: PlayerCollection, staffs: StaffCollection) -> Self {
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
        self.players.len() + self.staffs.len()
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.contracts.iter().map(|p| &p.player).collect()
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.get_main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach),
            players: PlayerSelector::select(self, main_coach),
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        for player_event in self.players.simulate(context) {
            PlayerEventHandlers::handle(player_event, context);
        }

        for staff_event in self.staffs.simulate(context) {
            StaffEventHandlers::handle(staff_event, context);
        }

        self.board.simulate(context);
    }
}
