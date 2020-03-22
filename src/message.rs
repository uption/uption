use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    timestamp: DateTime<Utc>,
    source: String,
    payload: String,
}

impl Message {
    pub fn new(source: String, payload: String) -> Message {
        Message {
            timestamp: Utc::now(),
            source,
            payload,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, source={}] {}",
            self.timestamp.to_rfc3339(),
            self.source,
            self.payload
        )
    }
}
