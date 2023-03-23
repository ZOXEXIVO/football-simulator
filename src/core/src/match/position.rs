use nalgebra::Vector2;
use rand::Rng;
use rand_distr::num_traits::Pow;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PositionDataItem {
    pub timestamp: u64,
    pub x: f32,
    pub y: f32,
}

impl PositionDataItem {
    pub fn new(timestamp: u64, x: f32, y: f32) -> Self {
        PositionDataItem { timestamp, x, y }
    }
}

impl PartialEq<PositionDataItem> for PositionDataItem {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.x == other.x && self.y == other.y
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

    pub fn add_player_positions(&mut self, player_id: u32, timestamp: u64, x: f32, y: f32) {
        if let Some(player_data) = self.player_positions.get_mut(&player_id) {
            let last_data = player_data.last().unwrap();
            let position_data = PositionDataItem::new(timestamp, x, y);
            if *last_data != position_data {
                player_data.push(position_data);
            }
        } else {
            self.player_positions
                .insert(player_id, vec![PositionDataItem::new(timestamp, x, y)]);
        }
    }

    pub fn add_ball_positions(&mut self, timestamp: u64, x: f32, y: f32) {
        let position = PositionDataItem::new(timestamp, x, y);

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

#[derive(Debug, Copy, Clone)]
pub struct FieldPosition {
    pub x: f32,
    pub y: f32,
}

impl FieldPosition {
    pub fn new(x: f32, y: f32) -> Self {
        FieldPosition { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x.pow(2.0) + self.y.pow(2.0)) as f32
    }

    pub fn normalize(&self) -> FieldPosition {
        let mut val = *self;

        let len = val.length();
        if len != 0.0 {
            val.x /= len;
            val.y /= len;

            if len > MAX_NORMALIZED_VALUE {
                val.x *= MAX_NORMALIZED_VALUE / len;
                val.y *= MAX_NORMALIZED_VALUE / len;
            }
        }

        val
    }

    fn is_collision(&self, other: &FieldPosition) -> bool {
        const COLLISION_RADIUS: f32 = 2.0;

        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();

        x_diff <= COLLISION_RADIUS && y_diff <= COLLISION_RADIUS
    }

    pub fn distance_to(&self, other: &FieldPosition) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn random_in_unit_circle() -> Self {
        let mut rng = rand::thread_rng();

        let r: f32 = rng.gen_range(0.0..1.0);
        let theta: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        FieldPosition {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }
}

impl Sub for FieldPosition {
    type Output = FieldPosition;

    fn sub(self, other: FieldPosition) -> FieldPosition {
        FieldPosition {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for FieldPosition {
    type Output = FieldPosition;

    fn sub(self, other: f32) -> FieldPosition {
        FieldPosition {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Sub<Vector2<f32>> for FieldPosition {
    type Output = FieldPosition;

    fn sub(self, other: Vector2<f32>) -> FieldPosition {
        FieldPosition {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<f32> for FieldPosition {
    type Output = FieldPosition;

    fn add(self, other: f32) -> FieldPosition {
        FieldPosition {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add<FieldPosition> for FieldPosition {
    type Output = FieldPosition;

    fn add(self, other: FieldPosition) -> FieldPosition {
        FieldPosition {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<f32> for FieldPosition {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Add<Vector2<f32>> for FieldPosition {
    type Output = FieldPosition;

    fn add(self, other: Vector2<f32>) -> FieldPosition {
        FieldPosition {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for FieldPosition {
    type Output = FieldPosition;

    fn mul(self, other: f32) -> FieldPosition {
        FieldPosition {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl PartialEq for FieldPosition {
    fn eq(&self, other: &FieldPosition) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct PlayerFieldPosition {
    pub player_id: u32,
    pub is_home: bool,
    pub position: FieldPosition,
}
