use std::f64::consts::PI;
use macroquad::prelude::*;
use crate::connect_4_error::Connect4Error;

const WINDOW_MIDDLE: f32 = 0.;

const X_TIMER_PLAYER_1: f32 = -0.8;
const X_TIMER_PLAYER_2: f32 = 0.2;
const Y_TIMER_PLAYERS: f32 = 0.7;

const WIDTH_TIMER_PLAYERS: f32 = 0.6;
const HEIGHT_TIMER_PLAYERS: f32 = 0.25;

const MARGIN_SELECTION_PLAYER: f32 = 0.03;

const TIMER_RADIUS: f32 = 0.9;
const NEEDLES_RADIUS: f32 = TIMER_RADIUS - 0.08;

pub struct TimerGraphics {
    name_player_1: String,
    name_player_2: String
}

impl TimerGraphics {
    pub fn new(name_player_1: String, name_player_2: String) -> TimerGraphics {
        let camera = Camera2D {
            ..Default::default()
        };

        // Fixe le centre de la camera aux coordonnées 0 dans l'axe x et 0 dans l'axe y.
        set_camera(&camera);

        // Retourne la camera
        TimerGraphics {
            name_player_1,
            name_player_2
        }
    }

    pub async fn update_window(&self, p_1_min: f64, p_1_sec: f64, p_2_min: f64, p_2_sec: f64, id_current_player: i8)
        -> Result<(), Connect4Error> {

        display_bg().await?;
        display_selection_player(id_current_player);
        self.display_players(p_1_min, p_1_sec, p_2_min, p_2_sec);
        displayer_needles(if id_current_player == 1 { p_1_sec } else { p_2_sec });
        Ok(())
    }


    pub fn display_players(&self, player_1_minutes: f64, player_1_seconds: f64, player_2_minutes: f64, player_2_seconds: f64) {
        // Player 1 name
        draw_text_ex( "P1:", -0.95, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( &self.name_player_1, -0.78, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_1, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Player 1 digital minutes
        draw_text_ex( &player_1_minutes.to_string(), X_TIMER_PLAYER_1,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Two-points
        draw_text_ex( ":", X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 2. / 5.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Player 1 digital seconds
        draw_text_ex( &player_1_seconds.to_string(), X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 4. / 7.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    
        // Player 2 name
        draw_text_ex( "P2:", 0.05, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( &self.name_player_2, 0.22, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_2, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Player 2 digital minutes
        draw_text_ex( &player_2_minutes.to_string(), X_TIMER_PLAYER_2,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Two-points
        draw_text_ex( ":", X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS*2./5.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Player 2 digital seconds
        draw_text_ex( &player_2_seconds.to_string(), X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS * 4. / 7.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    }
}

//time: f32
fn get_params_time() -> TextParams<'static> {
    //let rotation = time*(1./12. as f32)*2.*PI as f32;
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.15);
    TextParams { font_size, font_scale,  //rotation: rotation,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

fn get_params_players_name() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.12);
    TextParams { font_size, font_scale,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

fn get_params_players_times() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.3);
    TextParams { font_size, font_scale,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

pub async fn display_bg() -> Result<(), Connect4Error>{

    // Display each time
    let radius = 0.55;
    let radius_half = radius / 2.;
    let radius_three_quarter = radius*150./ 180.;
    let half_size_time = 0.03;

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

    Ok(())
}

pub fn display_selection_player(current_player: i8) {
    // Player designation
    if current_player == 1 {
        draw_rectangle(X_TIMER_PLAYER_1-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER,
                       WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.), HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.),
                       Color::from_rgba(25, 116, 44, 255));
    } else {
        draw_rectangle(X_TIMER_PLAYER_2-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER,
                       WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.), HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.),
                       Color::from_rgba(25, 116, 44, 255));
    }
}

pub fn displayer_needles(current_player_seconds: f64) {

    // Compute the angular seconds.
    let angular = current_player_seconds*(PI/30.) - (PI/2.);

    // External circle timer
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (TIMER_RADIUS+0.03)/2., BLACK);
    // Internal circle timer
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (TIMER_RADIUS)/2., Color::from_rgba(163, 207, 207, 255));
    // Middle circle timer
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, 0.02, BLACK);

    // Display minutes needle
    draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE,
              (NEEDLES_RADIUS*angular.cos() as f32)/2., (NEEDLES_RADIUS*angular.sin() as f32)/2.,
              0.025, BLACK);

    // Display seconds needle
    draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE,
              (NEEDLES_RADIUS*angular.cos() as f32)/2., (NEEDLES_RADIUS*angular.sin() as f32)/2.,
              0.025, BLACK);
}

