use crate::club::player::utils::PlayerUtils;
use crate::club::{
    PersonBehaviour, PersonBehaviourState, PlayerAttributes, PlayerClubContract,
    PlayerCollectionResult, PlayerMailbox, PlayerResult, PlayerSkills, PlayerTraining, Staff,
};
use crate::context::GlobalContext;
use crate::shared::fullname::FullName;
use crate::utils::{DateUtils, Logging};
use crate::{
    ContractType, Person, PersonAttributes, PlayerContractProposal, PlayerMessageType,
    PlayerPositionType, PlayerPositions, PlayerSquadStatus, PlayerStatusData, PlayerTransferStatus,
    Relations,
};
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::{Display, Formatter, Result};

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
    pub statuses: PlayerStatusData,
    pub skills: PlayerSkills,
    pub contract: Option<PlayerClubContract>,
    pub positions: PlayerPositions,
    pub preferred_foot: PlayerPreferredFoot,
    pub player_attributes: PlayerAttributes,
    pub mailbox: PlayerMailbox,
    pub training: PlayerTraining,
    pub relations: Relations,
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
            result: &mut PlayerResult
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
                    PersonBehaviourState::Normal => {
                        
                    }
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
        PlayerTraining::personal_train(self, coach);
    }

    #[inline]
    pub fn position(&self) -> PlayerPositionType {
        self.positions.position()
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
        self.skills.get_for_position(self.position())
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
        let mut result: u32 = 0;

        for player in &self.players {
            if let Some(contract) = &player.contract {
                result += contract.salary as u32
            }
        }

        result
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

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
