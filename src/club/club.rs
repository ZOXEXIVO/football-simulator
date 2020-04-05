use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{BoardContext, ClubMood, TacticsSelector, TransferItem};
use crate::core::context::GlobalContext;
use crate::people::{
    Player, PlayerCollection, PlayerContext, PlayerSelector, StaffCollection, StaffContext,
    TransferRequestNegotiation, TransferRequestNegotiationResult,
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
    pub fn items_count(&self) -> usize {
        self.players.len() + self.staffs.len()
    }

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
        self.simulate_players(ctx);

        // let ctx = &mut ctx.with_staff(StaffContext::new());
        // self.staffs.simulate(ctx);
        //
        // let mut ctx = &mut ctx.with_board(BoardContext::new());
        // self.board.simulate(ctx);
    }

    fn simulate_players(&mut self, ctx: &mut GlobalContext) {
        let ctx = ctx.with_player(PlayerContext::new());

        self.players.simulate(ctx);

        let player_ctx = ctx.player();
        
        for request_player_id in &player_ctx.transfer_requests {
            
        }
    }

    fn process_ctx(&mut self, context: PlayerContext) {
        //or transfer_request in context.transfer_requests {
        //let player = self.players.get(transfer_request);

        // match TransferRequestNegotiation::negotiate(self, player) {
        //     TransferRequestNegotiationResult::Complete => {}
        // }
        //}

        //for improve_contract_request in context.contract_improve_requests {
        // match ContractImproveRequestNegotiation::negotiate(
        //     self,
        //     self.players.get(improve_contract_request),
        // ) {
        //     ContractImproveRequestNegotiationResult::Complete => {}
        // }
        //}
    }
}
