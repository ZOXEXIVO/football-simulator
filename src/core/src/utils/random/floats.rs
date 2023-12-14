extern crate rand;
use rand::*;

pub struct FloatUtils;

impl FloatUtils {
    #[inline]
    pub fn random(min: f32, max: f32) -> f32 {
        let random_val: f32 = random();

        min + (random_val * (max - min))
    }
}
