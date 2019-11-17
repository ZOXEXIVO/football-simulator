const SKILL_MIN_VALUE: u8 = 1;
const SKILL_MAX_VALUE: u8 = 20;

#[derive(Clone)]
pub struct PlayerSkills {
    pub technical: Technical,
    pub metal: Metal,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn train(&mut self, val: i8) {
        self.technical.train(val);
        self.metal.train(val);
        self.physical.train(val);
    }

    pub fn rest(&mut self){
        
    }
}

#[derive(Clone)]
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
    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.corners, val);
        safe_modify(&mut self.crossing, val);
        safe_modify(&mut self.dribbling, val);
        safe_modify(&mut self.finishing, val);
        safe_modify(&mut self.first_touch, val);
        safe_modify(&mut self.free_kick_taking, val);
        safe_modify(&mut self.heading, val);
        safe_modify(&mut self.long_shots, val);
        safe_modify(&mut self.long_throws, val);
        safe_modify(&mut self.marking, val);
        safe_modify(&mut self.passing, val);
        safe_modify(&mut self.penalty_taking, val);
        safe_modify(&mut self.tackling, val);
        safe_modify(&mut self.technique, val);
    }
}

#[derive(Clone)]
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
    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.aggression, val);
        safe_modify(&mut self.anticipation, val);
        safe_modify(&mut self.brawery, val);
        safe_modify(&mut self.composure, val);
        safe_modify(&mut self.contentration, val);
        safe_modify(&mut self.decisions, val);
        safe_modify(&mut self.determination, val);
        safe_modify(&mut self.flair, val);
        safe_modify(&mut self.leadership, val);
        safe_modify(&mut self.off_the_ball, val);
        safe_modify(&mut self.positioning, val);
        safe_modify(&mut self.teamwork, val);
        safe_modify(&mut self.vision, val);
        safe_modify(&mut self.work_rate, val);

    }
}

#[derive(Clone)]
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
    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.acceleration, val);
        safe_modify(&mut self.agility, val);
        safe_modify(&mut self.balance, val);
        safe_modify(&mut self.jumping_reach, val);
        safe_modify(&mut self.natural_fitness, val);
        safe_modify(&mut self.pace, val);
        safe_modify(&mut self.stamina, val);
        safe_modify(&mut self.strength, val);
    }
}

#[inline]
fn safe_modify(skill: &mut u8, val: i8) {
    if val < 0 {
        let abs_val = (val * -1) as u8;

        if *skill <= abs_val {
            *skill = SKILL_MIN_VALUE;
        } else {
            *skill = *skill - abs_val;
        }
    } else {
        let abs_val = val as u8;

        if *skill + abs_val > SKILL_MAX_VALUE {
            *skill = SKILL_MAX_VALUE;
        } else {
            *skill = *skill + abs_val;
        }
    }
}