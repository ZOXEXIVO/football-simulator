#[derive(Clone)]
pub struct CountryContext {
    id: u32
}

impl CountryContext {
    pub fn new(id: u32) -> Self {
        CountryContext {
            id
        }
    }
}
