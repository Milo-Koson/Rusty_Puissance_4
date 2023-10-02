
use std::sync::mpsc::{Receiver, Sender};
/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
/**
 * Crates librairies
 */
extern crate queues;
use blockingqueue::BlockingQueue;
use queues::*;

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

pub fn start_grid(rx_grid_from_state_manager: Receiver<i32>, tx_grid_to_state_manager: Sender<i32>) {

    println!("Grid - Start");

    let mut should_quit = false;

    while !should_quit {

        // Catch new event in the grid_queue.
        let event_received= rx_grid_from_state_manager.recv();
        println!("Grid - new event received: {:?}", event_received);

        // End game
        if event_received == Ok(EventTypes::LAUNCH as i32) {
            println!("Grid - LAUNCH received");

        // Acceptable coordinates 
        } else if event_received == Ok(EventTypes::POSITION as i32) {
            println!("Grid - Position received");

        // Acceptable coordinates 
        } else if event_received == Ok(EventTypes::END_GAME as i32) {
            println!("Grid - Position received");
            should_quit = true;
        // Unknown event
        } else {
            println!("Grid - Unknown event received (not in range)");
            should_quit = true;
        }
    } 
    
    // End of program 
    thread::sleep(Duration::from_millis(250));
    // May be send message to state_manager ? 
    println!("Grid - END OF PROGRAM");
}

pub fn end_game_grid() {

}
