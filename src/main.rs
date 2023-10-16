use macroquad::prelude::*;

use std::sync::mpsc::channel;

use std::thread::{self, current};
use std::time::Duration;

use crate::game_manager::GameManager;
use crate::timer_manager::TimerManager;

mod game_manager;
mod timer_manager;

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

fn init_timer_manager() {

}

#[derive(Debug)]
pub enum Event {
    PlayerChange,
    End
}

#[macroquad::main(window_conf)]
async fn main() {

    let camera = Camera2D {
        ..Default::default()
    };
    
    // Fixe le centre de la camera aux coordonnées 0 x-axis and 0 y-axis. 
    set_camera(&camera);

    // Créer les canaux entre game manager et timer manager 
    let (tx_timer, rx_timer) = channel::<Event>();
    let (tx_game_manager, rx_game_manager) = channel::<Event>();
    
    // Modifie les noms des joueurs avec les noms saisis
    let mut game_manager = GameManager::new(tx_timer, rx_game_manager);

    println!("Starting game ...");
    // Run game manager
    let (game_started, (name_player_1, name_player_2)) = game_manager.get_game_information();

    let thread_game_manager = thread::spawn(move || {
        game_manager.run_game();
    });
    
    if game_started {
        // Si partie lancée, on démarre timer manager
        let mut timer_manager = TimerManager::new(&name_player_1, &name_player_2, tx_game_manager);
        timer_manager.start();
        let mut end_game = false;

        while !end_game {

            end_game = timer_manager.run().await;
    
            // Si un endgame a déjà été signalé par temps écoulé, on ve vérifie pas de message dans le
            // canal du state manager
            if !end_game {
                // Vérifie si endgame par victoire, envoyé par state manager
                let response_from_state_manager = rx_timer.try_recv();
    
                match response_from_state_manager {
                    Ok(Event::PlayerChange) => timer_manager.change_player(),
                    Ok(Event::End) => {
                        println!("Réponse de state manager : {:?}", response_from_state_manager);
                        end_game = true;
                    },
                    _ => print!("")
                }
            }
            next_frame().await;
        }
        
        timer_manager.stop();
        println!("Main - END GAME");
    } else {
        println!("Game not started ...");
    }

    let _ = thread_game_manager.join();

}

