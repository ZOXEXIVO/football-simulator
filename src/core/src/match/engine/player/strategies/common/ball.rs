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
        match self.ctx.tick_context.ball.side {
            BallSide::Left => self.ctx.player.side == Some(PlayerSide::Left),
            BallSide::Right => self.ctx.player.side == Some(PlayerSide::Right),
        }
    }

    pub fn distance(&self) -> f32 {
        self.ctx
            .tick_context
            .object_positions
            .ball_position
            .distance_to(&self.ctx.player.position)
    }

    pub fn speed(&self) -> f32 {
        self.ctx.tick_context.object_positions.ball_velocity.norm()
    }

    pub fn is_owned(&self) -> bool {
        self.ctx.tick_context.ball.is_owned
    }

    pub fn owner_id(&self) -> Option<u32> {
        self.ctx.tick_context.ball.current_owner
    }

    pub fn is_towards_player(&self) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.object_positions.ball_position,
            &self.ctx.tick_context.object_positions.ball_velocity,
            &self.ctx.player.position,
            0.95,
        );
        is_towards
    }

    pub fn is_towards_player_with_angle(&self, angle: f32) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.object_positions.ball_position,
            &self.ctx.tick_context.object_positions.ball_velocity,
            &self.ctx.player.position,
            angle,
        );
        is_towards
    }

    pub fn distance_to_own_goal(&self) -> f32 {
        let target_goal = match self.ctx.player.side {
            Some(PlayerSide::Left) => Vector3::new(
                self.ctx.context.goal_positions.left.x,
                self.ctx.context.goal_positions.left.y,
                0.0,
            ),
            Some(PlayerSide::Right) => Vector3::new(
                self.ctx.context.goal_positions.right.x,
                self.ctx.context.goal_positions.right.y,
                0.0,
            ),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        self.ctx
            .tick_context
            .object_positions
            .ball_position
            .distance_to(&target_goal)
    }

    pub fn direction_to_own_goal(&self) -> Vector3<f32> {
        match self.ctx.player.side {
            Some(PlayerSide::Left) => self.ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => self.ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn direction_to_opponent_goal(&self) -> Vector3<f32> {
        let player = self.ctx.player;
        let ball_position = self.ctx.tick_context.object_positions.ball_position;

        let players = self.ctx.team();
        let goalkeepers = players.goalkeeper_opponents();
        let goalkeeper = goalkeepers.first().unwrap();

        let player_position = player.position;
        let goalkeeper_position = goalkeeper.position;

        // Calculate the angle between the player, ball, and goalkeeper
        let player_to_ball = ball_position - player_position;
        let ball_to_goalkeeper = goalkeeper_position - ball_position;
        let angle = player_to_ball.angle(&ball_to_goalkeeper);

        // Determine the target position based on the angle and player's shooting ability
        let shooting_ability = player.skills.technical.finishing;
        if angle < 0.5 * std::f32::consts::PI {
            // If the angle is small, aim towards the opposite corner of the goal
            let goal_width = self.ctx.context.field_size.width as f32;
            let goal_position = self.ctx.player().opponent_goal_position();
            let target_x = if player_position.x < ball_position.x {
                goal_position.x + goal_width / 2.0
            } else {
                goal_position.x - goal_width / 2.0
            };
            Vector3::new(target_x, goal_position.y, 0.0)
        } else {
            // If the angle is large, aim towards the goal center with some randomness
            let goal_position = self.ctx.player().opponent_goal_position();
            let random_offset = (rand::random::<f32>() - 0.5) * shooting_ability;
            Vector3::new(goal_position.x + random_offset, goal_position.y, 0.0)
        }
    }

    pub fn distance_to_opponent_goal(&self) -> f32 {
        let target_goal = match self.ctx.player.side {
            Some(PlayerSide::Left) => Vector3::new(
                self.ctx.context.goal_positions.right.x,
                self.ctx.context.goal_positions.right.y,
                0.0,
            ),
            Some(PlayerSide::Right) => Vector3::new(
                self.ctx.context.goal_positions.left.x,
                self.ctx.context.goal_positions.left.y,
                0.0,
            ),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        self.ctx
            .tick_context
            .object_positions
            .ball_position
            .distance_to(&target_goal)
    }

    pub fn on_own_third(&self) -> bool {
        let field_length = self.ctx.context.field_size.width as f32;
        let ball_x = self.ctx.tick_context.object_positions.ball_position.x;

        if self.ctx.player.side == Some(PlayerSide::Left) {
            // Home team defends the left side (negative X)
            ball_x < -field_length / 3.0
        } else {
            // Away team defends the right side (positive X)
            ball_x > field_length / 3.0
        }
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
