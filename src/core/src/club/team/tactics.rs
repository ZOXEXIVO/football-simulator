use crate::club::{Staff, PlayerPositionType, PersonBehaviourState};
use std::collections::HashMap;
use crate::Team;

#[derive(Debug)]
pub struct Tactics {
    pub positioning: TacticsPositioning,
}

impl Tactics {
    pub fn new(positioning: TacticsPositioning) -> Self {
        Tactics { positioning }
    }
}

#[derive(Debug)]
pub enum TacticsPositioning {
    T235,
    T442,
    T451,
    T433,
    T442Diamond,
    T442DiamondWide,
    T442Narrow,
    T352,
    T4231,
    T4141,
    T4411,
    T343,
    T1333,
    T4312,
    T4222,
}

pub struct TacticsSelector;

impl TacticsSelector {
    pub fn select(team: &Team, coach: &Staff) -> Tactics {
        match coach.behaviour.state {
            PersonBehaviourState::Poor => Tactics::new(TacticsPositioning::T451),
            PersonBehaviourState::Normal => Tactics::new(Self::team_players(team)),
            PersonBehaviourState::Good => Tactics::new(Self::team_players(team)),
        }
    }

    fn team_players(team: &Team) -> TacticsPositioning {
        let player_stats = Self::players_by_position(team);

        let scores = {
            let mut defending_score: i8 = 0;

            if let Some(defenders) = player_stats.get(&PlayerPositionType::Defender) {
                match defenders {
                    1..=2 => defending_score += 1,
                    3..=6 => defending_score += 2,
                    _ => defending_score += 3,
                }
            }

            let mut midfielder_score: i8 = 0;

            if let Some(midfielders) = player_stats.get(&PlayerPositionType::Midfielder) {
                match midfielders {
                    1..=2 => midfielder_score += 1,
                    3..=6 => midfielder_score += 2,
                    _ => midfielder_score += 3,
                }
            }

            let mut forward_score: i8 = 0;

            if let Some(forwards) = player_stats.get(&PlayerPositionType::Forward) {
                match forwards {
                    1..=2 => forward_score += 1,
                    3..=6 => forward_score += 2,
                    _ => forward_score += 3,
                }
            }

            (defending_score, midfielder_score, forward_score)
        };

        let defending = scores.0;
        let midfielders = scores.1;
        let forwarding = scores.2;

        if defending > midfielders && defending > forwarding {
            return TacticsPositioning::T442;
        }

        if midfielders > defending && midfielders > forwarding {
            return TacticsPositioning::T451;
        }

        if forwarding > defending && forwarding > midfielders {
            return TacticsPositioning::T235;
        }

        TacticsPositioning::T442
    }

    fn players_by_position(team: &Team) -> HashMap<PlayerPositionType, i16> {
        let mut player_positions = HashMap::<PlayerPositionType, i16>::new();

        let team_players = team.players();

        let ready_for_match_players = team_players.iter().filter(|p| p.is_ready_for_match());

        for player in ready_for_match_players {
            let position = player.position();

            let entry = player_positions.entry(position).or_insert(0);

            *entry += 1;
        }

        player_positions
    }
}
