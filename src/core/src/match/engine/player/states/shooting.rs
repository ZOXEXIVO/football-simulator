use crate::r#match::{MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct ShootingState {}

impl ShootingState {
    pub fn process(
        _in_state_time: u64,
        _player: &mut MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        // write code for processing shoot state

        //         player.velocity = player.skills.running_speed();
        //         // let distance_to_goal = (self.position.x - self.field.width as i16 / 2).abs();
        //         // if distance_to_goal < 50 {
        //         //     let mut rng = thread_rng();
        //     let shot_success = rng.gen_range(0, 100);
        //
        //     let shooting_skill = self.skills.technical.finishing;
        //
        //     if shot_success < shooting_skill {
        //         if self.position.x < self.field.width as i16 / 2 {
        //             self.field.home_goals += 1;
        //         } else {
        //             self.field.away_goals += 1;
        //         }
        //     }
        // }

        None
    }
}