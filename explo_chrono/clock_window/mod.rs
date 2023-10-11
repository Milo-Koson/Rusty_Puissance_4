
use macroquad::prelude::*;

const WINDOW_MIDDLE: f32 = 0.;

const X_TIMER_PLAYER_1: f32 = -0.8;
const X_TIMER_PLAYER_2: f32 = 0.2;
const Y_TIMER_PLAYERS: f32 = 0.7;

const WIDTH_TIMER_PLAYERS: f32 = 0.6;
const HEIGHT_TIMER_PLAYERS: f32 = 0.25;

const MARGIN_SELECTION_PLAYER: f32 = 0.03;

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

pub fn display_bg() {
    
    
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
}

pub fn display_players(player_1_minutes: &str, player_1_seconds: &str, player_2_minutes: &str, player_2_seconds: &str) {
    // Player 1 name
    draw_text_ex( "P1:", -0.95, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
    draw_text_ex( "Player 1", -0.78, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
    draw_rectangle(X_TIMER_PLAYER_1, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
    // Player 1 digital minutes 
    draw_text_ex( player_1_minutes, X_TIMER_PLAYER_1, 
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    // Two-points
    draw_text_ex( &":", X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 2. / 5., 
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    // Player 1 digital seconds 
    draw_text_ex( player_1_seconds, X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 4. / 7., 
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());

    // Player 2 name
    draw_text_ex( "P2:", 0.05, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
    draw_text_ex( "Player 2", 0.22, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
    draw_rectangle(X_TIMER_PLAYER_2, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
    // Player 2 digital minutes 
    draw_text_ex( player_2_minutes, X_TIMER_PLAYER_2,  
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    // Two-points
    draw_text_ex( &":", X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS*2./5.,  
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    // Player 2 digital seconds 
    draw_text_ex( player_2_seconds, X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS * 4. / 7., 
    Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
}

pub fn display_selection_player(current_player: i8) {
    // Player designation
    if current_player == 1 {
        draw_rectangle(X_TIMER_PLAYER_1-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER as f32,
            WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32, HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32,
            Color::from_rgba(25, 116, 44, 255));
    } else {
        draw_rectangle(X_TIMER_PLAYER_2-MARGIN_SELECTION_PLAYER as f32, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER as f32,
            WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32, HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.) as f32,
            Color::from_rgba(25, 116, 44, 255));
    }
}