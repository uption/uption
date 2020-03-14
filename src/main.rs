mod exporters;
mod message;
mod receivers;
mod url;

use std::process;
use std::thread;

use crossbeam_channel::unbounded;

use crate::url::HttpUrl;
use exporters::{Exporter, Stdout};
use receivers::{Ping, Receiver, HTTP};

fn main() {
    println!("Uption started");

    ctrlc::set_handler(|| {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (s, r) = unbounded();

    let url = HttpUrl::parse("https://www.google.com").unwrap();

    let mut receiver = Receiver::new();
    receiver.register(HTTP::new(url.into()));
    receiver.register(Ping::new(String::from("localhost"), 1));

    let thread1 = thread::spawn(move || receiver.start(s));

    let exporter = Exporter::new(Stdout::new());
    let thread2 = thread::spawn(move || exporter.start(r));

    thread1.join().expect("The sender thread has panicked");
    thread2.join().expect("The receiver thread has panicked");
}
