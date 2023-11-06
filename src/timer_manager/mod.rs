use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::connect_4_error::Connect4Error;
use crate::{Event, EventTimerTick};
use crate::timer_manager::players_times::PlayersTimes;
use crate::timer_manager::timer_graphics::TimerGraphics;
use crate::timer_manager::timer_tick::Tick;

mod timer_graphics;
mod players_times;
mod timer_tick;

use crate::ConnectFourThreadObject;

/**
Structure qui comporte les instances des objets, canaux de communication et l'état de la partie
liés à la gestion du temps.
*/
pub struct TimerManager {
    timer_graphics: TimerGraphics,
    players_times: PlayersTimes,
    rx_tick: Receiver<Tick>,
    tx_tick: Sender<EventTimerTick>,
    tx_game_manager: Sender<Event>,
    end_game: bool
}

/**
Implémente les fonctions de la structure de TimerManager
*/
impl TimerManager {
    /**
    Crée et renvoie une instance de TimerManager en prenant en paramètre les noms des joueurs et
    le canal d'envoi de communication pour le game_manager.
    */
    pub(crate) fn new(name_player_1: String, name_player_2: String, tx_game_manager: Sender<Event>) -> TimerManager {
        // Crée deux canaux de communications pour le timer tick (deux sens de communications)
        let (tx_for_timer_tick, rx_for_timer_tick) = channel();
        let (tx_for_timer_manager, rx_for_timer_manager) = channel();

        // Création du thread du timer tick
        let _ = thread::spawn(move || -> Result<(), Connect4Error> {
            timer_tick::run(rx_for_timer_tick, tx_for_timer_manager)?;
            Ok(())
        });

        TimerManager {
            timer_graphics: TimerGraphics::new(name_player_1.clone(), name_player_2.clone()),
            players_times: PlayersTimes::new(name_player_1, name_player_2),
            rx_tick: rx_for_timer_manager,
            tx_tick: tx_for_timer_tick,
            tx_game_manager,
            end_game: false
        }
    }

    /**
    Envoie le start au timer_tick qui commencera le décompte.
    */
    pub fn start(&self) {
        let _ = self.tx_tick.send(EventTimerTick::Start);
    }

    /**
    Fonction principale du timer_manager pour rafraîchir la fenêtre du timer et récupérer les ticks
    du timer_tick.
    */
    pub async fn run(&mut self) -> Result<(), Connect4Error> {

        // Vérifie un tick provenant du timer_tick, de façon non-bloquant.
        let response_tick = self.rx_tick.try_recv();

        // Si tick, met à jour le temps du joueur courant
        match response_tick {
            // Présence d'un tick, on met à jour le temps du joueur courant.
            Ok(Tick::Tick) => {
                let response_players_ticks = self.players_times.tick_time();
                match response_players_ticks {
                    true => {
                        self.timeout();
                        self.end_game = true;
                        Ok::<(), Connect4Error>(())
                    },
                    false => Ok(())
                }
            },
            _ => Ok(())
        }.ok();

        // Met à jour la fenêtre graphique
        self.timer_graphics.update_window(
            self.players_times.timer_player_1.minutes, self.players_times.timer_player_1.seconds,
            self.players_times.timer_player_2.minutes, self.players_times.timer_player_2.seconds,
            self.players_times.id_current_player()
        ).await?;

        Ok(())
    }

    /**
    Interverti de joueur courant.
    */
    pub fn change_player(&mut self) {
        self.players_times.change_player();
    }

    /**
    Renvoie un booléen en fonction de l'état du jeu. Vrai si fin de jeu, sinon faux.
    */
    pub fn is_end_game(&self) -> bool {
        self.end_game
    }

}

/**
Implémentation des fonctions manager du jeu.
*/
impl ConnectFourThreadObject for TimerManager {

    /**
    Temps écoulé pour le joueur courant. Affiche la victoire de l'autre joueur et averti les
    autres objets (game_manager et timer_tick).
    */
    fn timeout(&mut self) {
        println!("Timeout !! Félicitations au vainqueur : {} !! ", self.players_times.get_current_player_name());
        println!("Saisissez n'importe quoi pour quitter");
        // Envoi du temps écoulé à game manager et timer_tick
        let _ = self.tx_game_manager.send(Event::Timeout);
        let _ = self.tx_tick.send(EventTimerTick::End);
    }


    /**
    Fin du jeu envoyé par game_manager. Averti le timer_tick de la fin du jeu.
    */
    fn end_game(&mut self) -> Result<(), Connect4Error> {
        self.end_game = true;
        self.tx_tick.send(EventTimerTick::End)?;
        Ok(())
    }

    /**
    S'assure de l'arrêt du timer_tick.
    */
    fn destroy(&self) {
        let _ = self.tx_tick.send(EventTimerTick::End);
    }
}
