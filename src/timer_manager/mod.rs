use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::connect_4_error::{Connect4Error, Connect4Result};
use crate::{Event, EventTimerTick};
use crate::timer_manager::players_times::PlayersTimes;
use crate::timer_manager::timer_graphics::TimerGraphics;
use crate::timer_manager::timer_tick::{Tick};

mod timer_graphics;
mod players_times;
mod timer_tick;

use crate::ConnectFourThreadObject;

pub struct TimerManager {
    timer_graphics: TimerGraphics,
    players_times: PlayersTimes,
    rx_tick: Receiver<Tick>,
    tx_tick: Sender<EventTimerTick>,
    tx_game_manager: Sender<Event>,
    end_game: bool
}

impl TimerManager {
    pub(crate) fn new(name_player_1: String, name_player_2: String, tx_game_manager: Sender<Event>) -> TimerManager {
        // Crée deux canaux de communications pour le timer tick
        let (tx_for_timer_tick, rx_for_timer_tick) = channel();
        let (tx_for_timer_manager, rx_for_timer_manager) = channel();

        // Création du thread du timer tick
        let _ = thread::spawn(move || {
            timer_tick::run(rx_for_timer_tick, tx_for_timer_manager);
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

    pub fn start(&self) {
        let _ = self.tx_tick.send(EventTimerTick::Start);
    }

    pub async fn run(&mut self) -> Result<(), Connect4Error> {

        // Vérifie le tick dans le canal venant du tick
        let response_tick = self.rx_tick.try_recv();

        // Si tick, met à jour le temps du joueur courant
        match response_tick {
            Ok(Tick::Tick) => {
                let response_players_ticks = self.players_times.tick_time();
                match response_players_ticks {
                    true => {
                        self.timeout();
                        self.end_game = true;
                        Ok::<(), Connect4Error>(())
                    },
                    false => Ok({})
                }
            },
            _ => Ok({})
        }.ok();

        // Met à jour la fenêtre graphique
        self.timer_graphics.update_window(
            self.players_times.timer_player_1.minutes, self.players_times.timer_player_1.seconds,
            self.players_times.timer_player_2.minutes, self.players_times.timer_player_2.seconds,
            self.players_times.id_current_player()
        ).await?;

        Ok(())
    }

    pub fn change_player(&mut self) {
        println!("Timer manager change player");
        self.players_times.change_player();
    }

    pub fn is_end_game(&self) -> bool {
        self.end_game
    }

}

impl ConnectFourThreadObject for TimerManager {

    fn timeout(&mut self) {
        println!("Timeout !! Félicitations au vainqueur : {} !! ", self.players_times.get_current_player_name());
        println!("Saisissez n'importe quoi pour quitter");
        // Envoi du temps écoulé à game manager
        let _ = self.tx_game_manager.send(Event::Timeout);
        let _ = self.tx_tick.send(EventTimerTick::End);
    }

    // Fin du jeu reçu par le game_manager, alerte le timer_tick
    fn end_game(&mut self) -> Result<(), Connect4Error> {
        self.end_game = true;
        self.tx_tick.send(EventTimerTick::End)?;
        Ok(())
    }

    fn destroy(&self) {
        //println!("Timer manager - End game / stop");
        // Envoi d'un signal pour arrêter le timer tick
        let _ = self.tx_tick.send(EventTimerTick::End);
    }
}
