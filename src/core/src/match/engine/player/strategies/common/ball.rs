use nalgebra::Vector3;
use crate::r#match::{BallState, StateProcessingContext};
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
        if let Some(ball_state) = self.ctx.context.state.ball_state {
            return self.ctx.player.is_home && ball_state == BallState::HomeSide;
        }

        false
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
