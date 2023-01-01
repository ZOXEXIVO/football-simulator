mod mental;
mod physical;
pub mod result;
mod technical;

use crate::club::utils::PlayerUtils;
use crate::{Person, Player, Staff};
use chrono::Utc;
pub use mental::*;
pub use physical::*;
pub use technical::*;

fn determine_base_value_to_skill_increase(
    weeks_since_last_training: u32,
    player: &Player,
    coach: &Staff,
) -> f32 {
    let mut value_to_increase = 0.0;

    let base_value = 0.1;

    let coach_factor = coach.staff_attributes.mental.determination as f32 / 20.0;

    let ambition_factor = player.attributes.ambition as f32 / 20.0;

    let professionalism_factor = player.attributes.professionalism as f32 / 20.0;

    let age_factor = PlayerUtils::age_factor(player.age(Utc::now().naive_utc().date()));

    let training_factor = (1.0 + weeks_since_last_training as f32 / 2.0).powf(0.5);

    value_to_increase = base_value
        * coach_factor
        * ambition_factor
        * professionalism_factor
        * age_factor
        * training_factor;

    value_to_increase
}
