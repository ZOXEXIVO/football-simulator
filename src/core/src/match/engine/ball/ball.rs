use crate::r#match::ball::events::BallEvent;
use crate::r#match::events::EventCollection;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer};
use nalgebra::Vector3;

pub struct Ball {
    pub start_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub center_field_position: f32,
    pub height: f32,

    pub previous_owner: Option<u32>,
    pub current_owner: Option<u32>,
}

impl Ball {
    pub fn with_coord(x: f32, y: f32) -> Self {
        Ball {
            position: Vector3::new(x, y, 0.0),
            start_position: Vector3::new(x, y, 0.0),
            velocity: Vector3::zeros(),
            center_field_position: x, // initial ball position = center field
            height: 0.0,
            previous_owner: None,
            current_owner: None,
        }
    }

    pub fn update(
        &mut self,
        context: &MatchContext,
        players: &[MatchPlayer],
        tick_context: &GameTickContext,
        events: &mut EventCollection
    ) {
        self.update_velocity();
        self.check_goal(context, events);
        self.check_boundary_collision(context);
        self.check_ball_ownership(context, players, events);

        self.move_to(tick_context);
    }

    fn check_boundary_collision(
        &mut self,
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
        players: &[MatchPlayer],
        events: &mut EventCollection
    ) {
        const BALL_DISTANCE_THRESHOLD: f32 = 1.0;

        if let Some(owner_player_id) = self.current_owner {
            let t = owner_player_id;
        }

        if let Some(owner_player_id) = self.previous_owner {
            let owner = context.players.get(owner_player_id).unwrap();
            if owner.position.distance_to(&self.position) > BALL_DISTANCE_THRESHOLD {
                self.previous_owner = None;
            }
        } else {
            let mut nearby_players: Vec<&MatchPlayer> = players
                .iter()
                //.filter(|p| p.state != PlayerState::Injured)
                .filter(|player_position| {
                    let dx = player_position.position.x - self.position.x;
                    let dy = player_position.position.y - self.position.y;

                    dx * dx + dy * dy < BALL_DISTANCE_THRESHOLD * BALL_DISTANCE_THRESHOLD
                })
                .collect();

            let is_nearby_already_has_ball = nearby_players.iter().any(|player| player.has_ball);
            if is_nearby_already_has_ball {
                return;
            }

            let best_tackler = if nearby_players.len() == 1 {
                nearby_players.first()
            } else {
                nearby_players.iter().max_by(|player_a, player_b| {
                    let player_a = context.players.get(player_a.id).unwrap();
                    let player_b = context.players.get(player_b.id).unwrap();

                    let tackling_score_a = Self::calculate_tackling_score(player_a);
                    let tackling_score_b = Self::calculate_tackling_score(player_b);

                    tackling_score_a
                        .partial_cmp(&tackling_score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            };

            if let Some(player) = best_tackler {
                self.previous_owner = self.current_owner;
                self.current_owner = Some(player.id);

                if is_nearby_already_has_ball {
                    nearby_players
                        .iter()
                        .filter(|p| !p.has_ball)
                        .for_each(|mut player| {
                            events.add_ball_event(BallEvent::UnClaim(player.id));
                        });

                    return;
                }

                events.add_ball_event(BallEvent::Claimed(player.id));
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

    fn check_goal(&mut self, context: &MatchContext, result: &mut EventCollection) {
        if let Some(goal_side) = context.goal_positions.is_goal(self.position) {
            result.add_ball_event(BallEvent::Goal(goal_side, self.previous_owner));
            self.reset();
        }
    }

    fn update_velocity(&mut self) {
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

    fn move_to(&mut self, tick_context: &GameTickContext) {
        if let Some(owner_player_id) = self.current_owner {
            if let Some(owner_position) = tick_context
                .object_positions
                .players_positions
                .get_player_position(owner_player_id)
            {
                self.position = owner_position;
            }
        } else {
            self.position.x += self.velocity.x;
            self.position.y += self.velocity.y;
        }
    }


    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;

        self.velocity = Vector3::zeros();
    }
}