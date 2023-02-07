mod mental;
mod physical;
pub mod result;
mod technical;

use crate::club::utils::PlayerUtils;
use crate::{Person, Player, Staff};
use chrono::NaiveDate;
pub use mental::*;
pub use physical::*;
pub use technical::*;

fn determine_base_value_to_skill_increase(
    now: NaiveDate,
    weeks_since_last_training: u32,
    player: &Player,
    coach: &Staff,
) -> f32 {
    let mut base_value = 0.1;

    let coach_factor = coach.staff_attributes.mental.determination as f32 / 20.0;

    let ambition_factor = player.attributes.ambition as f32 / 20.0;

    let professionalism_factor = player.attributes.professionalism as f32 / 20.0;

    let age_factor = PlayerUtils::age_factor(player.age(now));

    let training_factor = (1.0 + weeks_since_last_training as f32 / 2.0).powf(0.5);

    let potential_ability_factor = player.player_attributes.potential_ability as f32 / 200.0;
    let current_ability_factor = player.player_attributes.current_ability as f32 / 200.0;

    let ability_factor = (1.0 - (current_ability_factor / potential_ability_factor)) * 0.3;

    base_value = base_value
        * ability_factor
        * coach_factor
        * ambition_factor
        * professionalism_factor
        * age_factor
        * training_factor;

    base_value
}
