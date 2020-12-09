use crate::{Player, Person, PlayerPositionType};
use chrono::{NaiveDate};

pub struct PlayerUtils;

impl PlayerUtils {
    #[inline]
    pub fn growth_potential(player: &Player, now: NaiveDate) -> f32 {
        let mut dap = ((player.skills.mental.determination as f32) / 5.0) * 0.05
            + ((player.attributes.ambition as f32) * 0.09)
            + ((player.attributes.professionalism as f32) * 0.115);

        let age = player.age(now);

        let ca = player.player_attributes.current_ability;
        let pa = player.player_attributes.potential_ability;

        if age < 24 {
            if pa <= (ca + 10) as u8 {
                dap = dap - 0.5;
            }
        }

        if age >= 24 && age < 29 {
            dap = dap - 0.5;
            if pa <= (ca + 10) as u8 {
                dap = dap - 0.5;
            }
        }

        if age >= 29 && age < 34 {
            dap = dap - 1.0;
            if pa <= (ca + 10) as u8 {
                dap = dap - 0.5;
            }
        }

        if age >= 34 {
            dap = dap - 1.0;
            if pa <= (ca + 10) as u8 && player.positions.position() == PlayerPositionType::Goalkeeper {
                dap = 0.5;
            }
        }

        dap = dap * 2.0;

        dap = dap.round();

        dap /= 2.0;

        dap
    }
}