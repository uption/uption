use std::io;

use super::Sink;

pub struct Stdout {}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {}
    }
}

impl Sink for Stdout {
    fn export(&self, data: &str) -> Result<(), io::Error> {
        println!("{}", data);
        Ok(())
    }
}
