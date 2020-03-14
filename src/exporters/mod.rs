//! Data exporters.
use std::io;

use crossbeam_channel::Receiver;

mod influxdb;
mod stdout;

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

    pub fn start(&self, receiver: Receiver<&str>) {
        loop {
            let data = receiver.recv().unwrap();
            self.sink.export(data).unwrap();
        }
    }
}

pub trait Sink {
    fn export(&self, data: &str) -> Result<(), io::Error>;
}
