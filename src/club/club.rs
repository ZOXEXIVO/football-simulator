use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{BoardContext, ClubMood, TacticsSelector, TransferItem};
use crate::simulator::context::GlobalContext;
use crate::people::{
    Player, PlayerCollection, PlayerContext, PlayerSelector, StaffCollection, StaffContext,
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

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        self.simulate_board(ctx);
        self.simulate_players(ctx);
        self.simulate_staff(ctx);
    }

    fn simulate_board(&mut self, ctx: &mut GlobalContext) {
        let ctx = &mut ctx.with_board();
        self.board.simulate(ctx);
    }

    fn simulate_staff(&mut self, ctx: &mut GlobalContext) {
        let ctx = &mut ctx.with_staff();
        self.staffs.simulate(ctx);
    }

    fn simulate_players(&mut self, ctx: &mut GlobalContext) {
        self.players.simulate(ctx.with_player());

        let player_ctx = ctx.player();

        for request_player_id in &player_ctx.transfer_requests {}
    }
}
