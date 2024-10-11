use core::r#match::ball::Ball;
use core::r#match::player::MatchPlayer;
use core::r#match::FootballEngine;
use core::r#match::MatchContext;
use core::r#match::MatchField;
use macroquad::prelude::*;
use std::thread;
use std::time::Duration;
//tactics
use core::club::player::Player;
use core::club::player::PlayerPositionType;
use core::club::team::tactics::{Tactics, TacticsPositioning};
use core::r#match::squad::TeamSquad;
use core::r#match::MatchBallLogic;
use core::r#match::MatchPlayerCollection;
use core::Vector3;
use env_logger::Env;
use std::time::Instant;

use core::r#match::PlayerSide;
use core::r#match::GOAL_WIDTH;
use core::NaiveDate;
use core::PlayerGenerator;
use core::r#match::MatchPositionData;
use core::r#match::Score;

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let width = screen_width();
    let height = screen_height();

    let field_width = 840.0;
    let field_height = 545.0;

    // Define an offset to center the field
    let offset_x = (width - field_width) / 2.0;
    let offset_y = (height - field_height) / 2.0;

    //840, 545
    // let mut ball = Ball::with_coord(field_width / 2.0, field_height / 2.0);

    let home_squad = get_home_squad();
    let away_squad = get_away_squad();

    let players = MatchPlayerCollection::from_squads(&home_squad, &away_squad);

    let mut field = MatchField::new(
        field_width as usize,
        field_height as usize,
        home_squad,
        away_squad,
    );

    let field_size = field.size.clone();

    let score = Score::new(1,  2);

    let mut context = MatchContext::new(&field_size, players, score);

    let mut current_frame = 0u64;

    let mut match_data = MatchPositionData::new();

    loop {
        current_frame += 1;

        clear_background(Color::new(255.0, 238.0, 7.0, 65.0));

        draw_rectangle_ex(
            offset_x,
            offset_y,
            field_width,
            field_height,
            DrawRectangleParams {
                color: Color::from_rgba(189, 255, 204, 255),
                offset: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
            },
        );

        let start = Instant::now();

        FootballEngine::<840, 545>::game_tick(&mut field, &mut context, &mut match_data);

        let elapsed = start.elapsed();

        draw_goals(offset_x, offset_y, &context);
        draw_players(offset_x, offset_y, &field);

        draw_ball(offset_x, offset_y, &field.ball);

        // FPS
        const average_fps_bucket_size: usize = 50;

        let mut max_fps: u128 = 0;

        let mut fps_data = [0u128; average_fps_bucket_size];

        let fps_data_current_idx = (current_frame % average_fps_bucket_size as u64) as usize;

        let elapsed_mcs = elapsed.as_micros() as u128;

        fps_data[fps_data_current_idx] = elapsed.as_micros() as u128;

        if current_frame > 100 && elapsed_mcs > max_fps {
            max_fps = elapsed_mcs;
        }

        draw_fps(offset_x, offset_y, &fps_data, max_fps);

        next_frame().await;

        thread::sleep(Duration::from_millis(50));
    }
}

const TRACKING_PLAYER_ID: u32 = 123;

pub fn get_home_squad() -> TeamSquad {
    let players = [
        get_player(101, PlayerPositionType::Goalkeeper),
        get_player(102,PlayerPositionType::DefenderLeft),
        get_player(103,PlayerPositionType::DefenderCenterLeft),
        get_player(104,PlayerPositionType::DefenderCenterRight),
        get_player(105,PlayerPositionType::DefenderRight),
        get_player(106,PlayerPositionType::MidfielderLeft),
        get_player(107,PlayerPositionType::MidfielderCenterLeft),
        get_player(108,PlayerPositionType::MidfielderCenterRight),
        get_player(109,PlayerPositionType::MidfielderRight),
        get_player(111,PlayerPositionType::WingbackLeft),
        get_player(112,PlayerPositionType::ForwardRight),
    ];

    let match_players: Vec<MatchPlayer> = players
        .iter()
        .map(|player| MatchPlayer::from_player(1, player, player.position(), player.id == TRACKING_PLAYER_ID))
        .collect();

    let home_squad = TeamSquad {
        team_id: 1,
        team_name: String::from("123"),
        tactics: Tactics::new(TacticsPositioning::T442),
        main_squad: match_players,
        substitutes: Vec::new(),
    };

    home_squad
}

