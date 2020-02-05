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

        result.append(&mut PlayerSelector::select_goalkeepers(club, staff));

        result
    }

    fn select_goalkeepers<'c>(club: &'c Club, staff: &Staff) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> = Vec::with_capacity(3);

        let current_players: Vec<&Player> = club
            .players
            .contracts
            .iter()
            .map(|p| &p.player)
            .filter(|p| *p.position() == PlayerPositionType::Goalkeeper)
            .collect();

        for player in current_players {
            if staff.is_favorite(&player) {
                result.push(SquadPlayer::new(&player, &PlayerPositionType::Goalkeeper))
            }
        }

        result
    }
}
