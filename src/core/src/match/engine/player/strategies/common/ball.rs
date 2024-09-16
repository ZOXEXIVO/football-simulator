use nalgebra::Vector3;
use crate::r#match::{BallSide, PlayerSide, StateProcessingContext};
use crate::r#match::position::VectorExtensions;

pub struct BallOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl <'b> BallOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        BallOperationsImpl { ctx }
    }
}

impl<'b> BallOperationsImpl<'b> {
    pub fn on_own_side(&self) -> bool {
        match self.ctx.context.ball.get() {
            BallSide::Left => self.ctx.player.side == Some(PlayerSide::Left),
            BallSide::Center => true,
            BallSide::Right => self.ctx.player.side == Some(PlayerSide::Right),
        }
    }

    pub fn distance(&self) -> f32 {
        self.ctx.tick_context
            .objects_positions
            .ball_position
            .distance_to(&self.ctx.player.position)
    }

    pub fn speed(&self) -> f32 {
        self.ctx.tick_context.objects_positions.ball_velocity.norm()
    }

    pub fn is_towards_player(&self) -> bool {
        MatchBallLogic::is_heading_towards_player(&self.ctx.tick_context.objects_positions.ball_position,
                                                  &self.ctx.player.position)
    }

    pub fn distance_to_own_goal(&self) -> f32 {
        let own_goal_position = if self.ctx.player.side.unwrap() == PlayerSide::Left {
            Vector3::new(0.0, self.ctx.context.field_size.height as f32 / 2.0, 0.0)
        } else {
            Vector3::new(
                self.ctx.context.field_size.width as f32,
                self.ctx.context.field_size.height as f32 / 2.0f32,
                0.0,
            )
        };

        self.ctx.tick_context
            .objects_positions
            .ball_position
            .distance_to(&own_goal_position)
    }
}

pub struct MatchBallLogic;

impl MatchBallLogic {
    pub fn is_heading_towards_player(ball_position: &Vector3<f32>, player_position: &Vector3<f32>) -> bool {
        let ball_to_goal = player_position - ball_position;

        let ball_forward = Vector3::new(1.0, 0.0, 0.0);

        let dot_product = ball_to_goal.normalize().dot(&ball_forward);

        dot_product > 0.8
    }
}
