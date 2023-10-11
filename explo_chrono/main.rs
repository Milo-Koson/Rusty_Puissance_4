use std::{f64::consts::PI, thread, time::Duration, sync::mpsc::channel};

use macroquad::prelude::*;

mod clock_window;

const WINDOW_SIZE: i32 = 500;
const WINDOW_MIDDLE: f32 = 0.;

const START_TIME_MINUTES: f64 = 1.;
const START_TIME_SECONDS: f64 = 2.;

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

fn who_is_current_player(current_player: &Player) -> i8 {
    if *current_player == Player::Player1 {
        return 1;
    } else {
        return 2;
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

    // Waiting tick thread
    let _ = tx_to_timer.send("Go");

    let mut count_pause = 0;
    let mut first_pause = 0;
    let mut second_pause = 0;

    while !end_game {

        // Background elements
        clock_window::display_bg();

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

        // Display selection player
        clock_window::display_selection_player(who_is_current_player(&players.current_player));

        // Display players informations 
        clock_window::display_players(&players.player1.minutes.to_string(), &players.player1.seconds.to_string(), 
            &players.player2.minutes.to_string(), &players.player2.seconds.to_string());
        
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
