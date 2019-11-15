use crate::player::player::Player;

pub struct FootballMatch{

}

impl FootballMatch{
    pub fn play(home: Vec<&Player>, away: Vec<&Player>) -> FootballMatchDetails{
        FootballMatchDetails{
            score: Score{
                home: 0,
                away: 0
            },
            player_changes: vec![],
        }
    }
}

pub struct FootballMatchDetails{
    pub score: Score,
    pub player_changes: Vec<PlayerChanges>
}

pub struct Score{
    pub home: u8,
    pub away: u8
}

pub struct PlayerChanges{

}