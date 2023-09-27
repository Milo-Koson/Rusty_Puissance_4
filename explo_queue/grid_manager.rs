
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
enum MyEvents {
    LAUNCH,
    END
}

pub fn start_grid_manager(stringToPrint: &str, queue: &mut Arc<Mutex<Queue<isize>>>) {
    println!("SECOND - print the string : {:?}. Adding a element in the queue !", stringToPrint);
    //queue.add(500);
    let mut q = queue.lock().unwrap();
    // Remove an element from the queue.
    q.add(MyEvents::END as isize);
    if let element = q.remove() {
        println!("SECOND - removed element: {:?}", element);
    }
    
    thread::sleep(Duration::from_millis(1500));
    println!("SECOND - Queue values : {:?}", &q);

}
