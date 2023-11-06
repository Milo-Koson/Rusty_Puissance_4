use std::f64::consts::PI;
use macroquad::prelude::*;
use crate::connect_4_error::Connect4Error;

// Constante du milieu de la fenêtre du timer.
const WINDOW_MIDDLE: f32 = 0.;

// Constantes pour régler la position graphique des temps des joueurs.
const X_TIMER_PLAYER_1: f32 = -0.8;
const X_TIMER_PLAYER_2: f32 = 0.2;
const Y_TIMER_PLAYERS: f32 = 0.7;

// Constantes des tailles des temps des joueurs.
const WIDTH_TIMER_PLAYERS: f32 = 0.6;
const HEIGHT_TIMER_PLAYERS: f32 = 0.25;

// Épaisseur du rectangle de sélection des joueurs.
const MARGIN_SELECTION_PLAYER: f32 = 0.03;

// Constantes pour régler les aiguilles du timer.
const TIMER_RADIUS: f32 = 0.9;
const NEEDLES_RADIUS: f32 = TIMER_RADIUS - 0.08;

/**
Structure qui regroupe les noms des joueurs à afficher sur le timer.
*/
pub struct TimerGraphics {
    name_player_1: String,
    name_player_2: String
}

/**
Implémentation des fonctions graphiques du timer.
*/
impl TimerGraphics {
    /**
    Création d'une instance de TimerGraphics prenant en paramètre les noms des joueurs.
    Il a aussi pour rôle de paramétrer la caméra utilisé par macroquad pour avoir des coordonnées
    centrées en 0,0 au milieu de la fenêtre.
    */
    pub fn new(name_player_1: String, name_player_2: String) -> TimerGraphics {
        // Crée une instance de caméra 2D (de macroquad).
        let camera = Camera2D {
            ..Default::default()
        };

        set_camera(&camera);
        TimerGraphics {
            name_player_1,
            name_player_2
        }
    }

    /**
    Met à jour toutes les entités graphiques de la fenêtre du timer : fond du timer, noms et temps
    des joueurs, sélection du joueur courant, cadrant et aiguille du timer.
    */
    pub async fn update_window(&self, p_1_min: f64, p_1_sec: f64, p_2_min: f64, p_2_sec: f64, id_current_player: i8)
        -> Result<(), Connect4Error> {

        // Mise à jour du fond d'écran (avec les numéro du cadrant).
        display_bg().await?;
        // Mise à jour de la sélection du courant.
        display_selection_player(id_current_player);
        // Mise à jour des noms des joueurs avec leurs temps.
        self.display_players(p_1_min, p_1_sec, p_2_min, p_2_sec);
        // Mise à jour de l'aiguille du timer.
        display_needles(if id_current_player == 1 { p_1_sec } else { p_2_sec },
                        if id_current_player == 1 { p_1_min } else { p_2_min });
        Ok(())
    }

