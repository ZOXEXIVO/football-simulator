use crate::people::{Behaviour, BehaviourState, PlayerAttributes, PlayerClubContract, PlayerContext, PlayerMailbox, PlayerSkills, PlayerResult, PlayerCollectionResult};
use crate::shared::fullname::FullName;
use crate::simulator::context::GlobalContext;
use crate::utils::{DateUtils, IntegerUtils};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub behaviour: Behaviour,
    pub skills: PlayerSkills,
    pub contract: Option<PlayerClubContract>,
    pub positions: Vec<PlayerPosition>,
    pub preferred_foot: PlayerFoot,
    pub attributes: PlayerAttributes,
    pub mailbox: PlayerMailbox,
}

impl Player {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        skills: PlayerSkills,
        attributes: PlayerAttributes,
        contract: Option<PlayerClubContract>,
        mut positions: Vec<PlayerPosition>,
    ) -> Self {
        positions.sort_by_key(|c| c.level);

        Player {
            id,
            full_name,
            birth_date,
            behaviour: Behaviour::default(),
            skills,
            positions,
            preferred_foot: PlayerFoot::Right,
            attributes,
            contract,
            mailbox: PlayerMailbox::new(),
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> PlayerResult {
        let mut result = PlayerResult::new();

        if DateUtils::is_birthday(self.birth_date, ctx.simulation.date.date()) {
            self.behaviour.try_increase();
        }

        if self.behaviour.state == BehaviourState::Poor {
            result.request_transfer(self.id);
        }

        self.train();

        result
    }

    pub fn position(&self) -> &PlayerPositionType {
        &self.positions.first().unwrap().position
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

    pub fn train(&mut self) {
        let change_val = IntegerUtils::random(-3, 3) as i8;

        self.skills.train(change_val);
    }
}

#[derive(Debug)]
pub enum PlayerFoot {
    Left,
    Right,
    Both,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerPositionType {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: PlayerPositionType,
    pub level: u8,
}

//DISPLAY
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}

const DEFAULT_PLAYER_TRANFER_BUFFER_SIZE: usize = 10;

#[derive(Debug)]
pub struct PlayerCollection {
    pub players: Vec<Player>,
}

impl PlayerCollection {
    pub fn new(players: Vec<Player>) -> Self {
        PlayerCollection { players }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> PlayerCollectionResult {
        let player_results: Vec<PlayerResult> = self.players.iter_mut()
            .map(|player| player.simulate(ctx.with_player(Some(player.id))))
            .collect();

        let mut outgoing_players = Vec::with_capacity(DEFAULT_PLAYER_TRANFER_BUFFER_SIZE);
                       
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

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|player| player).collect()
    }

    pub fn take(&mut self, player_id: &u32) -> Player {
        let player_idx = self.players.iter().position(|p| p.id == *player_id).unwrap();
        self.players.remove(player_idx)
    }
}

