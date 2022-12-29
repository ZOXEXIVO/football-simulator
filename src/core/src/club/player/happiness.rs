#[derive(Debug)]
pub struct PlayerHappiness {
    positive: Vec<PositiveHappiness>,
    negative: Vec<NegativeHappiness>,
}

impl PlayerHappiness {
    pub fn new() -> Self {
        PlayerHappiness {
            positive: Vec::new(),
            negative: Vec::new(),
        }
    }

    pub fn is_happy(&self) -> bool {
        self.positive.len() > self.negative.len()
    }
}

#[derive(Debug)]
pub struct PositiveHappiness {
    pub description: String,
}

#[derive(Debug)]
pub struct NegativeHappiness {
    pub description: String,
}
