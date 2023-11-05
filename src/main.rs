// use std::fmt::Display;
// use std::ptr::null;
use macroquad::prelude::*;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{ self, JoinHandle };
use crate::connect_4_error::{Connect4Error, Connect4Result};

use crate::game_manager::GameManager;
mod game_manager;

use crate::timer_manager::TimerManager;
mod timer_manager;
mod connect_4_error;

const WINDOW_SIZE: i32 = 500;

/**
Trait définissant les fonctions génériques au managers (game_manager et timer_manager)
*/
trait ConnectFourThreadObject {

    /**
    Déclenchement d'un timeout pour le joueur courant
    */
    fn timeout(&mut self);

    /**
    Fin du jeu détecté par le game_manager, a pour objectif d'alerter les objets liés au timer
    */
    fn end_game(&mut self) -> Result<(), Connect4Error>;

    /**
    En cas de fin de jeu, pour arrêter le thread de l'objet.
    */
    fn destroy(&self);
}

/**
Renvoie les configurations de la fenêtre du timer (utilisant macroquad).
*/
fn window_conf() -> Conf {
    Conf {
        /**
        Attribue le nom de la fenêtre du timer.
         */
        window_title: "Timer".to_owned(),
        /**
        Définie la longueur de la fenêtre, par la constante définie en haut de ce module.
        */
        window_width: WINDOW_SIZE,
        /**
        Définie la hauteur de la fenêtre, par la constante définie en haut de ce module.
         */
        window_height: WINDOW_SIZE,
        high_dpi: true,
        /**
        Permet d'autoriser la modification de la taille de la fenêtre du timer.
         */
        window_resizable: true,
        ..Default::default()
    }
}

/**
Crée le thread du game_manager en lui donnant les canaux de communication avec le time_manager et
les noms des joueurs.
*/
fn init_game_manager(tx_timer: Sender<Event>, rx_game_manager: Receiver<Event>, tx_player_names: Sender<String>)
    -> Result<JoinHandle<Result<(), Connect4Error>>, Connect4Error> {

    // Crée le thread du game_manager
    let game_manager_thread = thread::spawn(move || -> Result<(), Connect4Error> {

        // Initialise le game_manager
        let mut game_manager = GameManager::new(tx_timer, rx_game_manager, tx_player_names);

        // Lance la boucle principale de la gestion du jeu
        game_manager.run_game()?;
        Ok(())
    });

    Ok(game_manager_thread)
}

/**
Évènements échangés entre les objets
*/
#[derive(Debug)]
pub enum Event {
    /**
    Changement de joueur courant
    */
    PlayerChange,
    /**
    Temps écoulé pour le joueur courant
    */
    Timeout,
    /**
    Fin du jeu
    */
    End
}

/**
Enumération pour le timer tick (celui qui décompte le temps).
*/
#[derive(Debug)]
pub enum EventTimerTick {
    /**
    Lancement du timer tick
    */
    Start,
    /**
    Fin du jeu
    */
    End
}

/**
Fonction principale du programme.
Il est responsable du thread de timer_manager pour gérer l'affichage du timer.
*/
#[macroquad::main(window_conf)]
async fn main() -> Connect4Result<()> {

    // Crée le canal de communication pour envoyer des événements au timer_manager.
    // tx au timer_manager et rx au game_manager.
    let (tx_timer, rx_timer) = channel::<Event>();
    // Crée le canal de communication pour envoyer des événements au game_manager.
    // tx au timer_manager et rx au game_manager
    let (tx_game_manager, rx_game_manager) = channel::<Event>();
    // Crée le canal de communication pour envoyer les noms des joueurs de game_manager à
    // timer_manager pour les afficher sur le timer. tx au timer_manager et rx au game_manager.
    let (tx_player_names, rx_player_names) = channel::<String>();

    // Création du handle du thread du game manager (pour l'attendre à la fin du main).
    let game_manager_thread;
    // Lancement du thread et récupération du handle si pas d'erreur.
    match init_game_manager(tx_timer, rx_game_manager, tx_player_names) {
        Ok(game_manager_thread_recv) => game_manager_thread = game_manager_thread_recv,
        Err(connect_4_error) => return Err(connect_4_error)
    }

    // Création des noms des joueurs pour les transmettre à timer_manager
    let name_player_1;
    let name_player_2;

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

    // Création de l'instance de timer_manager.
    let mut timer_manager = TimerManager::new(name_player_1, name_player_2, tx_game_manager);

    // Démarrage de timer_manager. A pour effet de lancer le décomptage de timer_tick.
    timer_manager.start();

    // Tant que la partie est en cours
    while !timer_manager.is_end_game() {

        // Exécute le code principale de timer_manager (comprenant l'affichage de la fenêtre du
        // timer). Si une erreur survient, il s'agit de la fenêtre du timer.
        match timer_manager.run().await? {
            () => {}
        }

        // Récupère un possible événement provenant du state_manager (non bloquante).
        let response_from_state_manager = rx_timer.try_recv();

        // S'il y a un événement, on effectue l'action associé, sinon on ne fait rien.
        // Aucun événement reçu étant interprété comme une erreur, on ne considère par ce cas.
        match response_from_state_manager {
            // Changement de joueur demandé.
            Ok(Event::PlayerChange) => timer_manager.change_player(),
            // Fin du jeu (victoire/jeu nul).
            Ok(Event::End) => {
                // On déclenche l'action associé dans timer_manager pour la fin du jeu.
                match timer_manager.end_game() {
                    Ok(()) => {}
                    _ => return Err(Connect4Error::ChannelRecv)
                }
            },
            _ => {}
        }
        next_frame().await;
    }

    // Fin de la partie pour le timer_manager.
    timer_manager.destroy();

    // Attente du thread de game manager
    let _ = game_manager_thread.join();

    // Fin du programme en règle.
    Ok(())
}

