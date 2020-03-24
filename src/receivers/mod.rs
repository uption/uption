//! Data receivers.
mod error;
mod http;
mod ping;

use std::{thread, time::Duration};

use crossbeam_channel::Sender;

use crate::message::Message;
pub use error::ReceiverError;
pub use http::HTTP;
pub use ping::Ping;

pub struct Receiver {
    collectors: Vec<Box<dyn Collector + Send>>,
    interval: Duration,
}

impl Receiver {
    pub fn new(interval: u64) -> Receiver {
        Receiver {
            collectors: Vec::new(),
            interval: Duration::from_secs(interval),
        }
    }

    pub fn register(&mut self, collector: impl Collector + Send + 'static) {
        self.collectors.push(Box::new(collector));
    }

    pub fn start(&self, sender: Sender<Message>) {
        if self.collectors.is_empty() {
            println!("No receivers configured!");
            return;
        }
        println!("Collection scheduler started");

        loop {
            for collector in self.collectors.iter() {
                let msg = collector.collect().unwrap();
                match sender.send(msg) {
                    Ok(msg) => msg,
                    Err(_) => {
                        println!("Exporter disconnected. Stopping collectors.");
                        return;
                    }
                };
            }
            thread::sleep(self.interval);
        }
    }
}

pub trait Collector {
    fn collect(&self) -> Result<Message, ReceiverError>;
}
