#[derive(Debug)]
pub struct ClubMood {
    pub state: ClubMoodState,
}

impl ClubMood {
    pub fn default() -> Self {
        ClubMood {
            state: ClubMoodState::Normal,
        }
    }
}

#[derive(Debug)]
pub enum ClubMoodState {
    Poor,
    Normal,
    Good,
    Excellent,
}
