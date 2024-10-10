use crate::r#match::player::state::PlayerState;
use crate::r#match::{MatchContext, MatchField};
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
    Shoot(u32, Vector3<f32>),
    MovePlayer(u32, Vector3<f32>),
    StayInGoal(u32),
    MoveBall(u32, Vector3<f32>),
    CommunicateMessage(u32, &'static str),
    OfferSupport(u32),
    ClaimBall(u32),
    UnClaimBall(u32),
    GainBall(u32),
    CaughtBall(u32),
    CommitFoul,
    RequestPass(u32, u32),
    RequestHeading(u32, Vector3<f32>),
    RequestShot(u32, Vector3<f32>),
    RequestBallReceive(u32),
}

pub struct PlayerEventDispatcher;

impl PlayerEventDispatcher {
    pub fn process<'p>(&self, field: &mut MatchField, context: &mut MatchContext) {
        for event in &self.events {
            match event {
                PlayerUpdateEvent::Goal(player_id) => {
                    let player = field.get_player_mut(*player_id).unwrap();

                    player.statistics.add_goal(context.time.time)
                }
                PlayerUpdateEvent::Assist(player_id) => {
                    let player = field.get_player_mut(*player_id).unwrap();

                    player.statistics.add_assist(context.time.time)
                }
                PlayerUpdateEvent::BallCollision(player_id) => {
                    let player = field.get_player_mut(*player_id).unwrap();

                    if player.skills.technical.first_touch > 10.0 {
                        player.has_ball = true;
                        //field.ball.velocity = Vector3::<f32>::zeros();
                    }
                }
                PlayerUpdateEvent::TacklingBall(player_id) => {
                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.current_owner = Some(*player_id);

                    let player = field.get_player_mut(*player_id).unwrap();
                    player.has_ball = true;
                }
                PlayerUpdateEvent::BallOwnerChange(player_id) => {
                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.current_owner = Some(*player_id);
                }
                PlayerUpdateEvent::PassTo(player_id, pass_target, pass_power) => {
                    let ball_pass_vector = pass_target - field.ball.position;
                    field.ball.velocity = ball_pass_vector.normalize();

                    let player = field.get_player_mut(*player_id).unwrap();
                    player.has_ball = false;
                }
                PlayerUpdateEvent::RushOut(_) => {}
                PlayerUpdateEvent::StayInGoal(_) => {}
                PlayerUpdateEvent::CommunicateMessage(player_id, message) => {}
                PlayerUpdateEvent::OfferSupport(_) => {}
                PlayerUpdateEvent::ClaimBall(player_id) => {
                    // TODO
                    field.players.iter_mut().for_each(|player| {
                        player.has_ball = false;
                    });

                    let mut player = field.get_player_mut(*player_id).unwrap();

                    player.has_ball = true;

                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.previous_owner = Some(*player_id);
                }
                PlayerUpdateEvent::ClearBall(ball_velocity) => {
                    //field.ball.velocity = *ball_velocity;
                }
                PlayerUpdateEvent::MoveBall(player_id, ball_velocity) => {
                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.current_owner = Some(*player_id);
                }
                PlayerUpdateEvent::GainBall(player_id) => {
                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.current_owner = Some(*player_id);
                }
                PlayerUpdateEvent::CommitFoul => {}
                PlayerUpdateEvent::Shoot(player_id, target_direction) => {
                    let ball_pass_vector = target_direction - field.ball.position;

                    field.ball.previous_owner = Some(*player_id);
                    field.ball.current_owner = None;
                    field.ball.velocity = ball_pass_vector.normalize();

                    let player = field.get_player_mut(*player_id).unwrap();
                    player.has_ball = false;
                }
                PlayerUpdateEvent::RequestPass(_, _) => {}
                PlayerUpdateEvent::RequestHeading(_, _) => {}
                PlayerUpdateEvent::RequestShot(_, _) => {}
                PlayerUpdateEvent::RequestBallReceive(_) => {}
                PlayerUpdateEvent::UnClaimBall(player_id) => {
                    let mut player = field.get_player_mut(*player_id).unwrap();

                    player.state = PlayerState::Injured
                }
                PlayerUpdateEvent::CaughtBall(player_id) => {
                    field.players.iter_mut().for_each(|player| {
                        player.has_ball = false;
                    });

                    let mut player = field.get_player_mut(*player_id).unwrap();

                    player.has_ball = true;

                    field.ball.previous_owner = field.ball.current_owner;
                    field.ball.previous_owner = Some(*player_id);
                }
                PlayerUpdateEvent::MovePlayer(player_id, position) => {
                    let player = field.get_player_mut(*player_id).unwrap();
                    player.position = *position
                }
            }
        }
    }
}
