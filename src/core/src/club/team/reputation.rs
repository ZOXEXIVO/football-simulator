#[derive(Debug)]
pub struct TeamReputation{
    pub home: u16,
    pub national: u16,
    pub world: u16
}

impl TeamReputation {
    pub fn new(home: u16, national: u16, world: u16) -> Self {
        TeamReputation{
            home, 
            national,
            world
        }
    }
}