use crate::r#match::ball::Ball;
use crate::r#match::position::FieldPosition;
use crate::r#match::{
    FieldSquad, FootballMatchResult, MatchPlayer, PositionType, TeamSquad, POSITION_POSITIONING,
};

pub struct Field {
    pub width: usize,
    pub height: usize,
    pub ball: Ball,
    pub players: Vec<MatchPlayer>,
    pub substitutes: Vec<MatchPlayer>,

    pub home_players: FieldSquad,
    pub away_players: FieldSquad,
}

impl Field {
    pub fn new(width: usize, height: usize, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        let home_players = FieldSquad {
            main: home_squad.main_squad.iter().map(|p| p.player_id).collect(),
            substitutes: home_squad.substitutes.iter().map(|p| p.player_id).collect(),
        };

        let away_players = FieldSquad {
            main: away_squad.main_squad.iter().map(|p| p.player_id).collect(),
            substitutes: away_squad.substitutes.iter().map(|p| p.player_id).collect(),
        };

        let (players_on_field, substitutes) = setup_player_on_field(home_squad, away_squad);

        Field {
            width,
            height,
            ball: Ball::with_coord(width as f32 / 2.0, height as f32 / 2.0),
            players: players_on_field,
            substitutes,
            home_players,
            away_players,
        }
    }

    pub fn write_match_positions(&self, result: &mut FootballMatchResult, timestamp: u64) {
        // player positions
        self.players.iter().for_each(|player| {
            result.position_data.add_player_positions(
                player.player_id,
                timestamp,
                player.position.x,
                player.position.y,
            );
        });

        // player positions
        self.substitutes.iter().for_each(|sub_player| {
            result.position_data.add_player_positions(
                sub_player.player_id,
                timestamp,
                sub_player.position.x,
                sub_player.position.y,
            );
        });

        // write positions
        result.position_data.add_ball_positions(
            timestamp,
            self.ball.position.x,
            self.ball.position.y,
        );
    }
}

fn setup_player_on_field(
    home_squad: TeamSquad,
    away_squad: TeamSquad,
) -> (Vec<MatchPlayer>, Vec<MatchPlayer>) {
    let mut players_on_field: Vec<MatchPlayer> = Vec::with_capacity(22);
    let mut substitutes: Vec<MatchPlayer> = Vec::with_capacity(30);

    // home
    home_squad
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
                        home_player.position = FieldPosition::new(*x as f32, *y as f32);
                        home_player.start_position = FieldPosition::new(*x as f32, *y as f32);

                        players_on_field.push(home_player);
                    }
                });
        });

    home_squad
        .substitutes
        .into_iter()
        .for_each(|mut home_player| {
            home_player.position = FieldPosition::new(1.0, 1.0);
            substitutes.push(home_player);
        });

    // away
    away_squad
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
                        away_player.position = FieldPosition::new(*x as f32, *y as f32);
                        away_player.start_position = FieldPosition::new(*x as f32, *y as f32);

                        players_on_field.push(away_player);
                    }
                });
        });

    away_squad
        .substitutes
        .into_iter()
        .for_each(|mut away_player| {
            away_player.position = FieldPosition::new(1.0, 1.0);
            substitutes.push(away_player);
        });

    (players_on_field, substitutes)
}
