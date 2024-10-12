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

const INNER_FIELD_WIDTH: f32 = 840.0;
const INNER_FIELD_HEIGHT: f32 = 545.0;

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let width = screen_width();
    let height = screen_height();

    let window_aspect_ratio = width / height;
    let field_aspect_ratio = INNER_FIELD_WIDTH / INNER_FIELD_HEIGHT;

    let (field_width, field_height, scale) = if window_aspect_ratio > field_aspect_ratio {
        let scale = height / INNER_FIELD_HEIGHT;
        (INNER_FIELD_WIDTH * scale, height, scale)
    } else {
        let scale = width / INNER_FIELD_WIDTH;
        (width, INNER_FIELD_HEIGHT * scale, scale)
    };

    let offset_x = (width - field_width) / 2.0;
    let offset_y = (height - field_height) / 2.0;

    let home_squad = get_home_squad();
    let away_squad = get_away_squad();

    let players = MatchPlayerCollection::from_squads(&home_squad, &away_squad);

    let mut field = MatchField::new(
        INNER_FIELD_WIDTH as usize,
        INNER_FIELD_HEIGHT as usize,
        home_squad,
        away_squad,
    );

    let field_size = field.size.clone();

    let score = Score::new(1, 2);

    let mut context = MatchContext::new(&field_size, players, score);

    let mut current_frame = 0u64;

    let mut match_data = MatchPositionData::new();

    let mut left_mouse_pressed = false;

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

        draw_goals(offset_x, offset_y, &context, field_width, scale);
        draw_players(offset_x, offset_y, &field, scale);

        draw_ball(offset_x, offset_y, &field.ball, scale);

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

        left_mouse_pressed = is_mouse_button_down(MouseButton::Left);

        if left_mouse_pressed {
            thread::sleep(Duration::from_millis(100));
        }

        next_frame().await;
    }
}

const TRACKING_PLAYER_ID: u32 = 102;

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

#[cfg(target_os = "macos")]
const WINDOW_WIDTH: i32 = 1300;
#[cfg(target_os = "macos")]
const WINDOW_HEIGHT: i32 = 1000;

#[cfg(target_os = "windows")]
const WINDOW_WIDTH: i32 = 2436;
#[cfg(target_os = "windows")]
const WINDOW_HEIGHT: i32 = 1902;

fn window_conf() -> Conf {
    Conf {
        window_title: "FootballSimulatorTesting".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
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

fn draw_goals(offset_x: f32, offset_y: f32, context: &MatchContext, field_width: f32, scale: f32) {
    let color = Color::from_rgba(0, 184, 186, 255);

    draw_line(
        offset_x,
        offset_y + context.goal_positions.left.y * scale - GOAL_WIDTH * scale,
        offset_x,
        offset_y + context.goal_positions.left.y * scale + GOAL_WIDTH * scale,
        15.0,
        color,
    );

    draw_line(
        offset_x + field_width,
        offset_y + context.goal_positions.right.y * scale - GOAL_WIDTH * scale,
        offset_x + field_width,
        offset_y + context.goal_positions.right.y * scale + GOAL_WIDTH * scale,
        15.0,
        color,
    );
}

fn draw_players(offset_x: f32, offset_y: f32, field: &MatchField, scale: f32) {
    field.players.iter().for_each(|player| {
        let translated_x = offset_x + player.position.x * scale;
        let translated_y = offset_y + player.position.y * scale;

        let mut color = if player.side == Some(PlayerSide::Left) {
            Color::from_rgba(0, 184, 186, 255)
        } else {
            Color::from_rgba(208, 139, 255, 255)
        };

        if player.tactics_position == PlayerPositionType::Goalkeeper {
            color = YELLOW;
        }

        let circle_radius = 15.0 * scale;

        // Draw the player circle
        draw_circle(translated_x, translated_y, circle_radius, color);

        if player.has_ball {
            draw_circle_lines(translated_x, translated_y, circle_radius + scale, 3.0, WHITE);
        }

        // Player position
        let position = &player.tactics_position.get_short_name();
        let position_font_size = 18.0 * scale;
        let position_text_dimensions = measure_text(position, None, position_font_size as u16, 1.0);
        draw_text(
            position,
            translated_x - position_text_dimensions.width / 2.0,
            translated_y + position_text_dimensions.height / 3.0,
            position_font_size,
            BLACK,
        );

        // Player ID
        let id_text = &player.id.to_string();
        let id_font_size = 8.0 * scale;
        let id_text_dimensions = measure_text(id_text, None, id_font_size as u16, 1.0);
        draw_text(
            id_text,
            translated_x - id_text_dimensions.width / 2.0,
            translated_y + position_text_dimensions.height + id_text_dimensions.height / 2.0,
            id_font_size,
            DARKGRAY,
        );

        // Player state and distance
        let distance = distance(&field.ball.position, &player.position);
        let state_distance_text = &format!("{} ({})", player_state(player), distance);
        let state_distance_font_size = 13.0 * scale;
        let state_distance_text_dimensions = measure_text(state_distance_text, None, state_distance_font_size as u16, 1.0);
        draw_text(
            state_distance_text,
            translated_x - state_distance_text_dimensions.width / 2.5,
            translated_y + circle_radius + state_distance_text_dimensions.height + 0.0,
            state_distance_font_size,
            DARKGRAY,
        );
    });
}

fn draw_ball(offset_x: f32, offset_y: f32, ball: &Ball, scale: f32) {
    let translated_x = offset_x + ball.position.x * scale;
    let translated_y = offset_y + ball.position.y * scale;

    draw_circle(translated_x, translated_y, 7.0 * scale, ORANGE);
}