pub fn get_away_squad() -> TeamSquad {
    let players = [
        get_player(113,PlayerPositionType::Goalkeeper),
        get_player(114,PlayerPositionType::DefenderLeft),
        get_player(115,PlayerPositionType::DefenderCenterLeft),
        get_player(116,PlayerPositionType::DefenderCenterRight),
        get_player(117,PlayerPositionType::DefenderRight),
        get_player(118,PlayerPositionType::MidfielderLeft),
        get_player(119,PlayerPositionType::MidfielderCenterLeft),
        get_player(120,PlayerPositionType::MidfielderCenterRight),
        get_player(121,PlayerPositionType::MidfielderRight),
        get_player(122,PlayerPositionType::ForwardLeft),
        get_player(123, PlayerPositionType::ForwardRight),
    ];

    let match_players: Vec<MatchPlayer> = players
        .iter()
        .map(|player| MatchPlayer::from_player(2, player, player.position(), player.id == TRACKING_PLAYER_ID))
        .collect();

    let away_squad = TeamSquad {
        team_id: 2,
        team_name: String::from("321"),
        tactics: Tactics::new(TacticsPositioning::T442),
        main_squad: match_players,
        substitutes: Vec::new(),
    };

    away_squad
}

fn get_player(id: u32, position: PlayerPositionType) -> Player {
    let mut player = PlayerGenerator::generate(
        1,
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        position,
        20,
    );

    player.id = id;

    player
}

fn average(numbers: &[u128]) -> u128 {
    let sum: u128 = numbers.iter().sum();
    let count = numbers.len() as u128;
    sum / count
}

fn player_state(player: &MatchPlayer) -> String {
    let state = player.state.to_string();

    let cleaned_state = state.split(':').nth(1).unwrap_or(&state).trim();

    return cleaned_state.to_string();
}

fn distance(a: &Vector3<f32>, b: &Vector3<f32>) -> usize {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt() as usize
}

pub fn is_towards_player(
    ball_position: &Vector3<f32>,
    ball_velocity: &Vector3<f32>,
    player_position: &Vector3<f32>,
) -> (bool, f32) {
    MatchBallLogic::is_heading_towards_player(ball_position, ball_velocity, player_position, 0.95)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "FootballSimulatorTesting".to_owned(),
        window_width: 1624,
        window_height: 1268,
        window_resizable: false,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

// draw

fn draw_fps(offset_x: f32, offset_y: f32, fps_data: &[u128], max_fps: u128) {
    draw_text(
        &format!("FPS AVG: {} mcs", average(&fps_data)),
        offset_x + 10.0,
        offset_y + 20.0,
        20.0,
        BLACK,
    );

    draw_text(
        &format!("FPS MAX: {} mcs", max_fps),
        offset_x + 10.0,
        offset_y + 40.0,
        20.0,
        BLACK,
    );
}

fn draw_goals(offset_x: f32, offset_y: f32, context: &MatchContext) {
    let color = Color::from_rgba(0, 184, 186, 255);

    draw_line(
        offset_x,
        offset_y + context.goal_positions.left.y - GOAL_WIDTH,
        offset_x,
        offset_y + context.goal_positions.left.y + GOAL_WIDTH,
        5.0,
        color,
    );

    draw_line(
        offset_x + context.goal_positions.right.x,
        offset_y + context.goal_positions.right.y - GOAL_WIDTH,
        offset_x + context.goal_positions.right.x,
        offset_y + context.goal_positions.right.y + GOAL_WIDTH,
        5.0,
        color,
    );
}

fn draw_players(offset_x: f32, offset_y: f32, field: &MatchField) {
    field.players.iter().for_each(|player| {
        let mut color = if player.side == Some(PlayerSide::Left) {
            Color::from_rgba(0, 184, 186, 255)
        } else {
            Color::from_rgba(208, 139, 255, 255)
        };

        if player.tactics_position == PlayerPositionType::Goalkeeper {
            color = YELLOW;
        }

        draw_text(
            &player.id.to_string(),
            offset_x + player.position.x - 8.0,
            offset_y + player.position.y - 20.0,
            12.0,
            DARKGRAY
        );

        draw_circle(
            offset_x + player.position.x,
            offset_y + player.position.y,
            16.0,
            color,
        );

        if player.has_ball {
            draw_circle_lines(
                offset_x + player.position.x,
                offset_y + player.position.y,
                16.0,
                3.0,
                WHITE,
            );
        }

        let state = &player.tactics_position.get_short_name();

        let left_offset = if state.len() == 3 { 12.0 } else { 8.0 };

        draw_text(
            state,
            offset_x + player.position.x - left_offset,
            offset_y + player.position.y + 5.0,
            19.0,
            BLACK,
        );

        draw_text(
            &format!("{}", player_state(player)),
            offset_x + player.position.x - left_offset - 15.0,
            offset_y + player.position.y + 27.0,
            15.0,
            DARKGRAY,
        );

        draw_text(
            &format!(
                "distance = {}",
                distance(&field.ball.position, &player.position)
            ),
            offset_x + player.position.x - 27.0,
            offset_y + player.position.y + 40.0,
            11.0,
            DARKGRAY,
        );
    });
}

fn draw_ball(offset_x: f32, offset_y: f32, ball: &Ball) {
    draw_circle(
        offset_x + ball.position.x,
        offset_y + ball.position.y,
        7.0,
        ORANGE,
    );
}
