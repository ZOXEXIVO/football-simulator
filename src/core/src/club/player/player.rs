use crate::club::player::utils::PlayerUtils;
use crate::club::{
    PersonBehaviour, PersonBehaviourState, PlayerAttributes, PlayerClubContract,
    PlayerCollectionResult, PlayerMailbox, PlayerResult, PlayerSkills, PlayerTraining, Staff,
};
use crate::context::GlobalContext;
use crate::shared::fullname::FullName;
use crate::utils::{DateUtils, Logging};
use crate::{
    ContractType, Person, PersonAttributes, PlayerContractProposal, PlayerHappiness,
    PlayerMessageType, PlayerPositionType, PlayerPositions, PlayerSquadStatus, PlayerStatistics,
    PlayerStatisticsHistory, PlayerStatusData, PlayerValueCalculator, Relations,
};
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::{Display, Formatter, Result};
use std::ops::Index;

#[derive(Debug)]
pub struct Player {
    //person data
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub country_id: u32,
    pub behaviour: PersonBehaviour,
    pub attributes: PersonAttributes,

    //player data
    pub happiness: PlayerHappiness,
    pub statuses: PlayerStatusData,
    pub skills: PlayerSkills,
    pub contract: Option<PlayerClubContract>,
    pub positions: PlayerPositions,
    pub preferred_foot: PlayerPreferredFoot,
    pub player_attributes: PlayerAttributes,
    pub mailbox: PlayerMailbox,
    pub training: PlayerTraining,
    pub relations: Relations,

    pub statistics: PlayerStatistics,
    pub statistics_history: PlayerStatisticsHistory,
}

impl Player {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        country_id: u32,
        skills: PlayerSkills,
        attributes: PersonAttributes,
        player_attributes: PlayerAttributes,
        contract: Option<PlayerClubContract>,
        positions: PlayerPositions,
    ) -> Self {
        Player {
            id,
            full_name,
            birth_date,
            country_id,
            behaviour: PersonBehaviour::default(),
            happiness: PlayerHappiness::new(),
            statuses: PlayerStatusData::new(),
            skills,
            positions,
            preferred_foot: PlayerPreferredFoot::Right,
            attributes,
            player_attributes,
            contract,
            training: PlayerTraining::new(),
            mailbox: PlayerMailbox::new(),
            relations: Relations::new(),
            statistics: PlayerStatistics::new(),
            statistics_history: PlayerStatisticsHistory::new(),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> PlayerResult {
        let now = ctx.simulation.date;

        let mut result = PlayerResult::new(self.id);

        if DateUtils::is_birthday(self.birth_date, now.date()) {
            self.behaviour.try_increase();
        }

        self.process_contract(&mut result, now.clone());
        self.process_mailbox(now.date(), &mut result);

        if self.behaviour.state == PersonBehaviourState::Poor {
            result.request_transfer(self.id);
        }

        result
    }

    fn process_contract(&mut self, result: &mut PlayerResult, now: NaiveDateTime) {
        if let Some(ref mut contract) = self.contract {
            const HALF_YEAR_DAYS: i64 = 30 * 6;

            if contract.days_to_expiration(now) < HALF_YEAR_DAYS {
                result.contract.want_extend_contract = true;
            }
        } else {
            result.contract.no_contract = true;
        }
    }

    fn process_mailbox(&mut self, now: NaiveDate, result: &mut PlayerResult) {
        for message in self.mailbox.get() {
            match message.message_type {
                PlayerMessageType::Greeting => {}
                PlayerMessageType::ContractProposal(proposal) => {
                    process_contract_proposal(self, proposal, now, result);
                }
            }
        }

        fn process_contract_proposal(
            player: &mut Player,
            proposal: PlayerContractProposal,
            now: NaiveDate,
            result: &mut PlayerResult,
        ) {
            match &player.contract {
                Some(player_contract) => {
                    if proposal.salary > player_contract.salary {
                        accept_contract_proposal(player, proposal, now);
                    } else {
                        result.contract.contract_rejected = true;
                    }
                }
                None => match player.behaviour.state {
                    PersonBehaviourState::Poor => {
                        result.contract.contract_rejected = true;
                    }
                    PersonBehaviourState::Normal => {}
                    PersonBehaviourState::Good => {
                        accept_contract_proposal(player, proposal, now);
                    }
                },
            }
        }

        fn accept_contract_proposal(
            player: &mut Player,
            proposal: PlayerContractProposal,
            now: NaiveDate,
        ) {
            player.contract = Some(PlayerClubContract {
                salary: proposal.salary,
                contract_type: ContractType::FullTime,
                squad_status: PlayerSquadStatus::FirstTeamRegular,
                is_transfer_listed: false,
                transfer_status: Option::None,
                started: Option::None,
                expiration: now, //TODO ADD YEARS
                bonuses: vec![],
                clauses: vec![],
            })
        }
    }

    pub fn personal_training(&mut self, coach: &Staff) {
        PlayerTraining::personal_training(self, coach);
    }

    pub fn value(&self, date: NaiveDate) -> f64 {
        PlayerValueCalculator::calculate(self, date)
    }

    #[inline]
    pub fn positions(&self) -> Vec<PlayerPositionType> {
        self.positions.positions()
    }

    #[inline]
    pub fn position(&self) -> PlayerPositionType {
        *self.positions.positions().first().unwrap()
    }

    pub fn preferred_foot_str(&self) -> &'static str {
        match self.preferred_foot {
            PlayerPreferredFoot::Left => "Left",
            PlayerPreferredFoot::Right => "Right",
            PlayerPreferredFoot::Both => "Both",
        }
    }

    pub fn is_ready_for_match(&self) -> bool {
        match self.skills.physical.match_readiness {
            0..=10 => false,
            10..=20 => true,
            _ => false,
        }
    }

    pub fn growth_potential(&self, now: NaiveDate) -> f32 {
        PlayerUtils::growth_potential(self, now)
    }

    pub fn get_skill(&self) -> u32 {
        let positions = self.positions();
        let positions_sum: u32 = positions
            .iter()
            .map(|pos| self.skills.get_for_position(*pos))
            .sum();

        (positions_sum as f32 / positions.len() as f32) as u32
    }
}

