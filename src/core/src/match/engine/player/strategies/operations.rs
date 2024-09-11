use crate::r#match::{BallState, MatchBallLogic, MatchPlayerLogic, PlayerDistanceFromStartPosition, StateProcessingContext};
use crate::r#match::position::VectorExtensions;

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
        if let Some(ball_state) = self.ctx.context.state.ball_state {
            return self.ctx.player.is_home && ball_state == BallState::HomeSide;
        }

        false
    }

    fn distance(&self) -> f32 {
        self.ctx.tick_context
            .objects_positions
            .ball_position
            .distance_to(&self.ctx.player.position)
    }

    fn speed(&self) -> f32 {
        self.ctx.tick_context.objects_positions.ball_velocity.norm()
    }

    fn is_towards_player(&self) -> bool {
        MatchBallLogic::is_heading_towards_player(&self.ctx.tick_context.objects_positions.ball_position,
                                                  &self.ctx.player.position)
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
    fn on_own_side(&self) -> bool {
        let field_half_width = self.ctx.context.field_size.width / 2;
        self.ctx.player.is_home && self.ctx.player.position.x < field_half_width as f32
    }

    fn position_to_distance(&self) -> PlayerDistanceFromStartPosition {
        MatchPlayerLogic::distance_to_start_position(&self.ctx.player)
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
    fn speed(&self) -> f32;
    fn is_towards_player(&self) -> bool;
}

pub trait PlayerOperations {
    fn on_own_side(&self) -> bool;
    fn position_to_distance(&self) -> PlayerDistanceFromStartPosition;
    fn is_tired(&self) -> bool;
    fn distances(&self) -> (usize, usize);
}
