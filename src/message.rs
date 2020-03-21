use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    payload: String,
}

impl Message {
    pub fn new(payload: String) -> Message {
        Message { payload }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.payload)
    }
}
