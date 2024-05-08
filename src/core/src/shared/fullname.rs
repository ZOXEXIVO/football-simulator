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
        let mut name = format!("{} {}", self.last_name, self.first_name);
        if let Some(middle_name) = self.middle_name.as_ref() {
            name.push_str(" ");
            name.push_str(middle_name);
        }
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fullname() {
        let fullname = FullName::new("John".to_string(), "Doe".to_string());

        assert_eq!(fullname.first_name, "John");
        assert_eq!(fullname.last_name, "Doe");
        assert_eq!(fullname.middle_name, None);
    }

    #[test]
    fn test_with_full_fullname() {
        let fullname = FullName::with_full("John".to_string(), "Doe".to_string(), "Smith".to_string());

        assert_eq!(fullname.first_name, "John");
        assert_eq!(fullname.last_name, "Doe");
        assert_eq!(fullname.middle_name, Some("Smith".to_string()));
    }

    #[test]
    fn test_display_without_middle_name() {
        let fullname = FullName::new("John".to_string(), "Doe".to_string());

        assert_eq!(format!("{}", fullname), "Doe John");
    }

    #[test]
    fn test_display_with_middle_name() {
        let fullname = FullName::with_full("John".to_string(), "Doe".to_string(), "Smith".to_string());

        assert_eq!(format!("{}", fullname), "Doe John Smith");
    }
}
