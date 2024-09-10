use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{MatchField, MatchPlayer};
use nalgebra::Vector3;

pub struct GameTickContext {
    pub objects_positions: MatchObjectsPositions,
}

pub struct PlayerTickContext {
    pub ball: BallContext,
    pub player: PlayerContext
}

pub struct BallContext {
    pub is_heading_towards_player: bool,
    pub on_own_side: bool,

    pub ball_distance: f32,
}

pub struct PlayerContext {
    pub distance_to_start_position: PlayerDistanceFromStartPosition
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PlayerDistanceFromStartPosition {
    Small,
    Medium,
    Big
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
                    distances.add(
                        outer_player.player_id,
                        outer_player.team_id,
                        outer_player.position,
                        inner_player.player_id,
                        inner_player.team_id,
                        inner_player.position,
                        distance,
                    );
                });
            });

        MatchObjectsPositions {
            ball_position: field.ball.position,
            ball_velocity: field.ball.velocity,
            players_positions: positions,
            player_distances: distances,
        }
    }

    pub fn is_big_opponents_concentration(&self, current_player: &MatchPlayer) -> bool {
        let max_distance = 100.0;

        let (nearest_teammates_count, nearest_opponents_count) = self
            .player_distances
            .players_within_distance_count(current_player, max_distance);

        ((nearest_teammates_count as f32) + 1.0) / ((nearest_opponents_count as f32) + 1.0) < 1.0
    }
}

pub struct PlayerDistanceClosure {
    distances: Vec<PlayerDistanceItem>,
}

pub struct PlayerDistanceItem {
    pub player_from_id: u32,
    player_from_team: u32,
    pub player_from_position: Vector3<f32>,

    pub player_to_id: u32,
    player_to_team: u32,
    pub player_to_position: Vector3<f32>,

    distance: f32,
}

impl PartialEq for PlayerDistanceItem {
    fn eq(&self, other: &Self) -> bool {
        self.player_from_id == other.player_from_id
            && self.player_from_team == other.player_from_team
            && self.player_to_id == other.player_to_id
            && self.player_to_team == other.player_to_team
            && self.distance == other.distance
    }
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
        player_from_position: Vector3<f32>,
        player_to_id: u32,
        player_to_team: u32,
        player_to_position: Vector3<f32>,
        distance: f32,
    ) {
        self.distances.push(PlayerDistanceItem {
            player_from_id,
            player_from_team,
            player_from_position,
            player_to_id,
            player_to_team,
            player_to_position,
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

    pub fn players_within_distance(
        &self,
        current_player: &MatchPlayer,
        max_distance: f32,
    ) -> (Vec<(u32, f32)>, Vec<(u32, f32)>) {
        let (teammates, opponents): (Vec<(u32, f32)>, Vec<(u32, f32)>) = self
            .distances
            .iter()
            .filter(|&p| p.distance < max_distance)
            .fold(
                (Vec::with_capacity(10), Vec::with_capacity(10)),
                |(mut teammates, mut opponents), distance| {
                    if distance.player_from_team == current_player.team_id {
                        if distance.player_from_id != current_player.player_id {
                            teammates.push((distance.player_from_id, distance.distance));
                        }
                    } else {
                        if distance.player_to_id != current_player.player_id {
                            opponents.push((distance.player_from_id, distance.distance));
                        }
                    }
                    (teammates, opponents)
                },
            );

        (teammates, opponents)
    }

    pub fn get_collisions(&self, max_distance: f32) -> Vec<&PlayerDistanceItem> {
        self.distances
            .iter()
            .filter(|&p| p.distance < max_distance)
            .collect()
    }

    pub fn players_within_distance_count(
        &self,
        current_player: &MatchPlayer,
        max_distance: f32,
    ) -> (usize, usize) {
        let (teammates_count, opponents_count) = self
            .distances
            .iter()
            .filter(|&p| p.distance < max_distance)
            .fold(
                (0, 0),
                |(mut teammates_count, mut opponents_count), distance| {
                    if distance.player_from_team == current_player.team_id
                        && distance.player_from_id != current_player.player_id
                    {
                        teammates_count += 1;
                    } else if distance.player_to_team == current_player.team_id
                        && distance.player_to_id != current_player.player_id
                    {
                        opponents_count += 1;
                    }
                    (teammates_count, opponents_count)
                },
            );

        (teammates_count, opponents_count)
    }

    pub fn find_closest_opponent(&self, player: &MatchPlayer) -> Option<(u32, f32)> {
        self.distances
            .iter()
            .filter(|distance| {
                distance.player_from_id == player.player_id
                    || distance.player_to_id == player.player_id
            })
            .filter(|distance| distance.player_from_id != player.player_id)
            .filter_map(|distance| {
                let opponent_id = if distance.player_from_id == player.player_id {
                    distance.player_to_id
                } else {
                    distance.player_from_id
                };
                let distance_to_opponent = self.get(player.player_id, opponent_id)?;
                Some((opponent_id, distance_to_opponent))
            })
            .min_by(|&(_, distance1), &(_, distance2)| distance1.partial_cmp(&distance2).unwrap())
    }
}
