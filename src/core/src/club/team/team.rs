use crate::context::GlobalContext;
use crate::{
    MatchHistory, Player, PlayerCollection, PlayerSelector, Squad, StaffCollection, Tactics,
    TacticsSelector, TeamResult, Training, TrainingSchedule, TransferItem,
};

#[derive(Debug)]
pub struct Team {
    pub id: u32,

    pub tactics: Option<Tactics>,

    pub players: PlayerCollection,
    pub staffs: StaffCollection,

    pub training_schedule: TrainingSchedule,

    pub transfer_list: Vec<TransferItem>,

    pub match_history: Vec<MatchHistory>,
}

impl Team {
    pub fn new(
        id: u32,
        training_schedule: TrainingSchedule,
        players: PlayerCollection,
        staffs: StaffCollection,
    ) -> Self {
        Team {
            id,
            players,
            staffs,
            tactics: None,
            training_schedule,
            transfer_list: Vec::new(),
            match_history: Vec::new(),
        }
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.players()
    }

    pub fn get_week_salary(&self) -> u32 {
        let mut result: u32 = 0;

        for player in &self.players.players {
            if let Some(contract) = player.contract {
                result += contract.salary as u32
            }
        }

        for staff in &self.staffs.staffs {
            if let Some(contract) = staff.contract {
                result += contract.salary as u32
            }
        }

        result
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach),
            players: PlayerSelector::select(self, main_coach),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> TeamResult {
        let result = TeamResult::new(
            self.players.simulate(ctx.with_player(None)),
            self.staffs.simulate(ctx.with_staff()),
        );

        if self.training_schedule.is_time(ctx.simulation.date) {
            Training::train_players(&mut self.players.players, self.staffs.coaches());
        }

        result
    }
}
