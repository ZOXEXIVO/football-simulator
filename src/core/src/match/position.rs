use std::collections::HashMap;

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
}
