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
}

impl Receiver {
    pub fn new() -> Receiver {
        Receiver {
            collectors: Vec::new(),
        }
    }

    pub fn register(&mut self, collector: impl Collector + Send + 'static) {
        self.collectors.push(Box::new(collector));
    }

    pub fn start(&self, sender: Sender<Message>) {
        if self.collectors.is_empty() {
            panic!("No receivers configured!");
        }

        loop {
            for collector in self.collectors.iter() {
                let msg = collector.collect().unwrap();
                sender.send(msg).unwrap();
            }
            thread::sleep(Duration::from_secs(5));
        }
    }
}

pub trait Collector {
    fn collect(&self) -> Result<Message, ReceiverError>;
}
