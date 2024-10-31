use crate::r#match::PlayerSide;
use nalgebra::Vector3;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct PositionDataItem {
    pub timestamp: u64,
    pub position: Vector3<f32>,
}

impl PositionDataItem {
    pub fn new(timestamp: u64, position: Vector3<f32>) -> Self {
        PositionDataItem {
            timestamp,
            position,
        }
    }
}

impl PartialEq<PositionDataItem> for PositionDataItem {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.position == other.position
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchPositionData {
    ball_positions: Vec<PositionDataItem>,
    player_positions: HashMap<u32, Vec<PositionDataItem>>,
}

impl MatchPositionData {
    pub fn new() -> Self {
        MatchPositionData {
            ball_positions: Vec::new(),
            player_positions: HashMap::with_capacity(22 * 2 * 9000),
        }
    }

    pub fn compress(&mut self) {}

    pub fn add_player_positions(&mut self, player_id: u32, timestamp: u64, position: Vector3<f32>) {
        if let Some(player_data) = self.player_positions.get_mut(&player_id) {
            let last_data = player_data.last().unwrap();
            if last_data.position.x != position.x
                || last_data.position.y != position.y
                || last_data.position.z != position.z
            {
                let position_data = PositionDataItem::new(timestamp, position);
                player_data.push(position_data);
            }
        } else {
            self.player_positions
                .insert(player_id, vec![PositionDataItem::new(timestamp, position)]);
        }
    }

    pub fn add_ball_positions(&mut self, timestamp: u64, position: Vector3<f32>) {
        let position = PositionDataItem::new(timestamp, position);

        if let Some(last_position) = self.ball.positions.last() {
            if last_position != &position {
                self.ball.positions.push(position);
            }
        } else {
            self.ball.positions.push(position);
        }
    }
}

const MAX_NORMALIZED_VALUE: f32 = 0.5f32;

pub struct PlayerFieldMetadata {
    pub player_id: u32,
    pub side: PlayerSide,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

pub trait VectorExtensions {
    fn length(&self) -> f32;
    fn random_in_unit_circle() -> Vector3<f32>;
    fn distance_to(&self, other: &Vector3<f32>) -> f32;
}

impl VectorExtensions for Vector3<f32> {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn random_in_unit_circle() -> Vector3<f32> {
        let mut rng = rand::thread_rng();

        let u = rng.gen::<f32>();
        let v = rng.gen::<f32>();

        let phi = std::f32::consts::PI * 2.0 * u;
        let costheta = (1.0 - 2.0 * v).sqrt();
        let sintheta = (1.0 - costheta * costheta).sqrt();

        Vector3::new(sintheta * phi.cos(), sintheta * phi.sin(), costheta)
    }

    fn distance_to(&self, other: &Vector3<f32>) -> f32 {
        let diff = self - other;
        diff.dot(&diff).sqrt()
    }
}
