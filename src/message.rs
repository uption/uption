use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    timestamp: DateTime<Utc>,
    payload: String,
}

impl Message {
    pub fn new(payload: String) -> Message {
        Message {
            timestamp: Utc::now(),
            payload,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.timestamp.to_rfc3339(), self.payload)
    }
}
