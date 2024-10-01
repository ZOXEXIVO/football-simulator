use crate::r#match::ball::events::BallUpdateEvent;
use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer};
use crate::Player;
use nalgebra::Vector3;
use rand_distr::num_traits::Pow;

pub struct Ball {
    pub start_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub ball_position: BallPosition,
    pub center_field_position: f32,
    pub height: f32,

    pub last_owner: Option<u32>,
    pub owned: bool,
}

impl Ball {
    pub fn with_coord(x: f32, y: f32) -> Self {
        Ball {
            position: Vector3::new(x, y, 0.0),
            start_position: Vector3::new(x, y, 0.0),
            velocity: Vector3::zeros(),
            ball_position: BallPosition::Home,
            center_field_position: x, // initial ball position = center field
            height: 0.0,
            last_owner: None,
            owned: false,
        }
    }

    pub fn update(
        &mut self,
        context: &MatchContext,
        tick_context: &GameTickContext,
    ) -> Vec<BallUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_velocity(&mut result);
        self.move_to(context);
        self.check_goal(context, &mut result);
        self.check_boundary_collision(&mut result, context);
        self.check_ball_ownership(context, tick_context, &mut result);

        result
    }

    fn check_boundary_collision(
        &mut self,
        _result: &mut Vec<BallUpdateEvent>,
        context: &MatchContext,
    ) {
        let field_width = context.field_size.width as f32 + 1.0;
        let field_height = context.field_size.height as f32 + 1.0;

        // Check if ball hits the boundary and reverse its velocity if it does
        if self.position.x <= 0.0 || self.position.x >= field_width {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y <= 0.0 || self.position.y >= field_height {
            self.velocity.y = -self.velocity.y;
        }
    }

    fn check_ball_ownership(
        &mut self,
        context: &MatchContext,
        tick_context: &GameTickContext,
        result: &mut Vec<BallUpdateEvent>,
    ) {
        const BALL_DISTANCE_THRESHOLD: f32 = 5.0;

        if let Some(owner_player_id) = self.last_owner {
            let owner = context.players.get(owner_player_id).unwrap();
            if owner.position.distance_to(&self.position) > BALL_DISTANCE_THRESHOLD {
                self.last_owner = None;
            }
        } else {
            let nearby_players: Vec<(u32, &PlayerFieldPosition)> = tick_context
                .object_positions
                .players_positions
                .items
                .iter()
                .filter(|player_position| {
                    let dx = player_position.position.x - self.position.x;
                    let dy = player_position.position.y - self.position.y;

                    dx * dx + dy * dy < BALL_DISTANCE_THRESHOLD * BALL_DISTANCE_THRESHOLD
                })
                .map(|player_position| (player_position.player_id, player_position))
                .collect();

            let best_tackler =
                nearby_players
                    .iter()
                    .max_by(|(player_id_a, _), (player_id_b, _)| {
                        let player_a = context.players.get(*player_id_a).unwrap();
                        let player_b = context.players.get(*player_id_b).unwrap();

                        let tackling_score_a = Self::calculate_tackling_score(player_a);
                        let tackling_score_b = Self::calculate_tackling_score(player_b);

                        tackling_score_a
                            .partial_cmp(&tackling_score_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

            if let Some((player_id, _)) = best_tackler {
                self.last_owner = Some(*player_id);
                self.owned = true;

                result.push(BallUpdateEvent::Claimed(*player_id));
            }
        }
    }

    fn calculate_tackling_score(player: &MatchPlayer) -> f32 {
        let technical_skills = &player.skills.technical;
        let mental_skills = &player.skills.mental;
        let physical_skills = &player.skills.physical;

        let tackling_weight = 0.4;
        let aggression_weight = 0.2;
        let bravery_weight = 0.1;
        let strength_weight = 0.2;
        let agility_weight = 0.1;

        technical_skills.tackling * tackling_weight
            + mental_skills.aggression * aggression_weight
            + mental_skills.bravery * bravery_weight
            + physical_skills.strength * strength_weight
            + physical_skills.agility * agility_weight
    }

    fn check_goal(&mut self, context: &MatchContext, result: &mut Vec<BallUpdateEvent>) {
        if let Some(goal_side) = context.goal_positions.is_goal(self.position) {
            result.push(BallUpdateEvent::Goal(goal_side, self.last_owner));
            self.reset();
        }
    }

    fn update_velocity(&mut self, _result: &mut Vec<BallUpdateEvent>) {
        let gravity = Vector3::new(0.0, 0.0, -9.81);

        const FRICTION_COEFFICIENT: f32 = 1.5;
        const BALL_MASS: f32 = 0.43;
        const STOPPING_THRESHOLD: f32 = 0.01;

        let velocity_norm = self.velocity.norm();
        let friction = if velocity_norm > 0.0 {
            -self.velocity.normalize() * FRICTION_COEFFICIENT
        } else {
            Vector3::zeros()
        };

        let total_force = gravity * BALL_MASS - friction;
        let acceleration = total_force / BALL_MASS;

        const TIME_STEP: f32 = 0.01;

        self.velocity += acceleration * TIME_STEP;

        if self.velocity.norm() < STOPPING_THRESHOLD {
            self.velocity = Vector3::zeros();
        }
    }

    fn move_to(&mut self, context: &MatchContext) {
        if self.owned {
            if let Some(owner_player_id) = self.last_owner {
                if let Some(owner) = context.players.get(owner_player_id) {
                    self.position.x += owner.position.x;
                    self.position.y += owner.position.y;
                }
            }
        } else {
            self.position.x += self.velocity.x;
            self.position.y += self.velocity.y;
        }
    }

    fn position(&self) -> BallPosition {
        if self.position.x <= self.center_field_position {
            BallPosition::Home
        } else {
            BallPosition::Away
        }
    }

    pub fn move_towards_player(&mut self, player_pos: &Vector3<f32>) {
        let position_diff = *player_pos - self.position;

        let distance = (position_diff.x.pow(2.0) + position_diff.y.pow(2.0)).sqrt();

        self.position.x += (position_diff.x / distance) * self.velocity.x;
        self.position.y += (position_diff.y / distance) * self.velocity.y;
    }

    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;

        self.velocity = Vector3::zeros();
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BallPosition {
    Home,
    Away,
}
