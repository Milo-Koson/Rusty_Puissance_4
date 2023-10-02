use std::sync::mpsc::{Sender, Receiver};
/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;

/**
 * Crates librairies
 */
extern crate queues;
use blockingqueue::BlockingQueue;

/**
 * Project librairies
 */


 enum EventTypes {
    DEFAULT = -1,
    END_GAME,
    LAUNCH,
    // Event for all modules 
    
    // Event for state_manager
    STOP_CHRONO, 
    TIME, 
    QUIT,
    
    // Event for state_manager & grid
    POSITION,
}

pub fn start_chrono(rx_chrono: Receiver<i32>, tx_chrono_to_state_manager: Sender<i32>) {

    println!("Chrono - Start");

    // Simulate game time of 5 secs
    thread::sleep(Duration::from_millis(5000)); 
    println!("Chrono - TIME !");

    let _ = tx_chrono_to_state_manager.send(EventTypes::TIME as i32);
    println!("Chrono - END OF PROGRAM");
 }