use crate::r#match::position::VectorExtensions;
use crate::r#match::{BallUpdateEvent, GameTickContext, PlayerUpdateEvent};

pub struct ObjectCollisions;

impl ObjectCollisions {
    pub fn process(
        tick_context: &GameTickContext,
    ) -> (Vec<BallUpdateEvent>, Vec<PlayerUpdateEvent>) {
        let mut ball_events = Vec::with_capacity(10);
        let mut player_events = Vec::with_capacity(10);

        let object_positions = &tick_context.objects_positions;

        // Ball-player collisions
        for player in &object_positions.players_positions {
            let distance = object_positions.ball_position.distance_to(&player.position);

            if distance < 3.0 {
                let collision_normal =
                    (object_positions.ball_position - player.position).normalize();
                let new_bank_velocity = (object_positions.ball_velocity
                    - 2.0
                        * object_positions.ball_velocity.dot(&collision_normal)
                        * collision_normal);

                ball_events.push(BallUpdateEvent::UpdateVelocity(new_bank_velocity));
                ball_events.push(BallUpdateEvent::PlayerCollision(player.player_id));

                player_events.push(PlayerUpdateEvent::BallCollision(player.player_id));
            }
        }

        let distance_radius = 5.0;
        let closest_players = object_positions
            .player_distances
            .get_collisions(distance_radius);

        for distance_item in closest_players {
            player_events.push(PlayerUpdateEvent::TryAroundPlayer(
                distance_item.player_from_id,
                distance_item.player_from_position,
            ));
            player_events.push(PlayerUpdateEvent::TryAroundPlayer(
                distance_item.player_to_id,
                distance_item.player_to_position,
            ));
        }

        (ball_events, player_events)
    }
}
