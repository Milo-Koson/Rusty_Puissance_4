mod stateManager;
mod chronometre;
mod displayer;
pub mod gridManager;

use std::thread;
use std::time::Duration;

fn main() {
    /* Launch thread objects */
    // Creation of StateManager
	let threadStateManager = thread::spawn(|| {
        stateManager::startStateManager();
        // Wait for 5 sec
        thread::sleep(Duration::from_millis(5000));
    	});

    // Creation of Chronometre
	let threadChronometre = thread::spawn(|| {
        chronometre::startTimer();
    	});
        
    // Creation of Displayer
	let threadDisplayer = thread::spawn(|| {
        displayer::startDisplayer();
    	});

    println!("Hello, world!");

    // Wait all thread end
    let waitStateManager = threadStateManager.join();
    let waitDisplayer = threadDisplayer.join();
    let waitChronometre = threadChronometre.join();

    println!("Finish main program")
}

