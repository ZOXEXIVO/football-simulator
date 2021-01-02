use crate::club::squad::SquadPlayer;
use crate::club::{Staff, PlayerPositionType, Player};
use crate::{Team};

pub struct PlayerSelector;

const DEFAULT_SQUAD_SIZE: i32 = 11;
const DEFAULT_BENCH_SIZE: i32 = 6;

const POSITIONS: &[PlayerPositionType; 8] = &[
    PlayerPositionType::Goalkeeper,
    PlayerPositionType::DefenderLeft,
    PlayerPositionType::DefenderCenter,
    PlayerPositionType::DefenderRight,
    PlayerPositionType::MidfielderLeft,
    PlayerPositionType::MidfielderCenter,
    PlayerPositionType::MidfielderRight,
    PlayerPositionType::Striker
];

impl PlayerSelector {
    pub fn select<'c>(club: &'c Team, staff: &Staff) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> =
            Vec::with_capacity((DEFAULT_SQUAD_SIZE + DEFAULT_BENCH_SIZE) as usize);

        for position in POSITIONS.iter() {
            result.append(&mut PlayerSelector::select_by_type(club, staff, position));
        }

        result
    }

    fn select_by_type<'c>(
        club: &'c Team,
        staff: &Staff,
        position: &PlayerPositionType,
    ) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> = Vec::with_capacity(3);

        //let current_tactics = club.tactics.unwrap();

        //current_tactics.positioning

        let current_players: Vec<&Player> = club
            .players
            .players
            .iter()
            .filter(|p| p.position() == *position)
            .collect();

        for player in current_players {
            if staff.relations.is_favorite_player(player.id) {
                result.push(SquadPlayer::new(&player, *position))
            }
            else{
                result.push(SquadPlayer::new(&player, *position))
            }
        }

        result
    }
}
