extern crate timer;
extern crate chrono;
use std::sync::mpsc::Receiver;

use std::thread;
use std::time::Duration;

pub enum EVENT_TIMER {
    End
}

pub fn start_timer(rx_timer: Receiver<EVENT_TIMER>) -> std::thread::JoinHandle<()> {
    println!("Start timer");

    let thread_timer = thread::spawn(move || {
        run(rx_timer);
        }
    );

    return thread_timer;

    /*
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        println!("In timer !");
        let _ignored = tx.send(());
    });
    
    rx.recv().unwrap();*/
}

fn run(rx_timer: Receiver<EVENT_TIMER>) {
    let mut in_game = true;
    println!("I am running timer");
    while in_game {
        // Wait for 1 sec
        thread::sleep(Duration::from_millis(1000));
        if let Ok(value) = rx_timer.try_recv() {
            match value {
                EVENT_TIMER::End => {
                    println!("End timer received");
                    in_game = false;
                }
            }
        } 
    }
    println!("End of timer run");
}
