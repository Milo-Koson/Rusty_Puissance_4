use std::fmt::Display;
use std::ptr::null;
use macroquad::prelude::*;

use std::sync::mpsc::{channel, Sender, Receiver, RecvError, TryRecvError};
use std::thread::{ self, JoinHandle };
use crate::connect_4_error::{Connect4Error, Connect4Result};

use crate::game_manager::GameManager;
mod game_manager;

use crate::timer_manager::TimerManager;
mod timer_manager;
mod connect_4_error;

const WINDOW_SIZE: i32 = 500;

trait ConnectFourThreadObject {

    // Déclenchement d'un timeout par la fin du timer
    fn timeout(&mut self);

    // Fin du jeu détecté par le game_manager, a pour objectif d'alerter les objets liés au timer
    fn end_game(&mut self) -> Result<(), Connect4Error>;

    // dyn Error => Dans une Box . map_err () => Wrapper le type d'erreur
    // Quitte le thread associé à l'objet et le jeu
    fn destroy(&self);
}

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


fn init_timer_manager(tx_timer: Sender<Event>, rx_game_manager: Receiver<Event>, tx_player_names: Sender<String>)
    -> Result<JoinHandle<Result<(), Connect4Error>>, Connect4Error> {

    let game_manager_thread = thread::spawn(move || -> Result<(), Connect4Error> {

        // Crée le game manager
        let mut game_manager = GameManager::new(tx_timer, rx_game_manager, tx_player_names);

        // Boucle principale de la gestion du jeu
        game_manager.run_game()?;
        Ok(())
    });

    return Ok(game_manager_thread)
}

// Évènements échangés entre les objets
#[derive(Debug)]
pub enum Event {
    PlayerChange,
    Timeout,
    End
}

#[derive(Debug)]
pub enum EventTimerTick {
    Start = 0,
    Pause,
    End
}

#[macroquad::main(window_conf)]
async fn main() -> Connect4Result<()> {

    // Créer les canaux entre game manager et timer manager 
    let (tx_timer, rx_timer) = channel::<Event>();
    let (tx_game_manager, rx_game_manager) = channel::<Event>();
    let (tx_player_names, rx_player_names) = channel::<String>();

    println!("Starting game ...");

    // Création du thread du game manager
    let game_manager_thread;
    match init_timer_manager(tx_timer, rx_game_manager, tx_player_names) {
        Ok(game_manager_thread_recv) => game_manager_thread = game_manager_thread_recv,
        Err(connect_4_error) => return Err(connect_4_error)
    }

    let mut name_player_1 = "".to_string();
    let mut name_player_2 = "".to_string();

    // Récupère le nom du joueur 1 donné par game_manager, par réception bloquante du canal.
    match rx_player_names.recv() {
        Ok(name_player_1_recv) => name_player_1 = name_player_1_recv,
        Err(_) => return Err(Connect4Error::ChannelRecv)
    }

    // Récupère le nom du joueur 2 donné par game_manager, par réception bloquante du canal.
    match rx_player_names.recv() {
        Ok(name_player_2_recv) => {name_player_2 = name_player_2_recv;},
        Err(_) => return Err(Connect4Error::ChannelRecv)
    }

    let mut timer_manager = TimerManager::new(name_player_1, name_player_2, tx_game_manager);

    timer_manager.start();

    while !timer_manager.is_end_game() {

        match timer_manager.run().await? {
            () => {}
            _ => return Err(Connect4Error::GraphicalTimerError)
        }

        // Si un endgame a déjà été signalé par temps écoulé, on ve vérifie pas de message dans le
        // canal du state manager
        // Vérifie si endgame par victoire, envoyé par state manager
        let response_from_state_manager = rx_timer.try_recv();

        match response_from_state_manager {
            Ok(Event::PlayerChange) => timer_manager.change_player(),
            Ok(Event::End) => {
                println!("ENVOI ENDGAME");
                // Envoi la fin du jeu au timer manager
                match timer_manager.end_game() {
                    Ok(()) => {}
                    _ => return Err(Connect4Error::ChannelRecv)
                }
            },
            _ => {}
        }
        next_frame().await;
    }

    timer_manager.destroy();
    println!("Main - END GAME");

    // Attente du thread de game manager
    let _ = game_manager_thread.join();

    // Quitte le programme avec un résultat satisfaisant
    Ok(())
}

