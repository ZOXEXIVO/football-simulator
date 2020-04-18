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

    pub fn simulate(&mut self, ctx: GlobalContext) {
        //self.process_incoming_transfers(&ctx);
        
        self.simulate_board(ctx.with_board());
        self.simulate_players(ctx.with_player());
        self.simulate_staff(ctx.with_staff());
    }

    fn simulate_board(&mut self, ctx: GlobalContext) {
        self.board.simulate(ctx);
    }

    fn simulate_staff(&mut self, ctx: GlobalContext) {
        self.staffs.simulate(ctx);
    }

    // fn process_incoming_transfers(&mut self, ctx: &GlobalContext){
    //     let continent = ctx.continent();
    //     let transfer_pool = &mut continent.borrow_mut().transfer_pool;
    //     if let Some(income_players) = transfer_pool.pull_transfers(self.id) {
    //         self.players.add(income_players);
    //     }
    // }
    
    fn simulate_players(&mut self, ctx: GlobalContext) {
        self.players.simulate(ctx);
        
        // let request_transfers = global_ctx.player().borrow().transfer_requests.clone();
        //
        // let continent = global_ctx.continent();
        // let transfer_pool = &mut continent.borrow_mut().transfer_pool;
        //
        // for request_player_id in request_transfers {
        //     let player_to_transfer = self.players.take(request_player_id);
        //    
        //     transfer_pool.push_transfer(player_to_transfer, 0);
        // }
    }
}
