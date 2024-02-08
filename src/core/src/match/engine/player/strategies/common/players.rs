use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, MatchState};
use nalgebra::Vector3;

pub struct MatchPlayerLogic;

impl MatchPlayerLogic {
    pub fn closest_teammate(
        players_positions: &[PlayerFieldPosition],
        current_player: &MatchPlayer,
        _state: &MatchState,
    ) -> Option<Vector3<f32>> {
        let max_pass_distance = 30.0;

        let mut closest_teammate = None;
        let mut closest_distance = f32::MAX;

        for teammate_player_position in players_positions.iter() {
            if teammate_player_position.player_id == current_player.player_id {
                continue;
            }

            if teammate_player_position.is_home != current_player.is_home {
                continue;
            }

            let distance = current_player
                .position
                .distance_to(&teammate_player_position.position);

            if distance < closest_distance && distance < max_pass_distance {
                closest_teammate = Some(teammate_player_position.position);
                closest_distance = distance;
            }
        }

        closest_teammate
    }

    pub fn closest_opponent<'p>(
        player: &MatchPlayer,
        players_positions: &'p [PlayerFieldPosition],
    ) -> Option<&'p PlayerFieldPosition> {
        players_positions
            .iter()
            .filter(|p| p.is_home != player.is_home && p.player_id != player.player_id) // Consider only opponents and exclude the current player
            .min_by(|a, b| {
                let distance_a_squared = (a.position - player.position).norm_squared();
                let distance_b_squared = (b.position - player.position).norm_squared();

                distance_a_squared
                    .partial_cmp(&distance_b_squared)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub fn find_leader(
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
    ) -> u32 {
        let mut leader_id = 0;
        let mut highest_leadership = 0.0;

        for player_position in &objects_positions.players_positions {
            let player = context.players.get(player_position.player_id).unwrap();
            let leadership_skill = player.skills.mental.leadership;

            if leadership_skill > highest_leadership {
                highest_leadership = leadership_skill;
                leader_id = player_position.player_id;
            }
        }

        leader_id
    }
}
