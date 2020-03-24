//! Data exporters.
mod influxdb;
mod stdout;

use std::io;
use std::{thread, time::Duration};

use crossbeam_channel::Receiver;

use crate::message::Message;
pub use influxdb::InfluxDB;
pub use stdout::Stdout;

pub struct Exporter {
    sink: Option<Box<dyn Sink + Send>>,
}

impl Exporter {
    pub fn new() -> Exporter {
        Exporter { sink: None }
    }

    pub fn register(&mut self, sink: impl Sink + Send + 'static) {
        self.sink.replace(Box::new(sink));
    }

    pub fn start(&self, receiver: Receiver<Message>) {
        if self.sink.is_none() {
            println!("No receivers configured!");
            return;
        }
        println!("Export scheduler started");

        loop {
            let msg = match receiver.recv() {
                Ok(msg) => msg,
                Err(_) => {
                    println!("Collectors disconnected. Stopping exporter.");
                    return;
                }
            };

            match self.sink.as_ref().unwrap().export(&msg) {
                Ok(_) => {
                    println!("Exported message from {} collector", msg.source);
                }
                Err(err) => {
                    println!("Export error: {}", err);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
}

pub trait Sink {
    fn export(&self, msg: &Message) -> Result<(), io::Error>;
}
