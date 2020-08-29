#[derive(Debug)]
pub struct ClubReputation{
    pub home: u16,
    pub national: u16,
    pub world: u16
}

impl ClubReputation {
    pub fn new(home: u16, national: u16, world: u16) -> Self {
        ClubReputation{
            home, 
            national,
            world
        }
    }
}