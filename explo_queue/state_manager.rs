
/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

/**
 * Crates librairies
 */
extern crate queues;
use queues::*;

/**
 * Project librairies
 */
//mod power_event;

enum MyEvents {
    // Event for all modules 
    LAUNCH,
    END_GAME,
    
    // Event for state_manager
    STOP_CHRONO,
    
    // Event for grid_manager
    POSITION
    
}

pub fn start_state_manager(stringToPrint: &str, queue: &mut Arc<Mutex<Queue<isize>>>) {
    let event = MyEvents::LAUNCH as isize;;

    //println!("Event created : {:?}", event);
    println!("MASTER - print the string : {:?}", stringToPrint);

    let mut q = queue.lock().unwrap();
    q.add(event);

    thread::sleep(Duration::from_millis(2000));
    println!("MASTER - Queue values : {:?}", &q);

}