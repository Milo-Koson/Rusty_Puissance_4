use macroquad::prelude::*;

use std::sync::mpsc::channel;

use std::thread::{self, current};
use std::time::Duration;

mod players;
mod game_data;

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

fn init_timer() {

}

#[macroquad::main(window_conf)]
async fn main() {

    let camera = Camera2D {
        ..Default::default()
    };
    
    // Fixe le centre de la camera aux coordonnées 0 x-axis and 0 y-axis. 
    set_camera(&camera);

    // Demande aux joueurs de saisir leurs noms
    let (player1_name, player2_name) = players::set_player_names();
    
    // Modifie les noms des joueurs avec les noms saisis
    let mut current_game = game_data::GameData::new(player1_name, player2_name);

        // Vérifie s'il y a un match nul ou une victoire
        while !current_game.game_over {
            current_game.play_game();
            current_game.is_game_draw();
            current_game.is_game_over();

            // Actualise le joueur l'état de jeu et le joueur courant en cas de victoire
            if current_game.is_game_over() {
                current_game.game_over = true;
                current_game.current_player = 1 - current_game.current_player;
            }
        }

    // Affiche le gagnant
    current_game.display();
    println!("Le gagnant est : {} ", current_game.get_name());
    println!("Fin du jeu !");
    
}

