use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{
    prelude::*,
    rand::{srand, ChooseRandom},
};

const PLAYER_SPEED: f32 = 4.0;
const PLAYER_HEIGHT: f32 = 150.0;
const BOUNCE_DISTANCE: f32 = 70.0;
const START_POSITION_X: f32 = 395.0;
const START_BALL_SPEED: f32 = 3.7;

fn generate_name() -> String {
    let first_syllables = vec![
        "An", "El", "Is", "Lin", "So", "Sa", "A", "No", "Ky", "Dy", "Oce",
    ];
    let syllables = vec![
        "mo", "sa", "shi", "ro", "lin", "la", "ki", "na", "ra", "kai", "mi", "sy", "an",
    ];

    let first_syllable = first_syllables.choose().unwrap();

    let mut name = String::new();

    name += first_syllable;

    let number_of_second_syllables = vec![2, 3, 4].choose().unwrap().to_owned();
    for _ in 0..number_of_second_syllables {
        let syllable = syllables.choose().unwrap();

        name += syllable;
    }
    name
}

#[macroquad::main("BasicShapes")]
async fn main() {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    );

    let background = load_texture("pong.png").await.unwrap();
    let win_image = load_texture("win.png").await.unwrap();

    let mut ball_speed: f32 = START_BALL_SPEED;

    let mut ball_x = START_POSITION_X;
    let mut ball_y = 130.0;
    let mut ball_speed_x = ball_speed;
    let mut ball_speed_y = -3.0;

    let mut player_1 = 150.0;
    let mut player_2 = 140.0;
    let over_line = 30.0;
    let under_line = 570.0;

    let mut player_points_1 = 0;
    let mut player_points_2 = 0;

    let player_1_name = generate_name();
    let player_2_name = generate_name();

    loop {
        clear_background(WHITE);

        if player_points_1 > 4 || player_points_2 > 4 {
            draw_texture_ex(
                win_image,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            if player_points_1 > 4 {
                draw_text(
                    &format!("GRATTIS {player_1_name} VANN"),
                    200.0,
                    250.0,
                    35.0,
                    VIOLET,
                );
            }
            if player_points_2 > 4 {
                draw_text(
                    &format!("GRATTIS {player_2_name} VANN"),
                    200.0,
                    250.0,
                    35.0,
                    DARKPURPLE,
                );
            }
        } else {
            draw_texture_ex(
                background,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            // update ball position
            ball_x += ball_speed_x;
            ball_y += ball_speed_y;

            // studskod
            if ball_x > screen_width() - BOUNCE_DISTANCE {
                if player_2 < ball_y && player_2 + PLAYER_HEIGHT > ball_y {
                    ball_speed_x = -ball_speed;
                    ball_speed += 0.5;
                } else {
                    ball_x = START_POSITION_X;
                    ball_speed = START_BALL_SPEED;
                    ball_speed_x = -ball_speed;
                    player_points_1 += 1;
                }
            }
            if ball_x < BOUNCE_DISTANCE + 10.0 {
                if player_1 < ball_y && player_1 + PLAYER_HEIGHT > ball_y {
                    ball_speed_x = ball_speed;
                    ball_speed += 0.5;
                } else {
                    ball_speed_x = ball_speed;
                    ball_speed = START_BALL_SPEED;
                    ball_x = START_POSITION_X;
                    player_points_2 += 1;
                }
            }
            if ball_y < over_line + 15.0 {
                ball_speed_y = ball_speed;
            }
            if ball_y > under_line - 12.0 {
                ball_speed_y = -ball_speed;
            }

            draw_text(
                &format!(
                    "{player_1_name}: {player_points_1}    {player_2_name}: {player_points_2}"
                ),
                200.0,
                20.0,
                10.0,
                DARKGRAY,
            );

            // player input
            if macroquad::input::is_key_down(KeyCode::W) {
                player_1 -= PLAYER_SPEED;
            }
            if macroquad::input::is_key_down(KeyCode::S) {
                player_1 += PLAYER_SPEED;
            }

            if macroquad::input::is_key_down(KeyCode::Up) {
                player_2 -= PLAYER_SPEED;
            }
            if macroquad::input::is_key_down(KeyCode::Down) {
                player_2 += PLAYER_SPEED;
            }

            draw_circle(ball_x, ball_y, 20.0, VIOLET);
            draw_line(50.0, player_1, 50.0, player_1 + PLAYER_HEIGHT, 10.0, BLACK);
            draw_line(
                750.0,
                player_2,
                750.0,
                player_2 + PLAYER_HEIGHT,
                10.0,
                BLACK,
            );
            draw_line(80.0, over_line, 720.0, over_line, 10.0, BLACK); //övre linjen
            draw_line(80.0, under_line, 720.0, under_line, 10.0, BLACK); //undre linjen
            draw_line(395.0, 30.0, 395.0, 570.0, 10.0, BLACK); //mitten linjen
        }
        next_frame().await
    }
}
