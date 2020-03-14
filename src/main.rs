use std::process;
use std::{thread, time::Duration};

use crossbeam_channel::{unbounded, Receiver, Sender};

mod exporters;
mod receivers;

fn main() {
    println!("wiperf started");

    ctrlc::set_handler(|| {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (s, r) = unbounded();

    // Ping
    thread::spawn(move || send_stuff(s));
    // Stdout
    thread::spawn(move || receive_stuff(r));

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn send_stuff(s: Sender<&str>) {
    loop {
        s.send("Hi!").unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn receive_stuff(r: Receiver<&str>) {
    loop {
        println!("Received: {:?}", r.recv());
        thread::sleep(Duration::from_secs(1));
    }
}