    /**
    Affiche les noms de joueurs avec leurs temps (comprend le signe ':' entre les minutes et secondes).
    */
    pub fn display_players(&self, player_1_minutes: f64, player_1_seconds: f64, player_2_minutes: f64, player_2_seconds: f64) {
        // Nom du joueur 1
        draw_text_ex( "P1:", -0.95, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( &self.name_player_1, -0.78, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_1, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Minutes du joueur 1
        draw_text_ex( &player_1_minutes.to_string(), X_TIMER_PLAYER_1,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // : du joueur 1
        draw_text_ex( ":", X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 2. / 5.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Secondes du joueur 1
        draw_text_ex( &player_1_seconds.to_string(), X_TIMER_PLAYER_1 + WIDTH_TIMER_PLAYERS * 4. / 7.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());

        // Nom du joueur 2
        draw_text_ex( "P2:", 0.05, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_text_ex( &self.name_player_2, 0.22, Y_TIMER_PLAYERS - 0.05, get_params_players_name());
        draw_rectangle(X_TIMER_PLAYER_2, Y_TIMER_PLAYERS, WIDTH_TIMER_PLAYERS, HEIGHT_TIMER_PLAYERS, WHITE);
        // Minutes du joueur 2
        draw_text_ex( &player_2_minutes.to_string(), X_TIMER_PLAYER_2,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // : du joueur 2
        draw_text_ex( ":", X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS*2./5.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
        // Secondes du joueur 2
        draw_text_ex( &player_2_seconds.to_string(), X_TIMER_PLAYER_2 + WIDTH_TIMER_PLAYERS * 4. / 7.,
                      Y_TIMER_PLAYERS + HEIGHT_TIMER_PLAYERS * 4. / 5., get_params_players_times());
    }
}

/**
Renvoie les paramètres graphiques (utilisés pour macroquad) pour afficher les numéros du cadrant.
*/
fn get_params_digits() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.15);
    TextParams { font_size, font_scale,  //rotation: rotation,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

/**
Renvoie les paramètres graphiques (utilisés pour macroquad) pour afficher les noms des joueurs.
 */
fn get_params_players_name() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.12);
    TextParams { font_size, font_scale,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

/**
Renvoie les paramètres graphiques (utilisés pour macroquad) pour afficher les temps des joueurs.
 */
fn get_params_players_times() -> TextParams<'static> {
    let (font_size, font_scale, font_aspect) = camera_font_scale(0.3);
    TextParams { font_size, font_scale,
        font_scale_aspect: font_aspect, color: BLACK, ..Default::default()}
}

/**
Rafraichie la page (retour en fond gris) et affichage du des numéro du cadrant du timer.
*/
pub async fn display_bg() -> Result<(), Connect4Error> {

    // Propriétés graphique du cadrant.
    let radius = 0.55;
    let radius_half = radius / 2.;
    let radius_three_quarter = radius*150./ 180.;
    let half_size_time = 0.03;

    // Rafraichie le fond de la fenêtre.
    clear_background(LIGHTGRAY);

    // Affichage de chaque numéro du cadrant.
    draw_text_ex("1", radius_half - half_size_time, -radius_three_quarter + half_size_time, get_params_digits());//time_index as f32
    draw_text_ex("2", radius_three_quarter- half_size_time, -radius_half + half_size_time, get_params_digits());
    draw_text_ex("3", radius- half_size_time * 2., WINDOW_MIDDLE, get_params_digits());
    draw_text_ex("4", radius_three_quarter- half_size_time, radius_half + half_size_time, get_params_digits());
    draw_text_ex("5", radius_half- half_size_time, radius_three_quarter + half_size_time, get_params_digits());
    draw_text_ex("6", WINDOW_MIDDLE- half_size_time, radius + half_size_time, get_params_digits());
    draw_text_ex("7", -radius_half- half_size_time, radius_three_quarter + half_size_time, get_params_digits());
    draw_text_ex("8", -radius_three_quarter- half_size_time, radius_half + half_size_time, get_params_digits());
    draw_text_ex("9", -radius- half_size_time, WINDOW_MIDDLE + half_size_time, get_params_digits());
    draw_text_ex("10", -radius_three_quarter - half_size_time * 3., -radius_half + half_size_time, get_params_digits());
    draw_text_ex("11", -radius_half- half_size_time * 2., -radius_three_quarter + half_size_time, get_params_digits());
    draw_text_ex("12", WINDOW_MIDDLE- half_size_time * 2., -radius + half_size_time, get_params_digits());

    Ok(())
}

/**
Affichage de la sélection du joueur courant en encadrant son nom.
*/
pub fn display_selection_player(current_player: i8) {
    // Si le joueur courant est le joueur 1
    if current_player == 1 {
        draw_rectangle(X_TIMER_PLAYER_1-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER,
                       WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.), HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.),
                       Color::from_rgba(25, 116, 44, 255));
    }
    // Sinon, c'est le joueur 2
    else {
        draw_rectangle(X_TIMER_PLAYER_2-MARGIN_SELECTION_PLAYER, Y_TIMER_PLAYERS-MARGIN_SELECTION_PLAYER,
                       WIDTH_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.), HEIGHT_TIMER_PLAYERS+(MARGIN_SELECTION_PLAYER*2.),
                       Color::from_rgba(25, 116, 44, 255));
    }
}

/**
Affichage de l'aiguille des secondes du timer.
Source du calcul de l'angle de l'aiguille du timer :
https://www.thecrazyprogrammer.com/2014/10/make-analog-clock-in-c-using-graphics.html
*/
pub fn display_needles(current_player_seconds: f64, current_player_minutes: f64) {

    // Calcule de l'angle de l'aiguille des secondes.
    let angular_sec = current_player_seconds*(PI/30.) - (PI/2.);
    let angular_min = current_player_minutes*(PI/30.) - (PI/2.);

    // Cercle externe du cadrant du timer.
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (TIMER_RADIUS+0.03)/2., BLACK);
    // Cercle interne du cadrant du timer.
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, (TIMER_RADIUS)/2., Color::from_rgba(163, 207, 207, 255));
    // Cercle du milieu du cadrant du timer.
    draw_circle(WINDOW_MIDDLE, WINDOW_MIDDLE, 0.02, BLACK);

    // Affichage de l'aiguille des secondes
    draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE,
              (NEEDLES_RADIUS*angular_sec.cos() as f32)/2., (NEEDLES_RADIUS*angular_sec.sin() as f32)/2.,
              0.025, BLACK);

    // Affichage de l'aiguille des minutes
    draw_line(WINDOW_MIDDLE, WINDOW_MIDDLE,
              (NEEDLES_RADIUS/2.*angular_min.cos() as f32)/2., (NEEDLES_RADIUS/2.*angular_min.sin() as f32)/2.,
              0.025, BLACK);
}

