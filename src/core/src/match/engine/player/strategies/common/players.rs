use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerDistanceFromStartPosition};
use crate::r#match::position::VectorExtensions;

pub struct MatchPlayerLogic;

impl MatchPlayerLogic {
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

    pub fn distance_to_start_position(
        player: &MatchPlayer,
    ) -> PlayerDistanceFromStartPosition {
        let start_position_distance = player.position.distance_to(&player.start_position);

        if start_position_distance < 50.0 {
            PlayerDistanceFromStartPosition::Small
        } else if start_position_distance < 100.0 {
            PlayerDistanceFromStartPosition::Medium
        } else {
            PlayerDistanceFromStartPosition::Big
        }
    }
}