impl Person for Player {
    fn id(&self) -> u32 {
        self.id
    }

    fn fullname(&self) -> &FullName {
        &self.full_name
    }

    fn birthday(&self) -> NaiveDate {
        self.birth_date
    }

    fn behaviour(&self) -> &PersonBehaviour {
        &self.behaviour
    }

    fn attributes(&self) -> &PersonAttributes {
        &self.attributes
    }

    fn relations(&self) -> &Relations {
        &self.relations
    }
}

#[derive(Debug)]
pub enum PlayerPreferredFoot {
    Left,
    Right,
    Both,
}

//DISPLAY
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}

const DEFAULT_PLAYER_TRANSFER_BUFFER_SIZE: usize = 10;

#[derive(Debug)]
pub struct PlayerCollection {
    pub players: Vec<Player>,
}

impl PlayerCollection {
    pub fn new(players: Vec<Player>) -> Self {
        PlayerCollection { players }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> PlayerCollectionResult {
        let player_results: Vec<PlayerResult> = self
            .players
            .iter_mut()
            .map(|player| {
                let message = &format!("simulate player: id: {}", &player.id);
                Logging::estimate_result(
                    || player.simulate(ctx.with_player(Some(player.id))),
                    message,
                )
            })
            .collect();

        let mut outgoing_players = Vec::with_capacity(DEFAULT_PLAYER_TRANSFER_BUFFER_SIZE);

        for transfer_request_player_id in player_results.iter().flat_map(|p| &p.transfer_requests) {
            outgoing_players.push(self.take(transfer_request_player_id))
        }

        PlayerCollectionResult::new(player_results, outgoing_players)
    }

    pub fn add(&mut self, players: Vec<Player>) {
        for player in players {
            self.players.push(player);
        }
    }

    pub fn get_week_salary(&self) -> u32 {
        self.players
            .iter()
            .filter_map(|p| p.contract.as_ref())
            .map(|c| c.salary)
            .sum::<u32>()
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|player| player).collect()
    }

    pub fn take(&mut self, player_id: &u32) -> Player {
        let player_idx = self
            .players
            .iter()
            .position(|p| p.id == *player_id)
            .unwrap();
        self.players.remove(player_idx)
    }
}

impl Index<u32> for PlayerCollection {
    type Output = Player;

    fn index(&self, player_id: u32) -> &Self::Output {
        &self.players.iter().find(|p| p.id == player_id).unwrap()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
