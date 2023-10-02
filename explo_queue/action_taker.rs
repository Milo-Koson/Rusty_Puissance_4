/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;
use std::io;
use std::sync::mpsc::{channel, RecvTimeoutError, Receiver, Sender};

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

 pub fn start_action_taker(rx_action_taker: Receiver<i32>, tx_action_taker_to_state_manager: Sender<i32>) {
    
    println!("Action taker - Start");

    let (tx, rx) = channel();

    // Creates sub-thread taker to catch entry user 
    let thread_sub_taker = thread::spawn(move || {

        println!("Sub-taker - Start");

        let mut playing = true;

        while playing {

            // Catch user entry
            let mut input_coordinates= String::new();

            io::stdin()
                .read_line(&mut input_coordinates)
                .expect("Sub-taker - Bad entry, try it again please");

            // Would take event from channel sent by action_taker
            let res: Result<&str, RecvTimeoutError> = rx.recv_timeout(Duration::from_millis(100));
            
            println!("res : {:?}", res);
            match res {
                Ok("STOP") => {
                    println!("STOP RECEIVED");
                    playing = false;
                },
                Ok(&_) => println!("Unknown event received !"),
                Err(_) => println!("TO waiting event from action taker"),
            }

            // Check if we still are in game 
            // If yes, send position to state_manager 
            if playing {

                println!("Sub-taker - input coordinates : {}", input_coordinates);

                let input_coordinates_int: Result<i8, _> = input_coordinates.parse();

                match input_coordinates_int {
                    Ok(value) => {
                        println!("Successfully parsed as i8: {}", value);
                    }
                    Err(_) => {
                        println!("Failed to parse as i8");
                    }
                }

                // state_manager_queue.push()
                println!("Sub-taker - Send coordinates to state_manager");
            } 
        }
        
        println!("Sub-taker - END OF PROGRAM");

        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        }
    );

    // Bloquing step waiting events
    let mut event_received = Ok(EventTypes::DEFAULT as i32);
    while event_received != Ok(EventTypes::END_GAME as i32) {

        // Catch new event in the grid_queue.
        event_received = rx_action_taker.recv();
        println!("Action taker - new event received: {:?}", event_received);

        if event_received == Ok(EventTypes::LAUNCH as i32) {
            println!("Action taker - LAUNCH received");
        } else if event_received == Ok(EventTypes::POSITION as i32) {
            println!("Action taker - Position received");
        } else if event_received == Ok(EventTypes::END_GAME as i32) {
            println!("Action taker - End game received");
            // Alert subthread and user to quit. 
            let _ignored = tx.send("STOP");
            println!("PLEASE QUIT");
        } else {
            println!("Action taker - Unknown event received");
        }
    }

    println!("Action taker - Waiting sub-taker");
    // Waiting the end of sub-taker thread.
    let _wait_end_sub_taker = thread_sub_taker.join();

    println!("Action taker - END OF PROGRAM");
 }
