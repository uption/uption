//! Data receivers.
mod dns;
mod http;
mod ping;

use std::{thread, time::Duration};

use crossbeam_channel::Sender;

use crate::message::Message;
pub use ping::Ping;

pub struct Receiver<T>
where
    T: Collector,
{
    collector: T,
}

impl<T> Receiver<T>
where
    T: Collector,
{
    pub fn new(collector: T) -> Receiver<T> {
        Receiver { collector }
    }

    pub fn start(&self, sender: Sender<Message>) {
        loop {
            let data = self.collector.collect().unwrap();
            sender.send(data).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
}

pub trait Collector {
    fn collect(&self) -> Result<Message, String>;
}
