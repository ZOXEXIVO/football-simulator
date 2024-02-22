use std::collections::HashMap;
use nalgebra::Vector3;
use crate::r#match::MatchField;
use crate::r#match::position::PlayerFieldPosition;

pub struct GameTickContext {
    pub objects_positions: MatchObjectsPositions
}

pub struct PlayerTickContext {
    pub ball_context: BallContext,
}

pub struct MatchObjectsPositions {
    pub ball_position: Vector3<f32>,
    pub ball_velocity: Vector3<f32>,
    pub players_positions: Vec<PlayerFieldPosition>,
    pub player_distances: PlayerDistanceClosure
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        let positions:  Vec<PlayerFieldPosition> = field
            .players
            .iter()
            .map(|p| PlayerFieldPosition {
                player_id: p.player_id,
                is_home: p.is_home,
                position: p.position,
            })
            .collect();

        let distances = PlayerDistanceClosure::new();



        MatchObjectsPositions {
            ball_position: field.ball.position,
            ball_velocity: field.ball.velocity,
            players_positions: positions,
            player_distances: distances
        }
    }
}


pub struct PlayerDistanceClosure {
    distances: HashMap<u32, f32>
}

impl PlayerDistanceClosure {
    pub fn new() -> Self {
        PlayerDistanceClosure {
            distances: HashMap::new()
        }
    }

    pub fn add(&mut self, player_id_from: u32, player_id_to: u32, distance: f32){

    }

    pub fn get(&self, player_id_from: u32, player_id_to: u32) -> Option<f32> {
        None
    }
}

pub struct BallContext {
    pub is_ball_heading_towards_goal: bool,
    pub ball_is_on_player_home_side: bool,

    pub ball_distance: f32,
}



