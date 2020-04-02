use crate::club::club::Club;
use crate::club::squad::SquadPlayer;
use crate::people::staff::staff::Staff;
use crate::people::{Player, PlayerPositionType};

pub struct PlayerSelector;

const DEFAULT_SQUAD_SIZE: i32 = 11;
const DEFAULT_BENCH_SIZE: i32 = 6;

impl PlayerSelector {
    pub fn select<'c>(club: &'c Club, staff: &Staff) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> =
            Vec::with_capacity((DEFAULT_SQUAD_SIZE + DEFAULT_BENCH_SIZE) as usize);

        let positions = vec![
            PlayerPositionType::Goalkeeper,
            PlayerPositionType::Defender,
            PlayerPositionType::Midfielder,
            PlayerPositionType::Forward,
        ];

        for position in positions {
            result.append(&mut PlayerSelector::select_by_type(club, staff, position));
        }

        result
    }

    fn select_by_type<'c>(
        club: &'c Club,
        staff: &Staff,
        position: PlayerPositionType,
    ) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> = Vec::with_capacity(3);

        //let current_tactics = club.tactics.unwrap();

        //current_tactics.positioning

        let current_players: Vec<&Player> = club
            .players
            .players
            .iter()
            .filter(|p| *p.position() == position)
            .collect();

        for player in current_players {
            if staff.is_favorite(&player) {
                result.push(SquadPlayer::new(&player, position))
            }

            //player.skills.physical.match_readiness
        }

        result
    }
}
