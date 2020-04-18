use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{ClubMood, TacticsSelector, TransferItem, ClubResult};
use crate::simulator::context::GlobalContext;
use crate::people::{
    Player, PlayerCollection, PlayerSelector, StaffCollection, StaffContext,
};

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub mood: ClubMood,
    pub board: ClubBoard,
    pub tactics: Option<Tactics>,

    pub players: PlayerCollection,
    pub staffs: StaffCollection,

    pub transfer_list: Vec<TransferItem>,
}

impl Club {
    pub fn players(&self) -> Vec<&Player> {
        self.players.players()
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.get_main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach.unwrap()),
            players: PlayerSelector::select(self, main_coach.unwrap()),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> ClubResult {
        let result = ClubResult::new(
            self.board.simulate(ctx.with_board()),
            self.players.simulate(ctx.with_player(None)),
            self.staffs.simulate(ctx.with_staff())
        );
        
        result
    }
}
