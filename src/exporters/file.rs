//! File exporter.
use super::Exporter;
use crate::error::Result;
use crate::message::Message;

pub struct File {}

impl File {
    pub fn new() -> File {
        File {}
    }
}

impl Exporter for File {
    fn export(&self, msg: &Message) -> Result<()> {
        println!("{}", msg);
        Ok(())
    }
}
