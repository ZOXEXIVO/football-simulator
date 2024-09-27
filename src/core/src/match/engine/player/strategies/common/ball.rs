use crate::r#match::position::VectorExtensions;
use crate::r#match::{BallSide, PlayerSide, StateProcessingContext};
use nalgebra::Vector3;

pub struct BallOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> BallOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        BallOperationsImpl { ctx }
    }
}

impl<'b> BallOperationsImpl<'b> {
    pub fn on_own_side(&self) -> bool {
        match self.ctx.context.ball.side() {
            BallSide::Left => self.ctx.player.side == Some(PlayerSide::Left),
            BallSide::Center => true,
            BallSide::Right => self.ctx.player.side == Some(PlayerSide::Right),
        }
    }

    pub fn distance(&self) -> f32 {
        self.ctx
            .tick_context
            .objects_positions
            .ball_position
            .distance_to(&self.ctx.player.position)
    }

    pub fn speed(&self) -> f32 {
        self.ctx.tick_context.objects_positions.ball_velocity.norm()
    }

    pub fn is_towards_player(&self) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.objects_positions.ball_position,
            &self.ctx.tick_context.objects_positions.ball_velocity,
            &self.ctx.player.position,
            0.95
        );
        is_towards
    }

    pub fn is_towards_player_with_angle(&self, angle: f32) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.objects_positions.ball_position,
            &self.ctx.tick_context.objects_positions.ball_velocity,
            &self.ctx.player.position,
            angle
        );
        is_towards
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

        self.ctx
            .tick_context
            .objects_positions
            .ball_position
            .distance_to(&own_goal_position)
    }
}

pub struct MatchBallLogic;

impl MatchBallLogic {
    pub fn is_heading_towards_player(
        ball_position: &Vector3<f32>,
        ball_velocity: &Vector3<f32>,
        player_position: &Vector3<f32>,
        angle: f32,
    ) -> (bool, f32) {
        Self::is_heading_towards_player_witj_angle(
            ball_position,
            ball_velocity,
            player_position,
            angle,
        )
    }

    pub fn is_heading_towards_player_witj_angle(
        ball_position: &Vector3<f32>,
        ball_velocity: &Vector3<f32>,
        player_position: &Vector3<f32>,
        angle: f32,
    ) -> (bool, f32) {
        let velocity_xy = Vector3::new(ball_velocity.x, ball_velocity.y, 0.0);
        let ball_to_player_xy = Vector3::new(
            player_position.x - ball_position.x,
            player_position.y - ball_position.y,
            0.0,
        );

        let velocity_norm = velocity_xy.norm();
        let direction_norm = ball_to_player_xy.norm();

        let normalized_velocity = velocity_xy / velocity_norm;
        let normalized_direction = ball_to_player_xy / direction_norm;
        let dot_product = normalized_velocity.dot(&normalized_direction);

        (dot_product >= angle, dot_product)
    }
}
