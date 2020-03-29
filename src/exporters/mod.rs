//! Data exporters.
mod influxdb;
mod stdout;

use std::io;
use std::{thread, time::Duration};

use crossbeam_channel::Receiver;

use crate::message::Message;
pub use influxdb::InfluxDB;
pub use stdout::Stdout;

pub struct ExporterScheduler {
    exporter: Option<Box<dyn Exporter + Send>>,
}

impl ExporterScheduler {
    pub fn new() -> ExporterScheduler {
        ExporterScheduler { exporter: None }
    }

    pub fn register(&mut self, exporter: impl Exporter + Send + 'static) {
        self.exporter.replace(Box::new(exporter));
    }

    pub fn start(&self, receiver: Receiver<Message>) {
        if self.exporter.is_none() {
            println!("No exporters configured!");
            return;
        }
        println!("Exporter scheduler started");

        loop {
            let msg = match receiver.recv() {
                Ok(msg) => msg,
                Err(_) => {
                    println!("Collectors disconnected. Stopping exporter.");
                    return;
                }
            };

            match self.exporter.as_ref().unwrap().export(&msg) {
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

pub trait Exporter {
    fn export(&self, msg: &Message) -> Result<(), io::Error>;
}
