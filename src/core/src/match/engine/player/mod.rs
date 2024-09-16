pub mod behaviours;
mod conditions;
pub mod context;
pub mod events;
pub mod player;
mod state;
pub mod statistics;
pub mod strategies;

pub use behaviours::*;
pub use context::*;
use itertools::Itertools;
pub use player::*;
pub use strategies::*;

pub struct GameFieldContextInput<'p> {
    objects_positions: &'p MatchObjectsPositions,
}

impl<'p> GameFieldContextInput<'p> {
    pub fn from_contexts(ctx: &StateProcessingContext<'p>) -> Self {
        GameFieldContextInput {
            objects_positions: &ctx.tick_context.objects_positions,
        }
    }

    pub fn to_input(&self) -> Vec<f64> {
        let players_positions: Vec<f64> = self
            .objects_positions
            .players_positions
            .items
            .iter()
            .sorted_by_key(|m| m.player_id)
            .flat_map(|p| p.position.as_slice().to_vec())
            .map(|m| m as f64)
            .collect();

        players_positions
    }
}
