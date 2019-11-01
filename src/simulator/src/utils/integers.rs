extern crate rand;
use rand::*;

pub struct IntegerUtils;

impl IntegerUtils {
      pub fn random(min: u32, max: u32) -> u32 {
            let random_val: f64 = rand::thread_rng().gen();

            min + (random_val * ((max - min) as f64)) as u32
      }
}

// pub fn SequenceGenerator() -> FnMut() -> u32 {
//       let currentCounter: u32 = 0;

//       let counterClosure = || {
//             let current = currentCounter;

//             currentCounter = currentCounter + 1;

//             return current;
//       };

//       counterClosure
// }
