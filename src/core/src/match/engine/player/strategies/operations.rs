use crate::r#match::{PlayerDistanceFromStartPosition, StateProcessingContext};

pub trait StateProcessingOperations {
    fn ball(&self) -> impl BallOperations;
    fn player(&self) -> impl PlayerOperations;
}

impl<'ct> StateProcessingOperations for StateProcessingContext<'ct> {
    fn ball(&self) -> impl BallOperations {
        BallOperationsImpl::new(self)
    }

    fn player(&self) -> impl PlayerOperations {
        PlayerOperationsImpl::new(self)
    }
}

pub struct BallOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl <'b> BallOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        BallOperationsImpl { ctx }
    }
}

impl<'b> BallOperations for BallOperationsImpl<'b> {
    fn on_own_side(&self) -> bool {
        self.ctx.player_context.ball.on_own_side
    }

    fn distance(&self) -> f32 {
        self.ctx.player_context.ball.ball_distance
    }

    fn is_towards_player(&self) -> bool {
        self.ctx.player_context.ball.is_heading_towards_player
    }
}

pub struct PlayerOperationsImpl<'p> {
    ctx: &'p StateProcessingContext<'p>,
}

impl <'p> PlayerOperationsImpl<'p> {
    pub fn new(ctx: &'p StateProcessingContext<'p>) -> Self {
        PlayerOperationsImpl { ctx }
    }
}

impl<'p> PlayerOperations for PlayerOperationsImpl<'p> {
    fn position_to_distance(&self) -> PlayerDistanceFromStartPosition {
        self.ctx.player_context.player.distance_to_start_position
    }

    fn is_tired(&self) -> bool {
        self.ctx.player.player_attributes.condition_percentage() > 50
    }

    fn distances(&self) -> (usize, usize) {
        self.ctx.tick_context
            .objects_positions
            .player_distances
            .players_within_distance_count(self.ctx.player, 10.0)
    }
}

pub trait BallOperations {
    fn on_own_side(&self) -> bool;
    fn distance(&self) -> f32;
    fn is_towards_player(&self) -> bool;
}

pub trait PlayerOperations {
    fn position_to_distance(&self) -> PlayerDistanceFromStartPosition;
    fn is_tired(&self) -> bool;
    fn distances(&self) -> (usize, usize);
}
