use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::core::SimulationContext;
use crate::utils::IntegerUtils;
use crate::{
    Player, PlayerCollection, PlayerEvent, PlayerEventHandlers, Staff, StaffCollection, StaffEvent,
    StaffEventHandlers, TacticsSelector,
};

use std::borrow::Borrow;

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
        self.players.len()
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.players.iter().map(|p| &p.player).collect()
    }

    pub fn get_match_squad(&self) -> Squad {
        let mut squad = Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, &self.staffs.get_main_coach()),
            players: Vec::new(),
        };

        for player in &self.players.players {
            let position = player.player.position().clone();

            squad.players.push((position, player.player.clone()));
        }

        squad
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
