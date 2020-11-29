use crate::club::{
    PersonBehaviour, PersonBehaviourState, PlayerAttributes, PlayerClubContract,
    PlayerCollectionResult, PlayerMailbox, PlayerResult, PlayerSkills, PlayerTraining, Staff,
};
use crate::context::GlobalContext;
use crate::shared::fullname::FullName;
use crate::utils::{DateUtils, Logging};
use crate::{Person, PersonAttributes, PlayerStatusData, Relations, PlayerPositionType, PlayerPositions};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result};
use rayon::iter::Positions;

#[derive(Debug)]
pub struct Player {
    //person data
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
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
        let mut result = PlayerResult::new(self.id);

        if DateUtils::is_birthday(self.birth_date, ctx.simulation.date.date()) {
            self.behaviour.try_increase();
        }
        
        self.process_contract(&mut result);
        self.process_mailbox(&mut result);

        if self.behaviour.state == PersonBehaviourState::Poor {
            result.request_transfer(self.id);
        }

        result
    }

    fn process_contract(&mut self, result: &mut PlayerResult) {
        if self.contract.is_none() {
            
        }
    }

    fn process_mailbox(&mut self, result: &mut PlayerResult) {
        for message in self.mailbox.get() {
            // handle
        }
    }

    pub fn train(&mut self, coach: &Staff) {
        let training = &self.training;

        match coach.behaviour.state {
            PersonBehaviourState::Good => {
                self.skills.mental.train(1);
                self.skills.technical.train(1)
            }
            PersonBehaviourState::Normal => self.skills.train(1),
            PersonBehaviourState::Poor => self.skills.physical.train(2),
        }
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

    pub fn get_skill(&self) -> u32 {
        self.skills.get_for_position(self.position())
    }

    pub fn growth_potential(&self) -> f32 {
        let mut dap = ((self.skills.mental.determination as f32) / 5.0) * 0.05
            + ((self.attributes.ambition as f32) * 0.09)
            + ((self.attributes.professionalism as f32) * 0.115);
       
        let age = self.age(NaiveDate::from_num_days_from_ce(200));
        
        let ca = self.player_attributes.current_ability;
        let pa = self.player_attributes.potential_ability;

        if age < 24 {
            if pa <= (ca + 10) as i8 {
                dap = dap - 0.5;
            }
        }
        
        if age >= 24 && age < 29 {
            dap = dap - 0.5;
            if pa <= (ca + 10) as i8 {
                dap = dap - 0.5;
            }
        }
        
        if age >= 29 && age < 34 {
            dap = dap - 1.0;
            if pa <= (ca + 10) as i8 {
                dap = dap - 0.5;
            }
        }
        
        if age >= 34 {
            dap = dap - 1.0;
            if pa <= (ca + 10) as i8 && self.positions.position() == PlayerPositionType::Goalkeeper {
                dap = 0.5;
            }
        }

        dap = dap * 2.0;
              
        dap = dap.round();
        
        dap /= 2.0;

        dap
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
