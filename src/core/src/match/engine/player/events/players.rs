use crate::r#match::events::Event;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{GoalDetail, MatchContext, MatchField};
use log::info;
use nalgebra::Vector3;
use crate::r#match::player::events::PassingEventModel;
use crate::r#match::statistics::MatchStatisticType;

#[derive(Debug)]
pub enum PlayerEvent {
    Goal(u32),
    Assist(u32),
    BallCollision(u32),
    TacklingBall(u32),
    BallOwnerChange(u32),
    PassTo(PassingEventModel),
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
    RequestHeading(u32, Vector3<f32>),
    RequestShot(u32, Vector3<f32>),
    RequestBallReceive(u32),
    TakeBall(u32),
}

pub struct PlayerEventDispatcher;

impl PlayerEventDispatcher {
    pub fn dispatch<'a>(
        event: PlayerEvent,
        field: &mut MatchField,
        context: &mut MatchContext,
    ) -> Vec<Event> {
        let remaining_events = Vec::new();

        info!("PLAYER EVENT: {:?}", event);

        match event {
            PlayerEvent::Goal(player_id) => {
                let player = field.get_player_mut(player_id).unwrap();
                player.statistics.add_goal(context.time.time);

                context.score.add_goal_detail(GoalDetail{
                    player_id,
                    stat_type: MatchStatisticType::Goal,
                    time: context.time.time,
                });

                field.ball.previous_owner = None;
                field.ball.current_owner = None;
            }
            PlayerEvent::Assist(player_id) => {
                let player = field.get_player_mut(player_id).unwrap();

                context.score.add_goal_detail(GoalDetail{
                    player_id,
                    stat_type: MatchStatisticType::Assist,
                    time: context.time.time,
                });

                player.statistics.add_assist(context.time.time)
            }
            PlayerEvent::BallCollision(player_id) => {
                let player = field.get_player_mut(player_id).unwrap();

                if player.skills.technical.first_touch > 10.0 {
                    //player.has_ball = true;
                    //field.ball.velocity = Vector3::<f32>::zeros();
                }
            }
            PlayerEvent::TacklingBall(player_id) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);
            }
            PlayerEvent::BallOwnerChange(player_id) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);
            }
            PlayerEvent::PassTo(event_model) => {
                let ball_pass_vector = event_model.pass_target - field.ball.position;
                let direction = ball_pass_vector.normalize();
                let pass_force = event_model.pass_force as f32;
                let pass_force_multiplier = 1.7;

                // Calculate the ball's velocity based on the pass force
                let velocity = direction * (pass_force * pass_force_multiplier);

                field.ball.velocity = velocity;

                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = None;

                field.ball.flags.in_passing_state_time = 100;
            }
            PlayerEvent::RushOut(_) => {}
            PlayerEvent::StayInGoal(_) => {}
            PlayerEvent::CommunicateMessage(_player_id, _message) => {}
            PlayerEvent::OfferSupport(_) => {}
            PlayerEvent::ClaimBall(player_id) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);

                field.ball.flags.in_passing_state_time = 30;
            }
            PlayerEvent::ClearBall(_ball_velocity) => {
                //field.ball.velocity = *ball_velocity;
            }
            PlayerEvent::MoveBall(player_id, ball_velocity) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);

                field.ball.velocity = ball_velocity;
            }
            PlayerEvent::GainBall(player_id) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);
            }
            PlayerEvent::CommitFoul => {}
            PlayerEvent::Shoot(player_id, target_direction) => {
                let ball_pass_vector = target_direction - field.ball.position;

                field.ball.previous_owner = Some(player_id);
                field.ball.current_owner = None;
                field.ball.velocity = ball_pass_vector.normalize();

                field.ball.flags.in_passing_state_time = 100;
            }
            PlayerEvent::RequestHeading(_, _) => {}
            PlayerEvent::RequestShot(_, _) => {}
            PlayerEvent::RequestBallReceive(_) => {}
            PlayerEvent::UnClaimBall(player_id) => {
                let player = field.get_player_mut(player_id).unwrap();

                player.state = PlayerState::Injured
            }
            PlayerEvent::CaughtBall(player_id) => {
                field.ball.previous_owner = field.ball.current_owner;
                field.ball.current_owner = Some(player_id);
            }
            PlayerEvent::MovePlayer(player_id, position) => {
                let player = field.get_player_mut(player_id).unwrap();
                player.position = position
            }
            PlayerEvent::TakeBall(player_id) => {
                let player = field.get_player_mut(player_id).unwrap();
                player.run_for_ball();
            }
        }

        remaining_events
    }
}
