use crate::PlayerPositionType;
use crate::{Behaviour, BehaviourState, Club, Staff};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tactics {
    pub positioning: TacticsPositioning,
}

impl Tactics {
    pub fn new(positioning: TacticsPositioning) -> Self {
        Tactics { positioning }
    }
}

#[derive(Debug, Clone)]
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
    pub fn select(club: &Club, staff: &Staff) -> Tactics {
        match staff.behaviour.state {
            BehaviourState::Poor => Tactics::new(TacticsPositioning::T451),
            BehaviourState::Normal => Tactics::new(club_players),
            BehaviourState::Good => Tactics::new(club_players),
        }
    }

    fn club_players(club: &Club) -> TacticsPositioning {
        let player_stats = Self::players_by_position(club);

        let scores = {
            let mut defending_score: i8 = 0;

            match player_stats.get(&PlayerPositionType::Defender) {
                Some(defenders) => match defenders {
                    1..=2 => defending_score += 1,
                    3..=6 => defending_score += 2,
                    _ => defending_score += 3,
                },
                None => {}
            }

            let mut midfielder_score: i8 = 0;

            match player_stats.get(&PlayerPositionType::Midfielder) {
                Some(midfielders) => match defenders {
                    1..=2 => midfielder_score += 1,
                    3..=6 => midfielder_score += 2,
                    _ => midfielder_score += 3,
                },
                None => {}
            }

            let mut forward_score: i8 = 0;

            match player_stats.get(&PlayerPositionType::Forward) {
                Some(forwards) => match defenders {
                    1..=2 => forward_score += 1,
                    3..=6 => forward_score += 2,
                    _ => forward_score += 3,
                },
                None => {}
            }

            (defending_score, midfielder_score, forward_score)
        };

        let defending = score.0;
        let midfielders = score.1;
        let forwarding = score.2;

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

    fn players_by_position(club: &Club) -> HashMap<&PlayerPositionType, i16> {
        let mut player_positions = HashMap::new();

        let ready_for_match_players = club.players().iter().filter(|p| p.is_ready_for_match());

        for player in ready_for_match_players {
            let position = player.position();

            match player_positions.get_mut(&position) {
                Some(&mut val) => {
                    *val += 1;
                }
                None => {
                    player_positions.insert(position, 0i16);
                }
            }
        }

        player_positions
    }
}
