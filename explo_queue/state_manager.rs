
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
//mod power_event;

struct MyEvents {
    event_type: EventTypes,
    position_x: i8,
    position_y: i8
}

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

pub fn start_state_manager(rx_state_manager: Receiver<i32>, tx_state_manager_to_grid: Sender<i32>, 
    tx_state_manager_to_chrono: Sender<i32>, tx_state_manager_to_action_taker: Sender<i32>) {

    println!("State manager - Start");
    // Start object 
    let _ = tx_state_manager_to_grid.send(EventTypes::LAUNCH as i32);

    let mut should_quit = false;
    while !should_quit {

        // Catch new event in the state_manager channel.
        let event_received = rx_state_manager.recv();
        println!("State manager - new event received: {:?}", event_received);

        if event_received == Ok(EventTypes::LAUNCH as i32) {
            println!("State manager - LAUNCH received");

        // Events from action taker
        } else if event_received == Ok(EventTypes::POSITION as i32) {
            println!("State manager - Position received");
        } else if event_received == Ok(EventTypes::QUIT as i32) {
            println!("State manager - Position received");

        // Events from chrono
        } else if event_received == Ok(EventTypes::TIME as i32) {
            println!("State manager - Time !!");
            should_quit = true;
            
        // Other events
        } else {
            println!("State manager - Unknown event received");
            should_quit = true;
        }
    } 
    
    // End of program : send the end game of all objects
    let _ = tx_state_manager_to_grid.send(EventTypes::END_GAME as i32);
    let _ = tx_state_manager_to_chrono.send(EventTypes::END_GAME as i32);
    let _ = tx_state_manager_to_action_taker.send(EventTypes::END_GAME as i32);
    
    thread::sleep(Duration::from_millis(500));
    // May be send message to state_manager ? 
    println!("State manager - END OF PROGRAM");
    
}