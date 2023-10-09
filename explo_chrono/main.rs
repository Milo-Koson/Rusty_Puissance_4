use std::{f64::consts::PI, thread, time::Duration, sync::mpsc::channel};

use macroquad::prelude::*;

const WINDOW_SIZE: i32 = 500;
const WINDOW_MIDDLE: f32 = 0.;

const X_TIMER_PLAYER_1: f32 = -0.8;
const X_TIMER_PLAYER_2: f32 = 0.2;
const Y_TIMER_PLAYERS: f32 = 0.7;

const WIDTH_TIMER_PLAYERS: f32 = 0.6;
const HEIGHT_TIMER_PLAYERS: f32 = 0.25;

const START_TIME_MINUTES: f64 = 1.;
const START_TIME_SECONDS: f64 = 2.;

// Display constants
const MARGIN_SELECTION_PLAYER: f32 = 0.03;

fn window_conf() -> Conf {
    Conf {
        window_title: "Timer".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

//time: f32
fn get_params_time() -> TextParams<'static> {
    //let rotation = time*(1./12. as f32)*2.*PI as f32;
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.15);
    let text_params = TextParams { font_size, font_scale,  //rotation: rotation,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()};
    return text_params;
}

fn get_params_players_name() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.12);
    let text_params = TextParams { font_size, font_scale, 
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()};
    return text_params;
}

fn get_params_players_times() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.3);
    let text_params = TextParams { font_size, font_scale, 
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()};
    return text_params;
}
/* 
fn deacrease_time(current_player_minutes: *mut f64, current_player_seconds: *mut f64) {
    *current_player_minutes = *current_player_minutes - 1.;
    if seconds <= 0. {
        if minutes > 0. {
            minutes -= 1.;
            seconds = 60.;
        } else {
            println!("TIME !!");
            end_game = true;
        }
    }
}*/

#[derive(Debug)]
#[derive(PartialEq)]

enum Player {
    Player1,
    Player2
}

struct TimerPlayer {
    minutes : f64,
    seconds : f64
}

struct Players {
    current_player : Player,
    player1 : TimerPlayer,
    player2 : TimerPlayer
} 

impl Players {
    
    fn tick(&mut self) -> bool {
        // Current player is player 1
        if self.current_player == Player::Player1 {

            self.player1.seconds -= 1.;
            if self.player1.seconds < 0. {
                if self.player1.minutes > 0. {
                    self.player1.minutes -= 1.;
                    self.player1.seconds = 59.;
                } else {
                    println!("TIME !!");
                    return true;
                }
            }
            return false;

        // Current player is player 2
        } else {
            self.player2.seconds -= 1.;
            if self.player2.seconds < 0. {
                if self.player2.minutes > 0. {
                    self.player2.minutes -= 1.;
                    self.player2.seconds = 59.;
                } else {
                    println!("TIME !!");
                    return true;
                }
            }
            return false;
        }
    }

