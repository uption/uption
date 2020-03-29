use std::io;

use super::Exporter;
use crate::message::Message;

pub struct Stdout {}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {}
    }
}

impl Exporter for Stdout {
    fn export(&self, msg: &Message) -> Result<(), io::Error> {
        println!("{}", msg);
        Ok(())
    }
}
