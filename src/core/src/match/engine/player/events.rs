use crate::r#match::{Ball, MatchContext};
use nalgebra::Vector3;

pub enum PlayerUpdateEvent {
    Goal(u32),
    Assist(u32),
    BallCollision(u32),
    TacklingBall(u32),
    BallOwnerChange(u32),
    PassTo(u32, Vector3<f32>, f64),
    ClearBall(Vector3<f32>),
    RushOut(u32),
    Shoot(u32),
    StayInGoal(u32),
    MoveBall(u32, Vector3<f32>),
    BallMoveTowardsPlayer(u32),
    CommunicateMessage(u32, &'static str),
    OfferSupport(u32),
    ClaimBall(u32),
    GainBall(u32),
    CommitFoul,
    RequestPass(u32, u32),
}

pub struct PlayerUpdateEventCollection {
    pub events: Vec<PlayerUpdateEvent>,
}

impl PlayerUpdateEventCollection {
    pub fn new() -> Self {
        PlayerUpdateEventCollection {
            events: Vec::with_capacity(10),
        }
    }

    pub fn with_event(event: PlayerUpdateEvent) -> Self {
        let mut vec = Vec::with_capacity(10);

        vec.push(event);

        PlayerUpdateEventCollection { events: vec }
    }

    pub fn add(&mut self, event: PlayerUpdateEvent) {
        self.events.push(event)
    }

    pub fn join(&mut self, events: PlayerUpdateEventCollection) {
        self.events.extend(events.events)
    }

    pub fn add_range(&mut self, events: impl Iterator<Item = PlayerUpdateEvent>) {
        self.events.extend(events)
    }

    pub fn process<'p>(&self, ball: &mut Ball, context: &mut MatchContext) {
        for event in &self.events {
            match event {
                PlayerUpdateEvent::Goal(player_id) => {
                    let player = context.players.get_mut(*player_id).unwrap();

                    player.statistics.add_goal(context.time.time)
                }
                PlayerUpdateEvent::Assist(player_id) => {
                    let player = context.players.get_mut(*player_id).unwrap();

                    player.statistics.add_assist(context.time.time)
                }
                PlayerUpdateEvent::BallCollision(player_id) => {
                    let player = context.players.get_mut(*player_id).unwrap();

                    if player.skills.technical.first_touch > 10.0 {
                        player.has_ball = true;
                        ball.velocity = Vector3::<f32>::zeros();
                    }
                }
                PlayerUpdateEvent::TacklingBall(player_id) => {
                    ball.owned = true;
                    ball.last_owner = Some(*player_id);

                    let mut player = context.players.get_mut(*player_id).unwrap();
                    player.has_ball = true;
                }
                PlayerUpdateEvent::BallOwnerChange(player_id) => {
                    ball.owned = true;
                    ball.last_owner = Some(*player_id);
                }
                PlayerUpdateEvent::PassTo(player_id, pass_target, pass_power) => {
                    let ball_pass_vector = pass_target - ball.position;
                    ball.velocity = ball_pass_vector.normalize();

                    let mut player = context.players.get_mut(*player_id).unwrap();
                    player.has_ball = false;
                }
                PlayerUpdateEvent::RushOut(_) => {}
                PlayerUpdateEvent::StayInGoal(_) => {}
                PlayerUpdateEvent::CommunicateMessage(player_id, message) => {}
                PlayerUpdateEvent::OfferSupport(_) => {}
                PlayerUpdateEvent::ClaimBall(player_id) => {
                    let player = context.players.get(*player_id).unwrap();
                    // Рассчитываем приоритет на основе позиции, скорости и навыков
                    // let priority = player.calculate_ball_priority(ball);
                    // ball_claims.insert(player_id, priority);
                }
                PlayerUpdateEvent::ClearBall(ball_velocity) => {
                    ball.velocity = *ball_velocity;
                }
                PlayerUpdateEvent::MoveBall(player_id, ball_velocity) => {
                    ball.velocity = *ball_velocity;

                    ball.owned = true;
                    ball.last_owner = Some(*player_id)
                }
                PlayerUpdateEvent::GainBall(player_id) => {
                    ball.owned = true;
                    ball.last_owner = Some(*player_id);
                }
                PlayerUpdateEvent::CommitFoul => {}
                PlayerUpdateEvent::BallMoveTowardsPlayer(player_id) => {
                    let player = context.players.get_mut(*player_id).unwrap();

                    ball.velocity = player.velocity;
                    ball.owned = true;
                    ball.last_owner = Some(*player_id)
                }
                PlayerUpdateEvent::Shoot(_) => {}
                PlayerUpdateEvent::RequestPass(_, _) => {}
            }
        }
    }
}
