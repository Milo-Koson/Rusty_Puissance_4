/**
 * Standard librairies
 */
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::sync::Arc;

/**
 * Creates librairies
 */
extern crate queues;
use queues::*;

/**
 * Project librairies
 */
mod queue;
mod state_manager;
mod grid_manager;


fn main() {
    let apple = Arc::new("the same apple");
    let apple2 = Arc::clone(&apple);
    let appleClone = Arc::clone(&apple2);

    let mut queue_state_manager: Queue<isize> = queue![];
    // Wrap the queue in an Arc (atomic reference counter) and a Mutex to share it among threads safely.
    let mut queue_state_manager = Arc::new(Mutex::new(queue_state_manager));
    // Clone Arc for each thread.
    let mut q1 = Arc::clone(&queue_state_manager);
    let mut q2 = Arc::clone(&queue_state_manager);

    //queue::mainQueue(&mut q);

    let thread_state_manager = thread::spawn(move || {
        state_manager::start_state_manager(&apple, &mut q1);//, &mut queue);
        // Wait for 5 sec
        thread::sleep(Duration::from_millis(5000));
    	}
    );

    let thread_grid_manager = thread::spawn(move || {
        grid_manager::start_grid_manager(&appleClone, &mut q2);//, &mut q);
        // Wait for 5 sec
        thread::sleep(Duration::from_millis(5000));
        }
    );

    let wait_state_manager = thread_state_manager.join();
    let wait_grid_manager = thread_grid_manager.join();
    //let received_data = receiver.lock().unwrap().recv().unwrap();

}