    fn change_player(&mut self) {
        if self.current_player == Player::Player1 {
            self.current_player = Player::Player2;
        } else {
            self.current_player = Player::Player1;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let camera = Camera2D {
        ..Default::default()
    };
    
    // Fix camera center at 0 x-axis and 0 y-axis. 
    set_camera(&camera);

    let timer_radius = 0.9;
    let timer_seconds_radius = timer_radius - 0.08;

    // DEV 
    let mut count = 0;

    // Player properties 
    let mut players = Players {
        current_player: Player::Player1,
        player1: TimerPlayer { minutes: START_TIME_MINUTES, seconds: START_TIME_SECONDS },
        player2: TimerPlayer { minutes: START_TIME_MINUTES, seconds: START_TIME_SECONDS }
    };

    // Create channels between display and tick threads.
    let (tx_to_timer, rx_for_timer) = channel();
    let (tx_to_displayer, rx_for_displayer) = channel();

    // Tick thread timer 
    let _ = thread::spawn(move || {

        let end_game_timer = false;

        let _ = rx_for_timer.recv();
        println!("Go timer !");

        while !end_game_timer {
            if let Ok(_) = rx_for_timer.try_recv() {
                println!("PAUSE !");
                let pause_thread = thread::spawn(move || {
                    thread::sleep(Duration::from_millis(2000));
                });

                // Waiting the restart
                let _ = pause_thread.join();
            } else {
                println!("Not pause !");
            }
            thread::sleep(Duration::from_millis(500));
            println!("Tick and send !");
            let _ = tx_to_displayer.send("_");
        }
    });

    let mut end_game = false;
    
    // Display each time
    let radius = 0.55;
    let radius_half = radius / 2.;
    let radius_three_quarter = radius*150./ 180.;
    let half_size_time = 0.03;

    // Waiting tick thread
    let _ = tx_to_timer.send("Go");

    let mut count_pause = 0;
    let mut first_pause = 0;
    let mut second_pause = 0;

    while !end_game {

        // Background elements
        clear_background(LIGHTGRAY);
        draw_text_ex( "1", radius_half - half_size_time, -radius_three_quarter + half_size_time, get_params_time());//time_index as f32
        draw_text_ex( "2", radius_three_quarter- half_size_time, -radius_half + half_size_time, get_params_time());
        draw_text_ex( "3", radius- half_size_time * 2., WINDOW_MIDDLE, get_params_time());
        draw_text_ex( "4", radius_three_quarter- half_size_time, radius_half + half_size_time, get_params_time());
        draw_text_ex( "5", radius_half- half_size_time, radius_three_quarter + half_size_time, get_params_time());
        draw_text_ex( "6", WINDOW_MIDDLE- half_size_time, radius + half_size_time, get_params_time());
        draw_text_ex( "7", -radius_half- half_size_time, radius_three_quarter + half_size_time, get_params_time());
        draw_text_ex( "8", -radius_three_quarter- half_size_time, radius_half + half_size_time, get_params_time());
        draw_text_ex( "9", -radius- half_size_time, WINDOW_MIDDLE + half_size_time, get_params_time());
        draw_text_ex( "10", -radius_three_quarter - half_size_time * 3., -radius_half + half_size_time, get_params_time());
        draw_text_ex( "11", -radius_half- half_size_time * 2., -radius_three_quarter + half_size_time, get_params_time());
        draw_text_ex( "12", WINDOW_MIDDLE- half_size_time * 2., -radius + half_size_time, get_params_time());

        // DEV Simulation of player turn
        
        count_pause += 1;
        if count < 10 {
            players.current_player = Player::Player2;
        } else if count == 11 {
            if first_pause == 0 {
                first_pause = count_pause;
                let _ = tx_to_timer.send("pause");
            }
        } else if count <= 20 {
            players.current_player = Player::Player1;
        } else if count == 21 {
            if second_pause == 0 {
                second_pause = count_pause;
                let _ = tx_to_timer.send("pause");
            }
        } else if count <= 30 {
            players.current_player = Player::Player2;
        } else if count <= 40 {
            players.current_player = Player::Player1;
        } else if count <= 50 {
            players.current_player = Player::Player2;
        } else if count <= 60 {
            players.current_player = Player::Player1;
        } else if count <= 70 {
            players.current_player = Player::Player2;
        } else if count <= 80 {
            players.current_player = Player::Player1;
        } else if count <= 90 {
            players.current_player = Player::Player2;
        } else if count <= 100 {
            players.current_player = Player::Player1;
        } else {
            players.current_player = Player::Player2;
        }

        // Player designation
        if players.current_player == Player::Player1 {
            draw_rectangle(X_TIMER_PLAYER_1-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER as f32,
                WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32, HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32,
                Color::from_rgba(25, 116, 44, 255));
        } else {
            draw_rectangle(X_TIMER_PLAYER_2-MARGIN_SELECTION_PLAYER as f32, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER as f32,
                WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32, HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32,
                Color::from_rgba(25, 116, 44, 255));
        }

        // Player 1 name
        draw_text_ex( "P1:", -0.95, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( "Player 1", -0.78, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_1, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Player 1 digital minutes 
        draw_text_ex( &players.player1.minutes.to_string(), X_TIMER_PLAYER_1, 
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());
        // Two-points
        draw_text_ex( &":", X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS*2./5., 
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());
        // Player 1 digital seconds 
        draw_text_ex( &players.player1.seconds.to_string(), X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS*4./7., 
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());

        // Player 2 name
        draw_text_ex( "P2:", 0.05, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( "Player 2", 0.22, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_2, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Player 2 digital minutes 
        draw_text_ex( &players.player2.minutes.to_string(), X_TIMER_PLAYER_2,  
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());
        // Two-points
        draw_text_ex( &":", X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS*2./5.,  
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());
        // Player 2 digital seconds 
        draw_text_ex( &players.player2.seconds.to_string(), X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS*4./7., 
        Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS*4./5., get_params_players_times());
        
        if let Ok(_) = rx_for_displayer.try_recv() {
            if players.tick() {
                end_game = true;
            }
            count += 1;
        } 

        let angular: f64;

        // Compute the angular seconds. 
        if players.current_player == Player::Player1 {
            angular = players.player1.seconds*(PI/30.) - (PI/2.);
        } else {
            angular = players.player2.seconds*(PI/30.) - (PI/2.);
        }

        // External circle timer
        draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (timer_radius+0.03)/2., BLACK);
        // Internal circle timer
        draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (timer_radius)/2., Color::from_rgba(163, 207, 207, 255));
        // Middle circle timer
        draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, 0.02, BLACK);

        // Display minutes needle
        draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE, 
            (timer_seconds_radius*angular.cos() as f32)/2., (timer_seconds_radius*angular.sin() as f32)/2., 
            0.025, BLACK);
        
        // Display seconds needle
        draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE, 
            (timer_seconds_radius*angular.cos() as f32)/2., (timer_seconds_radius*angular.sin() as f32)/2., 
        0.025, BLACK);

        // Waiting 60 FPS
        let fps_desired = 1. / 60.; 
        let frame_time = get_frame_time();
        if frame_time < fps_desired {
            let time_should_wait = (fps_desired - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_should_wait as u64));
        }
        next_frame().await;
    }
}
