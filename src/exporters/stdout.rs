use std::io;

use super::Sink;
use crate::message::Message;

pub struct Stdout {}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {}
    }
}

impl Sink for Stdout {
    fn export(&self, data: Message) -> Result<(), io::Error> {
        println!("{}", data);
        Ok(())
    }
}
