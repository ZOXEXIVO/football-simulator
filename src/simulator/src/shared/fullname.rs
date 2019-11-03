use std::fmt::{Display, Formatter, Result};

pub struct FullName {
      pub first_name: String,
      pub last_name: String,
      pub middle_name: String,
}


impl Display for FullName {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {} {}", self.last_name, self.first_name, self.middle_name)
      }
}