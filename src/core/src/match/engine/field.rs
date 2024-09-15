use crate::r#match::ball::Ball;
use crate::r#match::{
    FieldSquad, MatchFieldSize, MatchPlayer, MatchResultRaw, PositionType, TeamSquad,
    POSITION_POSITIONING,
};
use nalgebra::Vector3;

pub struct MatchField {
    pub size: MatchFieldSize,
    pub ball: Ball,
    pub players: Vec<MatchPlayer>,
    pub substitutes: Vec<MatchPlayer>,

    pub home_players: Option<FieldSquad>,
    pub away_players: Option<FieldSquad>,
}

impl MatchField {
    pub fn new(width: usize, height: usize, left_team_squad: TeamSquad, right_team_squad: TeamSquad) -> Self {
        let left_squad = FieldSquad::from_team(&left_team_squad);
        let away_squad = FieldSquad::from_team(&right_team_squad);

        let (players_on_field, substitutes) = setup_player_on_field(left_team_squad, right_team_squad);

        MatchField {
            size: MatchFieldSize::new(width, height),
            ball: Ball::with_coord(width as f32 / 2.0, height as f32 / 2.0),
            players: players_on_field,
            substitutes,
            home_players: Some(left_squad),
            away_players: Some(away_squad),
        }
    }

    pub fn swap_squads(&mut self) {
        std::mem::swap(&mut self.home_players, &mut self.away_players);
        self.players.iter_mut().for_each(|p| p.is_home = !p.is_home);
    }

    pub fn write_match_positions(&self, result: &mut MatchResultRaw, timestamp: u64) {
        // player positions
        self.players.iter().for_each(|player| {
            result
                .position_data
                .add_player_positions(player.player_id, timestamp, player.position);
        });

        // player positions
        self.substitutes.iter().for_each(|sub_player| {
            result.position_data.add_player_positions(
                sub_player.player_id,
                timestamp,
                sub_player.position,
            );
        });

        // write positions
        result
            .position_data
            .add_ball_positions(timestamp, self.ball.position);
    }
}

fn setup_player_on_field(
    left_team_squad: TeamSquad,
    right_team_squad: TeamSquad,
) -> (Vec<MatchPlayer>, Vec<MatchPlayer>) {
    let mut players_on_field: Vec<MatchPlayer> = Vec::with_capacity(22);
    let mut substitutes: Vec<MatchPlayer> = Vec::with_capacity(30);

    // home
    left_team_squad
        .main_squad
        .into_iter()
        .for_each(|mut home_player| {
            let tactics_position = home_player.tactics_position;

            POSITION_POSITIONING
                .iter()
                .filter(|(positioning, _, _)| *positioning == tactics_position)
                .map(|(_, home_position, _)| home_position)
                .for_each(|position| {
                    if let PositionType::Home(x, y) = position {
                        home_player.is_home = true;
                        home_player.position = Vector3::new(*x as f32, *y as f32, 0.0);
                        home_player.start_position = Vector3::new(*x as f32, *y as f32, 0.0);

                        players_on_field.push(home_player.clone());
                    }
                });
        });

    left_team_squad
        .substitutes
        .into_iter()
        .for_each(|mut home_player| {
            home_player.is_home = true;
            home_player.position = Vector3::new(1.0, 1.0, 0.0);

            substitutes.push(home_player);
        });

    // away
    right_team_squad
        .main_squad
        .into_iter()
        .for_each(|mut away_player| {
            let tactics_position = away_player.tactics_position;

            POSITION_POSITIONING
                .iter()
                .filter(|(positioning, _, _)| *positioning == tactics_position)
                .map(|(_, _, away_position)| away_position)
                .for_each(|position| {
                    if let PositionType::Away(x, y) = position {
                        away_player.is_home = false;

                        away_player.position = Vector3::new(*x as f32, *y as f32, 0.0);
                        away_player.start_position = Vector3::new(*x as f32, *y as f32, 0.0);

                        players_on_field.push(away_player.clone());
                    }
                });
        });

    right_team_squad
        .substitutes
        .into_iter()
        .for_each(|mut away_player| {
            away_player.is_home = false;
            away_player.position = Vector3::new(1.0, 1.0, 0.0);

            substitutes.push(away_player);
        });

    (players_on_field, substitutes)
}
