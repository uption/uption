//! Data exporters.
mod influxdb;
mod stdout;

use std::io;
use std::{thread, time::Duration};

use crossbeam_channel::Receiver;

use crate::message::Message;
pub use influxdb::InfluxDB;
pub use stdout::Stdout;

pub struct Exporter<T>
where
    T: Sink,
{
    sink: T,
}

impl<T> Exporter<T>
where
    T: Sink,
{
    pub fn new(sink: T) -> Exporter<T> {
        Exporter { sink }
    }

    pub fn start(&self, receiver: Receiver<Message>) {
        loop {
            let msg = receiver.recv().unwrap();

            match self.sink.export(&msg) {
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
