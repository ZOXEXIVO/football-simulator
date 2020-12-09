extern crate rand;
use rand::*;

pub struct IntegerUtils;

impl IntegerUtils {
      pub fn random(min: i32, max: i32) -> i32 {
            let random_val: f64 = rand::thread_rng().gen();

            min + (random_val * ((max - min) as f64)) as i32
      }
}