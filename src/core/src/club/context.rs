#[derive(Clone)]
pub struct ClubContext<'c> {
    pub id: u32,
    pub name: &'c str
}

impl<'c> ClubContext<'c> {
    pub fn new(id: u32, name: &'c str) -> Self {
        ClubContext {
            id,
            name
        }
    }
}
