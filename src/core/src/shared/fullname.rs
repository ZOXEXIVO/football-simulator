use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct FullName {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
}

impl FullName {
    pub fn new(first_name: String, last_name: String) -> Self {
        FullName {
            first_name,
            last_name,
            middle_name: None,
        }
    }

    pub fn with_full(first_name: String, last_name: String, middle_name: String) -> Self {
        FullName {
            first_name,
            last_name,
            middle_name: Some(middle_name),
        }
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.middle_name.is_some() {
            write!(f, "{} {} {}", self.last_name, self.first_name, self.middle_name.as_ref().unwrap())
        } else {
            write!(f, "{} {}", self.last_name, self.first_name)
        }
    }
}