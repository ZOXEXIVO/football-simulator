use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{MatchField, MatchPlayer, MatchState};
use itertools::Itertools;
use nalgebra::Vector3;
use std::collections::HashMap;

pub struct GameTickContext {
    pub objects_positions: MatchObjectsPositions,
}

pub struct PlayerTickContext {
    pub ball_context: BallContext,
}

pub struct MatchObjectsPositions {
    pub ball_position: Vector3<f32>,
    pub ball_velocity: Vector3<f32>,
    pub players_positions: Vec<PlayerFieldPosition>,
    pub player_distances: PlayerDistanceClosure,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        let positions: Vec<PlayerFieldPosition> = field
            .players
            .iter()
            .map(|p| PlayerFieldPosition {
                player_id: p.player_id,
                is_home: p.is_home,
                position: p.position,
            })
            .collect();

        // fill distances

        let mut distances = PlayerDistanceClosure::new();

        field
            .players
            .iter()
            .enumerate()
            .for_each(|(i, outer_player)| {
                field.players.iter().skip(i + 1).for_each(|inner_player| {
                    let distance = outer_player.position.distance_to(&inner_player.position);
                    distances.add(outer_player.player_id, outer_player.team_id, inner_player.player_id, inner_player.team_id, distance);
                });
            });

        MatchObjectsPositions {
            ball_position: field.ball.position,
            ball_velocity: field.ball.velocity,
            players_positions: positions,
            player_distances: distances,
        }
    }
}

pub struct PlayerDistanceClosure {
    distances: Vec<PlayerDistanceItem>,
}

pub struct PlayerDistanceItem {
    player_from_id: u32,
    player_from_team: u32,

    player_to_id: u32,
    player_to_team: u32,

    distance: f32,
}

impl PlayerDistanceClosure {
    pub fn new() -> Self {
        PlayerDistanceClosure {
            distances: Vec::with_capacity(50),
        }
    }

    pub fn add(
        &mut self,
        player_from_id: u32,
        player_from_team: u32,
        player_to_id: u32,
        player_to_team: u32,
        distance: f32,
    ) {
        self.distances.push(PlayerDistanceItem {
            player_from_id,
            player_from_team,
            player_to_id,
            player_to_team,
            distance,
        })
    }

    pub fn get(&self, player_from_id: u32, player_to_id: u32) -> Option<f32> {
        self.distances
            .iter()
            .find(|distance| {
                (distance.player_from_id == player_from_id && distance.player_to_id == player_to_id)
                    || (distance.player_from_id == player_to_id
                        && distance.player_to_id == player_from_id)
            })
            .map(|dist| dist.distance)
    }

    pub fn closest_teammate(
        &self,
        positions: &MatchObjectsPositions,
        current_player: &MatchPlayer,
        max_distance: f32
    ) -> Option<Vec<u32>> {
        let filtered_players: Vec<&PlayerDistanceItem> = positions.player_distances.distances.iter()
            .filter(|&p| p.player_from_team == current_player.team_id)
            .filter(|&p| p.distance < max_distance)
            .collect();

        let forward_players: Vec<u32> = filtered_players
            .iter()
            .filter(|distance| distance.player_from_id == current_player.player_id)
            .map(|d| d.player_to_id)
            .collect();

        let backward_players: Vec<u32> = filtered_players
            .iter()
            .filter(|distance| distance.player_from_id == current_player.player_id)
            .map(|d| d.player_to_id)
            .collect();

        Some([forward_players, backward_players].concat())
    }
}

pub struct BallContext {
    pub is_ball_heading_towards_goal: bool,
    pub ball_is_on_player_home_side: bool,

    pub ball_distance: f32,
}
