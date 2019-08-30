extern crate rand;

use rand::*;

pub struct StringUtils;

impl StringUtils {
      pub fn random_string(n: i32) -> String {
          (0..n).map(|_| random::<char>()).collect()
      }
}
