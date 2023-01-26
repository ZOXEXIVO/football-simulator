use nalgebra::Vector2;
use rand_distr::num_traits::Pow;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

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
            player_data.push(PositionDataItem::new(timestamp, x, y));
        } else {
            self.player_positions
                .insert(player_id, vec![PositionDataItem::new(timestamp, x, y)]);
        }
    }

    pub fn add_ball_positions(&mut self, timestamp: u64, x: f32, y: f32) {
        self.ball_positions
            .push(PositionDataItem::new(timestamp, x, y));
    }
}

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
        let len = self.length().sqrt();
        if len != 0.0 {
            FieldPosition {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
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
