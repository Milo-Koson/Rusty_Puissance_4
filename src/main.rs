use macroquad::prelude::*;

use std::sync::mpsc::{channel, Sender, Receiver};

use std::thread::{ self, JoinHandle };

use crate::game_manager::GameManager;
use crate::timer_manager::TimerManager;

mod game_manager;
mod timer_manager;

trait ConnectFourThreadObject {

    // Quitte le thread associé à l'objet et le jeu
    fn stop(&self);
}

const WINDOW_SIZE: i32 = 500;

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

fn init_timer_manager(tx_timer: Sender<Event>, rx_game_manager: Receiver<Event>, tx_player_names: Sender<(String, String)>) -> JoinHandle<()> {
    thread::spawn(move || {
        // Crée le game manager
        let mut game_manager = GameManager::new(tx_timer, rx_game_manager, tx_player_names);

        // Boucle principale de la gestion du jeu
        game_manager.run_game();
    })
}

#[derive(Debug)]
pub enum Event {
    PlayerChange,
    Timeout,
    End
}

fn game_started(name_player_1: &String, name_player_2: &String) -> bool {
    if !name_player_1.is_empty() && !name_player_2.is_empty() {
        return true;
    }
    false
}

#[macroquad::main(window_conf)]
async fn main() {

    // Créer les canaux entre game manager et timer manager 
    let (tx_timer, rx_timer) = channel::<Event>();
    let (tx_game_manager, rx_game_manager) = channel::<Event>();

    let (tx_player_names, rx_player_names) = channel::<(String, String)>();

    println!("Starting game ...");

    // Création du thread du game manager
    let thread_game_manager = init_timer_manager(tx_timer, rx_game_manager, tx_player_names);

    // Récupère les informations du game manager avec les noms des joueurs, par réception bloquante du canal.
    // let (game_started, (name_player_1, name_player_2)) = game_manager.get_game_information();

    let Ok((name_player_1, name_player_2)) = rx_player_names.recv() else { panic!("Error recv names") };

    if game_started(&name_player_1, &name_player_2) {
        // Si partie lancée, on démarre timer manager
        let mut timer_manager = TimerManager::new(name_player_1, name_player_2, tx_game_manager);

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

