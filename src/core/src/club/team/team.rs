use crate::context::GlobalContext;
use crate::{MatchHistory, Player, PlayerCollection, PlayerSelector, Squad, StaffCollection, Tactics, TacticsSelector, TeamResult, Training, TrainingSchedule, TransferItem, TeamReputation};
use std::str::FromStr;

#[derive(Debug)]
pub struct Team {
    pub id: u32,
    pub name: String,    
    pub team_type: TeamType,    
    pub tactics: Option<Tactics>,

    pub players: PlayerCollection,
    pub staffs: StaffCollection,

    pub reputation: TeamReputation,
    pub training_schedule: TrainingSchedule,
    pub transfer_list: Vec<TransferItem>,
    pub match_history: Vec<MatchHistory>
}

impl Team {
    pub fn new(
        id: u32,
        name: String,
        team_type: TeamType,
        training_schedule: TrainingSchedule,
        reputation: TeamReputation,
        players: PlayerCollection,
        staffs: StaffCollection,
    ) -> Self {
        Team {
            id,
            name,
            team_type,
            players,
            staffs,
            reputation,
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
            if let Some(contract) = &player.contract {
                result += contract.salary as u32
            }
        }

        for staff in &self.staffs.staffs {
            if let Some(contract) = &staff.contract {
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

#[derive(Debug, PartialEq)]
pub enum TeamType {
    Main = 0,
    B = 1,
    U18 = 2,
    U19 = 3,
    U21 = 4,
    U23 = 5
}

impl FromStr for TeamType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Main" => Ok(TeamType::Main),
            "B" => Ok(TeamType::B),
            "U18" => Ok(TeamType::U18),
            "U19" => Ok(TeamType::U19),
            "U21" => Ok(TeamType::U21),
            "U23" => Ok(TeamType::U23),
            _ => Err(format!("'{}' is not a valid value for WSType", s)),
        }
    }
}
