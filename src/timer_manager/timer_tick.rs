
use std::sync::mpsc::{Receiver, Sender}; //RecvError

use std::thread;
use std::time::Duration;
use crate::connect_4_error::Connect4Error;
use crate::EventTimerTick;

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

        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        //println!("tick");
        let _ = tx_timer.send(Tick::Tick);

        // Vérification d'un événement envoyé par le timer_manager.
        if let Ok(value_received) = rx_timer.try_recv() {
            match value_received {
                // Fin de partie envoyé
                EventTimerTick::End => {
                    end_game = true;
                }
                _ => {}
            }
        } 
    }

    Ok(())
}
