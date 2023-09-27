extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;

pub fn start_timer() {

    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        println!("In timer !");
        let _ignored = tx.send(());
    });
    
    rx.recv().unwrap();

}
