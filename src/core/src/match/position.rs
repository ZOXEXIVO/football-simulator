use nalgebra::Vector3;
use rand::Rng;
use rand_distr::num_traits::real::Real;
use rand_distr::num_traits::Pow;
use std::collections::HashMap;
use crate::r#match::PlayerSide;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MatchPositionData {
    pub ball_positions: Vec<PositionDataItem>,
    pub player_positions: HashMap<u32, Vec<PositionDataItem>>,
}

impl MatchPositionData {
    pub fn new() -> Self {
        MatchPositionData {
            ball_positions: Vec::new(),
            player_positions: HashMap::with_capacity(22 * 2 * 9000),
        }
    }

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

        if let Some(last_position) = self.ball_positions.last() {
            if last_position != &position {
                self.ball_positions.push(position);
            }
        } else {
            self.ball_positions.push(position);
        }
    }
}

const MAX_NORMALIZED_VALUE: f32 = 0.5f32;

// #[derive(Debug, Copy, Clone)]
// pub struct Vector3<f32> {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }
//
// impl Vector3<f32> {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Vector3<f32> { x, y, z }
//     }
//
//     pub fn length(&self) -> f32 {
//         (self.x.pow(2.0) + self.y.pow(2.0)) as f32
//     }
//
//     pub fn normalize(&self) -> Vector3<f32> {
//         let mut val = *self;
//
//         let len = val.length();
//         if len != 0.0 {
//             val.x /= len;
//             val.y /= len;
//             val.z /= len;
//
//             if len > MAX_NORMALIZED_VALUE {
//                 val.x *= MAX_NORMALIZED_VALUE / len;
//                 val.y *= MAX_NORMALIZED_VALUE / len;
//                 val.z *= MAX_NORMALIZED_VALUE / len;
//             }
//         }
//
//         val
//     }
//
//     fn is_collision(&self, other: &Vector3<f32>) -> bool {
//         const COLLISION_RADIUS: f32 = 2.0;
//
//         let x_diff = (self.x - other.x).abs();
//         let y_diff = (self.y - other.y).abs();
//         let z_diff = (self.z - other.z).abs();
//
//         x_diff <= COLLISION_RADIUS && y_diff <= COLLISION_RADIUS && z_diff <= COLLISION_RADIUS
//     }
//
//     pub fn distance_to(&self, other: &Vector3<f32>) -> f32 {
//         ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
//             .sqrt()
//     }
//
//     pub fn random_in_unit_circle() -> Self {
//         let mut rng = rand::thread_rng();
//
//         let r: f32 = rng.gen_range(0.0..1.0).powf(1.0 / 3.0) as f32;
//         let theta: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
//         let phi: f32 = rng.gen_range(0.0..std::f32::consts::PI);
//
//         Vector3<f32> {
//             x: r * phi.sin() * theta.cos(),
//             y: r * phi.sin() * theta.sin(),
//             z: r * phi.cos(),
//         }
//     }
// }

pub struct PlayerFieldPosition {
    pub player_id: u32,
    pub side: PlayerSide,
    pub position: Vector3<f32>,
}

pub trait VectorExtensions {
    fn length(&self) -> f32;
    fn random_in_unit_circle() -> Vector3<f32>;
    fn distance_to(&self, other: &Vector3<f32>) -> f32;
}

impl VectorExtensions for Vector3<f32> {
    fn length(&self) -> f32 {
        (self.x.pow(2.0) + self.y.pow(2.0) + self.z.pow(2.0)) as f32
    }

    fn random_in_unit_circle() -> Vector3<f32> {
        let mut rng = rand::thread_rng();

        let r: f32 = rng.gen_range(0.0..1.0).powf(1.0 / 3.0) as f32;
        let theta: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        let phi: f32 = rng.gen_range(0.0..std::f32::consts::PI);

        Vector3::new(
            r * phi.sin() * theta.cos(),
            r * phi.sin() * theta.sin(),
            r * phi.cos(),
        )
    }

    fn distance_to(&self, other: &Vector3<f32>) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}
