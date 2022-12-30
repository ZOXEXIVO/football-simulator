extern crate rand;
use rand::*;

pub struct FloatUtils;

impl FloatUtils {
    #[inline]
    pub fn random(min: f32, max: f32) -> f32 {
        let random_val: f64 = thread_rng().gen();

        min + (random_val as f32 * (max - min))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_sequential_returns_different_numbers() {
        let random_int_1 = FloatUtils::random(0f32, 1000.0f32);
        let random_int_2 = FloatUtils::random(0f32, 1000.0f32);

        assert_ne!(random_int_1, random_int_2);
    }
}
