use std::sync::mpsc::{Sender, Receiver};

use self::game_data::GameData;
mod game_data;

use crate::Event;

pub struct GameManager {
    game_data: GameData,
    tx_timer: Sender<Event>,
    rx_timer: Receiver<Event>
}

impl GameManager {
    pub fn new(tx_timer: Sender<Event>, rx_timer: Receiver<Event>) -> GameManager {

        // Demande aux joueurs de saisir leurs noms
        GameManager {
            game_data: GameData::new(),
            tx_timer,
            rx_timer
        }
    }

    pub fn run_game(&mut self) {
        // Vérifie s'il y a un match nul ou une victoire
        while !self.game_data.game_over {
            self.game_data.play_game();

            if self.game_data.is_game_draw() {
                println!("Game draw - Endgame");
            }

            // Actualise le joueur l'état de jeu et le joueur courant en cas de victoire
            if self.game_data.is_game_over() {
                self.game_data.game_over = true;
                self.game_data.current_player = 1 - self.game_data.current_player;
                
                // Envoi au timer de terminer la partie
                let _ = self.tx_timer.send(Event::End);

            } else {
                // Envoi au timer de changer de joueur
                let _ = self.tx_timer.send(Event::PlayerChange);
            }
        }

        // Affiche le gagnant
        self.game_data.display();
        println!("Le gagnant est : {} ", self.game_data.get_name());
        println!("Fin du jeu !");
    }

    pub fn get_game_information(&self) -> (bool, (String, String)) {
        if self.game_data.game_over {
            return (false, ("".to_string(), "".to_string()));
        } 
        (true, self.game_data.get_player_names())
    }
}
