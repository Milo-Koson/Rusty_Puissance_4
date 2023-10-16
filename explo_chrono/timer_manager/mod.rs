use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use macroquad::prelude::next_frame;
use crate::EventTimer;
use crate::timer_manager::players_times::PlayersTimes;
use crate::timer_manager::timer_graphics::TimerGraphics;
use crate::timer_manager::timer_tick::{EventTimerTick, Tick};

mod timer_graphics;
mod players_times;
mod timer_tick;

pub struct TimerManager<'a> {
    timer_graphics: TimerGraphics,
    players_times: PlayersTimes<'a>,
    rx_tick: Receiver<Tick>,
    tx_tick: Sender<EventTimerTick>
}

impl<'a> TimerManager<'a> {
    pub(crate) fn new(name_player_1: &'a str, name_player_2: &'a str) -> TimerManager<'a> {

        println!("Timer manager - New");
        // Crée deux canaux de communications pour le timer tick
        let (tx_for_timer_tick, rx_for_timer_tick) = channel();
        let (tx_for_timer_manager, rx_for_timer_manager) = channel();

        // Création du thread du timer tick
        let _ = thread::spawn(move || {
            timer_tick::run(rx_for_timer_tick, tx_for_timer_manager);
        });

        TimerManager {
            timer_graphics: TimerGraphics::new(),
            players_times: PlayersTimes::new(name_player_1, name_player_2),
            rx_tick: rx_for_timer_manager,
            tx_tick: tx_for_timer_tick
        }
    }

    pub fn start(&self) {
        let _ = self.tx_tick.send(EventTimerTick::Start);
    }

    pub async fn run(&mut self) -> bool {

        // Vérifie le tick dans le canal venant du tick
        let response_tick = self.rx_tick.try_recv();

        // Si tick, met à jour le temps du joueur courant
        match response_tick {
            Ok(Tick::Tick) => {
                self.players_times.tick_time();
            },
            Ok(Tick::End) => {
                println!("Timer manager - TIME ! From Tick");
                return true;
            },
            _ => {}
        }

        // Met à jour la fenêtre graphique
        self.timer_graphics.update_window(
            self.players_times.timer_player_1.minutes, self.players_times.timer_player_1.seconds,
            self.players_times.timer_player_2.minutes, self.players_times.timer_player_2.seconds,
            self.players_times.id_current_player()
        ).await;

        false
    }

    pub fn change_p(&mut self) {
        println!("Timer manager change player");
        self.players_times.change_player();
    }

    pub fn stop(&self) {

        println!("Timer manager - End game / stop");
        // Envoi d'un signal pour arrêter le timer tick
        let _ = self.tx_tick.send(EventTimerTick::End);
    }
}
