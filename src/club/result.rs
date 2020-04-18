use crate::people::{PlayerResult, StaffResult};
use crate::club::BoardResult;

pub struct ClubResult {
    pub board: BoardResult,
    pub player: PlayerResult,
    pub staff: StaffResult
}

impl ClubResult {
    pub fn new(board: BoardResult, player: PlayerResult, staff: StaffResult) -> Self {
        ClubResult {
            board,
            player,
            staff
        }
    }
}