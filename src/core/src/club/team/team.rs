use crate::context::GlobalContext;
use crate::{MatchHistory, Player, PlayerCollection, PlayerSelector, Squad, StaffCollection, Tactics, TacticsSelector, TeamResult, Training, TrainingSchedule, TransferItem, TeamReputation};
use std::str::FromStr;
use crate::shared::CurrencyValue;

#[derive(Debug, PartialEq)]
pub enum TeamType {
    Main = 0,
    B = 1,
    U18 = 2,
    U19 = 3,
    U21 = 4,
    U23 = 5
}

#[derive(Debug)]
pub struct Team {
    pub id: u32,
    pub league_id: u32,
    pub club_id: u32,
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
        league_id: u32,
        club_id: u32,
        name: String,
        team_type: TeamType,
        training_schedule: TrainingSchedule,
        reputation: TeamReputation,
        players: PlayerCollection,
        staffs: StaffCollection,
    ) -> Self {
        Team {
            id,
            league_id,
            club_id,
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

    pub fn add_player_to_transfer_list(&mut self, player_id: u32, value: CurrencyValue){
        self.transfer_list.push(TransferItem{
            player_id,
            amount: value
        })
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
        let head_coach = self.staffs.head_coach();

        Squad {
            team_id: self.id,
            tactics: TacticsSelector::select(self, head_coach),
            players: PlayerSelector::select(self, head_coach),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> TeamResult {
        let result = TeamResult::new(
            self.id,
            self.players.simulate(ctx.with_player(None)),
            self.staffs.simulate(ctx.with_staff(None)),
        );

        if self.training_schedule.is_time(ctx.simulation.date) {
            Training::train_players(&mut self.players.players, self.staffs.training_coach(&self.team_type));
        }
        
        if self.tactics.is_none() {
            self.tactics = Some(TacticsSelector::select(self, self.staffs.head_coach()));
        }
        
        result
    }
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
