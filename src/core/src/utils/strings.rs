extern crate rand;

use rand::*;

pub struct StringUtils;

impl StringUtils {
    #[inline]
    pub fn random_string(n: i32) -> String {
        (0..n)
            .map(|i| {
                if i == 0 {
                    (65 + random::<u8>() % 26) as char
                } else {
                    (97 + random::<u8>() % 26) as char
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_string_length_is_correct() {
        let random_string = StringUtils::random_string(10);

        assert_eq!(10, random_string.len());
    }

    #[test]
    fn random_string_not_equals() {
        let random_string_a = StringUtils::random_string(10);
        let random_string_b = StringUtils::random_string(10);

        assert_ne!(random_string_a, random_string_b);
    }
}
