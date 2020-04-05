//! Stdout exporter.
use super::Exporter;
use crate::error::Result;
use crate::message::Message;

pub struct Stdout {}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {}
    }
}

impl Exporter for Stdout {
    fn export(&self, msg: &Message) -> Result<()> {
        println!("{}", msg);
        Ok(())
    }
}
