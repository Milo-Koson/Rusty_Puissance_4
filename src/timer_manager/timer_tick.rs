use std::sync::mpsc::{Receiver, Sender};

use std::thread;
use std::time::Duration;
use crate::connect_4_error::Connect4Error;
use crate::EventTimerTick;

// Constante du temps d'attente pour le décompte du temps
const DELAY_MILLISECOND: u64 = 1000;

/**
Enumération pour le tick du décomptage du temps.
*/
pub enum Tick {
    Tick
}

/**
Boucle principale du décomptage du temps.
*/
pub fn run(rx_timer: Receiver<EventTimerTick>, tx_timer: Sender<Tick>) -> Result<(), Connect4Error>{

    // Atteste de l'arrêt de la partie.
    let mut end_game = false;

    // Attente du start envoyé par le timer_manager.
    match rx_timer.recv()
    {
        // Fin du jeu demandé avant le début de la partie.
        Ok(EventTimerTick::End) =>  {
            end_game = true;
        },
        Ok(_) => {},
        Err(_) => return Err(Connect4Error::ChannelRecv)
    }

    // Tant que la partie n'est pas finie, on continue de décompter.
    while !end_game {

        // Attente de décompte
        thread::sleep(Duration::from_millis(DELAY_MILLISECOND));

        // Envoi d'un tick au timer_manager
        let _ = tx_timer.send(Tick::Tick);

        // Vérification d'un événement envoyé par le timer_manager.
        if let Ok(value_received) = rx_timer.try_recv() {
            match value_received {
                // Fin de partie envoyé
                EventTimerTick::End => {
                    // Permet de quitter la boucle de décompte
                    end_game = true;
                }
                // Ne fait rien si un start est demandé en cours de décompte
                EventTimerTick::Start => {},
            }
        } 
    }

    Ok(())
}
