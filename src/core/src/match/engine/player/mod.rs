pub mod behaviours;
pub mod context;
pub mod player;
pub mod strategies;
mod conditions;
pub mod events;
mod state;
pub mod statistics;

use crate::r#match::MatchContext;
pub use behaviours::*;
pub use context::*;
use itertools::Itertools;
pub use player::*;
pub use strategies::*;

pub struct GameFieldContextInput<'p> {
    objects_positions: &'p MatchObjectsPositions,
}

impl<'p> GameFieldContextInput<'p> {
    pub fn from_contexts(
        context: &MatchContext,
        player: &MatchPlayer,
        tick_context: &'p GameTickContext,
    ) -> Self {
        GameFieldContextInput {
            objects_positions: &tick_context.objects_positions,
        }
    }

    pub fn to_input(&self) -> Vec<f64> {
        let players_positions: Vec<f64> = self
            .objects_positions
            .players_positions
            .iter()
            .sorted_by_key(|m| m.player_id)
            .flat_map(|p| p.position.as_slice().to_vec())
            .map(|m| m as f64)
            .collect();

        players_positions
    }
}
