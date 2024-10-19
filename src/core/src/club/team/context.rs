#[derive(Clone)]
pub struct TeamContext {
    pub id: u32,
}

impl<'c> TeamContext {
    pub fn new(id: u32) -> Self {
        TeamContext { id }
    }
}
