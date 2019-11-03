pub struct PlayerSkills {
    pub technical: Technical,
    pub metal: Metal,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn train(&mut self, val: u8) {
        self.technical.train(val);
        self.metal.train(val);
        self.physical.train(val);
    }
}

pub struct Technical {
    pub corners: u8,
    pub crossing: u8,
    pub dribbling: u8,
    pub finishing: u8,
    pub first_touch: u8,
    pub free_kick_taking: u8,
    pub heading: u8,
    pub long_shots: u8,
    pub long_throws: u8,
    pub marking: u8,
    pub passing: u8,
    pub penalty_taking: u8,
    pub tackling: u8,
    pub technique: u8,
}

impl Technical {
    pub fn train(&mut self, val: u8) {
        self.corners += val;
        self.long_shots += val;
    }
}

pub struct Metal {
    pub aggression: u8,
    pub anticipation: u8,
    pub brawery: u8,
    pub composure: u8,
    pub contentration: u8,
    pub decisions: u8,
    pub determination: u8,
    pub flair: u8,
    pub leadership: u8,
    pub off_the_ball: u8,
    pub positioning: u8,
    pub teamwork: u8,
    pub vision: u8,
    pub work_rate: u8,
}

impl Metal {
    pub fn train(&mut self, val: u8) {
        self.aggression += val;
        self.anticipation += val;
        self.positioning += val;
        self.determination += val;
        self.teamwork += val;
        self.vision += val;
        self.work_rate += val;
        self.off_the_ball += val;
    }
}

pub struct Physical {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping_reach: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
}

impl Physical {
    pub fn train(&mut self, val: u8) {
        self.acceleration += val;
        self.agility += val;
        self.balance += val;
        self.jumping_reach += val;
    }
}
