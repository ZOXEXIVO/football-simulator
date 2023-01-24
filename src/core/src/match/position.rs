use std::collections::HashMap;
use std::ops::{Add, Deref, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PositionDataItem {
    pub timestamp: u64,
    pub x: i16,
    pub y: i16,
}

impl PositionDataItem {
    pub fn new(timestamp: u64, x: i16, y: i16) -> Self {
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
            player_positions: HashMap::new(),
        }
    }

    pub fn add_player_positions(&mut self, player_id: u32, timestamp: u64, x: i16, y: i16) {
        if let Some(player_data) = self.player_positions.get_mut(&player_id) {
            player_data.push(PositionDataItem::new(timestamp, x, y));
        } else {
            self.player_positions
                .insert(player_id, vec![PositionDataItem::new(timestamp, x, y)]);
        }
    }

    pub fn add_ball_positions(&mut self, timestamp: u64, x: i16, y: i16) {
        self.ball_positions
            .push(PositionDataItem::new(timestamp, x, y));
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FieldPosition {
    pub x: i16,
    pub y: i16,
}

impl FieldPosition {
    pub fn new(x: i16, y: i16) -> Self {
        FieldPosition { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x.pow(2) + self.y.pow(2)) as f32
    }

    pub fn normalize(&self) -> FieldPosition {
        let len = self.length().sqrt();
        if len != 0.0 {
            FieldPosition {
                x: (self.x as f32 / len) as i16,
                y: (self.y as f32 / len) as i16,
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
            x: (self.x as f32 - other) as i16,
            y: (self.y as f32 - other) as i16,
        }
    }
}

impl Sub<i16> for FieldPosition {
    type Output = FieldPosition;

    fn sub(self, other: i16) -> FieldPosition {
        FieldPosition {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Add<f32> for FieldPosition {
    type Output = FieldPosition;

    fn add(self, other: f32) -> FieldPosition {
        FieldPosition {
            x: (self.x as f32 + other) as i16,
            y: (self.y as f32 + other) as i16,
        }
    }
}

impl Mul<f32> for FieldPosition {
    type Output = FieldPosition;

    fn mul(self, other: f32) -> FieldPosition {
        FieldPosition {
            x: (self.x as f32 * other) as i16,
            y: (self.y as f32 * other) as i16,
        }
    }
}

impl PartialEq for FieldPosition {
    fn eq(&self, other: &FieldPosition) -> bool {
        self.x == other.x && self.y == other.y
    }
}
