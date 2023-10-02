use std::sync::mpsc::channel;
/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;

/**
 * Creates librairies
 */
use blockingqueue::BlockingQueue;

/**
 * Project librairies
 */
mod queue;
mod state_manager;
mod grid;
mod power_event;
mod chronometre;
mod action_taker;

fn main() {

    // Channel of grid events
    let (tx_state_manager_to_grid, rx_grid) = channel();

    // Channel of state_manager events
    let (tx_to_state_manager, rx_state_manager) = channel();
    let tx_chrono_to_state_manager = tx_to_state_manager.clone();
    let tx_action_taker_to_state_manager = tx_to_state_manager.clone();
    let tx_grid_to_state_manager = tx_to_state_manager.clone();
    
    // Channel of chrono events
    let (tx_state_manager_to_chrono, rx_chrono) = channel();
    
    // Channel of action taker events
    let (tx_state_manager_to_action_taker, rx_action_taker) = channel();

    let thread_state_manager = thread::spawn(move || {
        state_manager::start_state_manager(rx_state_manager, tx_state_manager_to_grid, tx_state_manager_to_chrono,
            tx_state_manager_to_action_taker);
        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
    	}
    );

    let thread_grid = thread::spawn(move || {
        grid::start_grid(rx_grid, tx_grid_to_state_manager);
        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        }
    );

    let thread_chrono_manager = thread::spawn(move || {
        chronometre::start_chrono(rx_chrono, tx_chrono_to_state_manager);
        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        }
    );

    let thread_action_taker = thread::spawn(move || {
        action_taker::start_action_taker(rx_action_taker, tx_action_taker_to_state_manager);
        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        }
    );

    // Waiting end of all thread objects.
    let _wait_end_action_taker = thread_action_taker.join();
    let _wait_end_chrono = thread_chrono_manager.join();
    let _wait_end_grid = thread_grid.join();
    let _wait_end_state_manager = thread_state_manager.join();

}
