mod state_manager;
mod chronometre;
mod displayer;
mod grid;

use std::thread;
use std::time::Duration;

fn main() {
    /* Launch thread objects */
    // Creation of StateManager

    let grid = grid::create_grid(7,6);
    grid::display_grid(&grid);

	let thread_state_manager = thread::spawn(|| {
        //state_manager::start_state_manager();
        // Wait for 5 sec
        thread::sleep(Duration::from_millis(5000));
    	});
        
    // Creation of Chronometre
	let thread_chronometre = thread::spawn(|| {
        chronometre::start_timer();
    	});
        
    // Creation of Displayer
	let thread_displayer = thread::spawn(|| {
        displayer::start_displayer();
    	});

    println!("Hello, world!");

    // Wait all thread end
    let wait_state_manager = thread_state_manager.join();
    let wait_displayer = thread_displayer.join();
    let wait_chronometre = thread_chronometre.join();

    println!("Finish main program")
}

