#[derive(Clone)]
pub struct TeamContext<'c> {
    pub id: u32,
    pub name: &'c str
}

impl<'c> TeamContext<'c> {
    pub fn new(id: u32, name: &'c str) -> Self {
        TeamContext {
            id,
            name
        }
    }
}
