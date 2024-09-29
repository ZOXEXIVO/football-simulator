use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{BallSide, MatchField, MatchPlayer};
use nalgebra::Vector3;

pub struct GameTickContext {
    pub object_positions: MatchObjectsPositions,
    pub ball: BallMetadata
}

impl GameTickContext {
    pub fn new(field: &MatchField) -> Self {
        GameTickContext {
            ball: BallMetadata::from_field(field),
            object_positions: MatchObjectsPositions::from(field)
        }
    }
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
    pub players_positions: PlayerPositionsClosure,
    pub player_distances: PlayerDistanceClosure,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        let positions: Vec<PlayerFieldPosition> = field
            .players
            .iter()
            .map(|p| PlayerFieldPosition {
                player_id: p.id,
                side: p.side.unwrap(),
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
                        outer_player.id,
                        outer_player.team_id,
                        outer_player.position,
                        inner_player.id,
                        inner_player.team_id,
                        inner_player.position,
                        distance,
                    );
                });
            });

        MatchObjectsPositions {
            ball_position: field.ball.position,
            ball_velocity: field.ball.velocity,
            players_positions: PlayerPositionsClosure::new(positions),
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

pub struct BallMetadata {
    pub side: BallSide,
    pub is_owned: bool,
    pub last_owner: Option<u32>
}

impl BallMetadata {
    pub fn from_field(field: &MatchField) -> Self {
        BallMetadata {
            side: Self::calculate_side(field),
            is_owned: field.ball.owned,
            last_owner: field.ball.last_owner
        }
    }

    fn calculate_side(field: &MatchField) -> BallSide {
        if field.ball.position.x < field.ball.center_field_position {
            return BallSide::Left;
        }

        BallSide::Right
    }
}

pub struct PlayerPositionsClosure {
    pub items: Vec<PlayerFieldPosition>,
}

impl PlayerPositionsClosure {
    pub fn new(players_positions: Vec<PlayerFieldPosition>) -> Self {
        PlayerPositionsClosure {
            items: players_positions
        }
    }

    pub fn get_player_position(&self, player_id: u32) -> Option<Vector3<f32>> {
        self.items
            .iter()
            .find(|p| p.player_id == player_id)
            .map(|p| p.position)
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

    pub fn players_within_distance(&self, current_player: &MatchPlayer, max_distance: f32) -> (Vec<(u32, f32)>, Vec<(u32, f32)>) {
        self.distances.iter()
            .filter(|item| item.player_from_id == current_player.id && item.distance < max_distance)
            .fold((Vec::new(), Vec::new()), |(mut teammates, mut opponents), item| {
                if item.player_to_team == current_player.team_id {
                    teammates.push((item.player_to_id, item.distance));
                } else {
                    opponents.push((item.player_to_id, item.distance));
                }
                (teammates, opponents)
            })
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
                        && distance.player_from_id != current_player.id
                    {
                        teammates_count += 1;
                    } else if distance.player_to_team == current_player.team_id
                        && distance.player_to_id != current_player.id
                    {
                        opponents_count += 1;
                    }
                    (teammates_count, opponents_count)
                },
            );

        (teammates_count, opponents_count)
    }

    pub fn find_closest_opponent(&self, player: &MatchPlayer) -> Option<(u32, f32)> {
        self.distances.iter()
            .filter(|item| item.player_from_id == player.id && item.player_from_team != item.player_to_team)
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|item| (item.player_to_id, item.distance))
    }

    pub fn find_closest_opponents(&self, player: &MatchPlayer) -> Option<Vec<(u32, f32)>> {
        let mut opponents: Vec<_> = self.distances.iter()
            .filter(|item| item.player_from_id == player.id && item.player_from_team != item.player_to_team)
            .map(|item| (item.player_to_id, item.distance))
            .collect();
        opponents.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if opponents.is_empty() { None } else { Some(opponents) }
    }

    pub fn find_closest_teammates(&self, player: &MatchPlayer) -> Option<Vec<(u32, f32)>> {
        let mut teammates: Vec<_> = self.distances.iter()
            .filter(|item| item.player_from_id == player.id && item.player_from_team == item.player_to_team)
            .map(|item| (item.player_to_id, item.distance))
            .collect();
        teammates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        if teammates.is_empty() { None } else { Some(teammates) }
    }

    pub fn find_closest_to_ball(&self, ball_position: Vector3<f32>) -> Option<(u32, f32)> {
        self.distances.iter()
            .map(|item| {
                let dist_from = (item.player_from_position - ball_position).length();
                let dist_to = (item.player_to_position - ball_position).length();
                if dist_from < dist_to {
                    (item.player_from_id, dist_from)
                } else {
                    (item.player_to_id, dist_to)
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}
