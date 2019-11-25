#[derive(Debug, Clone)]
pub struct Tactics{
    pub positioning: TacticsPositioning
}

impl Tactics{
    pub fn new() -> Self{
        Tactics{
            positioning: TacticsPositioning::P442
        }
    }
}

#[derive(Debug, Clone)]
pub enum TacticsPositioning{
    P442
}