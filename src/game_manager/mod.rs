use std::sync::mpsc::{Sender, Receiver};

use self::game_data::GameData;
mod game_data;

use crate::{ConnectFourThreadObject, Event};
use crate::connect_4_error::Connect4Error;

pub struct GameManager {
    game_data: GameData,
    tx_timer: Sender<Event>,
    rx_timer: Receiver<Event>
}

impl GameManager {
    pub fn new(tx_timer: Sender<Event>, rx_timer: Receiver<Event>, tx_player_names: Sender<String>) -> GameManager {

        // Crée le game data qui contient toutes les informations du jeu
        let game_data = GameData::new();

        // Envoi les informations du jeu au timer.
        let _ = tx_player_names.send(game_data.get_player_names(1));
        let _ = tx_player_names.send(game_data.get_player_names(2));

        // Demande aux joueurs de saisir leurs noms
        GameManager {
            game_data,
            tx_timer,
            rx_timer
        }
    }

    pub fn run_game(&mut self) -> Result<(), Connect4Error> {
        // Vérifie s'il y a un match nul ou une victoire
        while !self.game_data.game_over {
            self.game_data.play_game()?;

            // Check timer rep 
            // Si timeout, on demande de quitter
            let response_timer = self.rx_timer.try_recv();
            match response_timer {
                Ok(Event::Timeout) => {
                    self.timeout();
                    return Ok(())
                },
                Ok(_) => {},
                Err(_) => {},
            }

            if self.game_data.is_game_draw() {
                println!("Game draw - Endgame");
                // TODO Envoi info endgame aux autres objets
                self.end_game()?;
            }

            // Actualise le joueur l'état de jeu et le joueur courant en cas de victoire
            if self.game_data.is_game_over() {
                self.game_data.game_over = true;
                self.game_data.current_player = 1 - self.game_data.current_player;

                self.end_game()?;

            } else {
                // Envoi au timer de changer de joueur
                let _ = self.tx_timer.send(Event::PlayerChange);
            }
        }

        self.destroy();
        Ok(())
    }

    /*
    pub fn get_game_information(&self) -> (bool, (String, String)) {
        if self.game_data.game_over {
            return (false, ("".to_string(), "".to_string()));
        } 
        (true, self.game_data.get_player_names())
    }
    */

}

impl ConnectFourThreadObject for GameManager {
    fn timeout(&mut self) {
        // Time out reçu de timer, on demande de quitter
        println!("Merci d'avoir joué, aurevoir !");
        self.game_data.timeout();
    }

    // Fin du jeu détecté par le game_manager, alerte le timer_manager
    fn end_game(&mut self) -> Result<(), Connect4Error> {
        // Envoi au timer de terminer la partie
        let _ = self.tx_timer.send(Event::End);
        Ok(())
    }

    fn destroy(&self) {
        // Affiche le gagnant
        self.game_data.display();
        println!("Le gagnant est : {} ", self.game_data.get_current_player_name());
        println!("Fin du jeu !");
    }

}
