use std::process;
use std::{thread, time::Duration};

use crossbeam_channel::{unbounded, Sender};

mod exporters;
mod receivers;

use exporters::{Exporter, Stdout};

fn main() {
    println!("Uption started");

    ctrlc::set_handler(|| {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (s, r) = unbounded();

    // Ping
    let thread1 = thread::spawn(move || send_stuff(s));
    // Stdout
    let exporter = Exporter::new(Stdout::new());
    let thread2 = thread::spawn(move || exporter.start(r));

    thread1.join().expect("The sender thread has panicked");
    thread2.join().expect("The receiver thread has panicked");
}

fn send_stuff(s: Sender<&str>) {
    loop {
        s.send("Hi!").unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
