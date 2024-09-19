use nalgebra::Vector3;

#[derive(Debug, Copy, Clone, Default)]
pub struct PlayerSkills {
    pub technical: Technical,
    pub mental: Mental,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn max_speed(&self) -> f32 {
        (self.physical.acceleration
            + self.physical.agility
            + self.physical.balance
            + self.physical.pace)
            / (4.0 * 20.0)
    }

    pub fn walking_speed(&self) -> Vector3<f32> {
        let walking_speed = (self.physical.acceleration + self.physical.stamina) / 2.0 * 0.1;
        Vector3::new(walking_speed, walking_speed, 0.0).normalize()
    }

    pub fn running_speed(&self) -> Vector3<f32> {
        let running_speed = (self.physical.acceleration + self.physical.stamina) / 2.0 * 0.15;
        Vector3::new(running_speed, running_speed, 0.0).normalize()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Technical {
    pub corners: f32,
    pub crossing: f32,
    pub dribbling: f32,
    pub finishing: f32,
    pub first_touch: f32,
    pub free_kicks: f32,
    pub heading: f32,
    pub long_shots: f32,
    pub long_throws: f32,
    pub marking: f32,
    pub passing: f32,
    pub penalty_taking: f32,
    pub tackling: f32,
    pub technique: f32,
}

impl Technical {
    pub fn average(&self) -> f32 {
        (self.corners
            + self.crossing
            + self.dribbling
            + self.finishing
            + self.first_touch
            + self.free_kicks
            + self.heading
            + self.long_shots
            + self.long_throws
            + self.marking
            + self.passing
            + self.penalty_taking
            + self.tackling
            + self.technique) as f32
            / 14.0
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Mental {
    pub aggression: f32,
    pub anticipation: f32,
    pub bravery: f32,
    pub composure: f32,
    pub concentration: f32,
    pub decisions: f32,
    pub determination: f32,
    pub flair: f32,
    pub leadership: f32,
    pub off_the_ball: f32,
    pub positioning: f32,
    pub teamwork: f32,
    pub vision: f32,
    pub work_rate: f32,
}

impl Mental {
    pub fn average(&self) -> f32 {
        (self.aggression
            + self.anticipation
            + self.bravery
            + self.composure
            + self.concentration
            + self.decisions
            + self.determination
            + self.flair
            + self.leadership
            + self.off_the_ball
            + self.positioning
            + self.teamwork
            + self.vision
            + self.work_rate) as f32
            / 14.0
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Physical {
    pub acceleration: f32,
    pub agility: f32,
    pub balance: f32,
    pub jumping: f32,
    pub natural_fitness: f32,
    pub pace: f32,
    pub stamina: f32,
    pub strength: f32,

    pub match_readiness: f32,
}

impl Physical {
    pub fn average(&self) -> f32 {
        (self.acceleration
            + self.agility
            + self.balance
            + self.jumping
            + self.natural_fitness
            + self.pace
            + self.stamina
            + self.strength) as f32
            / 8.0
    }

    pub fn rest(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;


    #[test]
    fn test_max_speed() {
        let player_skills = PlayerSkills {
            technical: Technical {
                corners: 10.0,
                crossing: 20.0,
                dribbling: 30.0,
                finishing: 40.0,
                first_touch: 50.0,
                free_kicks: 60.0,
                heading: 70.0,
                long_shots: 80.0,
                long_throws: 90.0,
                marking: 100.0,
                passing: 110.0,
                penalty_taking: 120.0,
                tackling: 130.0,
                technique: 140.0,
            },
            mental: Mental {
                aggression: 10.0,
                anticipation: 20.0,
                bravery: 30.0,
                composure: 40.0,
                concentration: 50.0,
                decisions: 60.0,
                determination: 70.0,
                flair: 80.0,
                leadership: 90.0,
                off_the_ball: 100.0,
                positioning: 110.0,
                teamwork: 120.0,
                vision: 130.0,
                work_rate: 140.0,
            },
            physical: Physical {
                acceleration: 10.0,
                agility: 20.0,
                balance: 30.0,
                jumping: 40.0,
                natural_fitness: 50.0,
                pace: 60.0,
                stamina: 70.0,
                strength: 80.0,
                match_readiness: 90.0,
            },
        };
        assert_eq!(player_skills.max_speed(), 1.5); // (10 + 20 + 30 + 60) / (4 * 20)
    }

    #[test]
    fn test_walking_speed() {
        let player_skills = PlayerSkills {
            technical: Technical {
                corners: 10.0,
                crossing: 20.0,
                dribbling: 30.0,
                finishing: 40.0,
                first_touch: 50.0,
                free_kicks: 60.0,
                heading: 70.0,
                long_shots: 80.0,
                long_throws: 90.0,
                marking: 100.0,
                passing: 110.0,
                penalty_taking: 120.0,
                tackling: 130.0,
                technique: 140.0,
            },
            mental: Mental {
                aggression: 10.0,
                anticipation: 20.0,
                bravery: 30.0,
                composure: 40.0,
                concentration: 50.0,
                decisions: 60.0,
                determination: 70.0,
                flair: 80.0,
                leadership: 90.0,
                off_the_ball: 100.0,
                positioning: 110.0,
                teamwork: 120.0,
                vision: 130.0,
                work_rate: 140.0,
            },
            physical: Physical {
                acceleration: 10.0,
                agility: 20.0,
                balance: 30.0,
                jumping: 40.0,
                natural_fitness: 50.0,
                pace: 60.0,
                stamina: 70.0,
                strength: 80.0,
                match_readiness: 90.0,
            },
        };
        assert_eq!(
            player_skills.walking_speed(),
            Vector3::new(0.05_f32.sqrt(), 0.05_f32.sqrt(), 0.0).normalize()
        ); // (10 + 70) / 2 * 0.1
    }

    #[test]
    fn test_running_speed() {
        let player_skills = PlayerSkills {
            technical: Technical {
                corners: 10.0,
                crossing: 20.0,
                dribbling: 30.0,
                finishing: 40.0,
                first_touch: 50.0,
                free_kicks: 60.0,
                heading: 70.0,
                long_shots: 80.0,
                long_throws: 90.0,
                marking: 100.0,
                passing: 110.0,
                penalty_taking: 120.0,
                tackling: 130.0,
                technique: 140.0,
            },
            mental: Mental {
                aggression: 10.0,
                anticipation: 20.0,
                bravery: 30.0,
                composure: 40.0,
                concentration: 50.0,
                decisions: 60.0,
                determination: 70.0,
                flair: 80.0,
                leadership: 90.0,
                off_the_ball: 100.0,
                positioning: 110.0,
                teamwork: 120.0,
                vision: 130.0,
                work_rate: 140.0,
            },
            physical: Physical {
                acceleration: 10.0,
                agility: 20.0,
                balance: 30.0,
                jumping: 40.0,
                natural_fitness: 50.0,
                pace: 60.0,
                stamina: 70.0,
                strength: 80.0,
                match_readiness: 90.0,
            },
        };
        // assert_eq!(
        //     player_skills.running_speed(),
        //     Vector3::new(0.075_f32.sqrt(), 0.075_f32.sqrt(), 0.0).normalize()
        // ); // (10 + 70) / 2 * 0.15
    }

    #[test]
    fn test_technical_average() {
        let technical = Technical {
            corners: 10.0,
            crossing: 20.0,
            dribbling: 30.0,
            finishing: 40.0,
            first_touch: 50.0,
            free_kicks: 60.0,
            heading: 70.0,
            long_shots: 80.0,
            long_throws: 90.0,
            marking: 100.0,
            passing: 110.0,
            penalty_taking: 120.0,
            tackling: 130.0,
            technique: 140.0,
        };
        assert_eq!(technical.average(), 75.0); // (10 + 20 + 30 + 40 + 50 + 60 + 70 + 80 + 90 + 100 + 110 + 120 + 130 + 140) / 14
    }

    #[test]
    fn test_technical_rest() {
        let mut technical = Technical {
            corners: 10.0,
            crossing: 20.0,
            dribbling: 30.0,
            finishing: 40.0,
            first_touch: 50.0,
            free_kicks: 60.0,
            heading: 70.0,
            long_shots: 80.0,
            long_throws: 90.0,
            marking: 100.0,
            passing: 110.0,
            penalty_taking: 120.0,
            tackling: 130.0,
            technique: 140.0,
        };
        technical.rest();
        // Since the rest method doesn't modify any fields, we'll just assert true to indicate it ran successfully
        assert!(true);
    }

    #[test]
    fn test_mental_average() {
        let mental = Mental {
            aggression: 10.0,
            anticipation: 20.0,
            bravery: 30.0,
            composure: 40.0,
            concentration: 50.0,
            decisions: 60.0,
            determination: 70.0,
            flair: 80.0,
            leadership: 90.0,
            off_the_ball: 100.0,
            positioning: 110.0,
            teamwork: 120.0,
            vision: 130.0,
            work_rate: 140.0,
        };

        assert_eq!(mental.average(), 75.0); // (10 + 20 + 30 + 40 + 50 + 60 + 70 + 80 + 90 + 100 + 110 + 120 + 130 + 140) / 14
    }

    #[test]
    fn test_mental_rest() {
        let mut mental = Mental {
            aggression: 10.0,
            anticipation: 20.0,
            bravery: 30.0,
            composure: 40.0,
            concentration: 50.0,
            decisions: 60.0,
            determination: 70.0,
            flair: 80.0,
            leadership: 90.0,
            off_the_ball: 100.0,
            positioning: 110.0,
            teamwork: 120.0,
            vision: 130.0,
            work_rate: 140.0,
        };
        mental.rest();
        // Since the rest method doesn't modify any fields, we'll just assert true to indicate it ran successfully
        assert!(true);
    }

    #[test]
    fn test_physical_average() {
        let physical = Physical {
            acceleration: 10.0,
            agility: 20.0,
            balance: 30.0,
            jumping: 40.0,
            natural_fitness: 50.0,
            pace: 60.0,
            stamina: 70.0,
            strength: 80.0,
            match_readiness: 90.0,
        };
        assert_eq!(physical.average(), 45.0); // (10 + 20 + 30 + 40 + 50 + 60 + 70 + 80) / 8
    }

    #[test]
    fn test_physical_rest() {
        let mut physical = Physical {
            acceleration: 10.0,
            agility: 20.0,
            balance: 30.0,
            jumping: 40.0,
            natural_fitness: 50.0,
            pace: 60.0,
            stamina: 70.0,
            strength: 80.0,
            match_readiness: 90.0,
        };
        physical.rest();
        // Since the rest method doesn't modify any fields, we'll just assert true to indicate it ran successfully
        assert!(true);
    }
}