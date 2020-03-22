use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub payload: HashMap<String, PayloadValue>,
}

impl Message {
    pub fn new(source: &str) -> Message {
        Message {
            timestamp: Utc::now(),
            source: String::from(source),
            payload: HashMap::new(),
        }
    }

    pub fn insert_data(&mut self, field: &str, value: impl Into<PayloadValue>) {
        self.payload.insert(String::from(field), value.into());
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut payload = String::new();
        for (key, value) in self.payload.iter() {
            payload.push_str(&format!(" {}={}", key, value));
        }

        write!(
            f,
            "[{}, source={}]{}",
            self.timestamp.to_rfc3339(),
            self.source,
            payload
        )
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum PayloadValue {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Float32(f32),
    Float64(f64),
}

impl From<&String> for PayloadValue {
    fn from(item: &String) -> Self {
        PayloadValue::String(item.clone())
    }
}

impl From<&str> for PayloadValue {
    fn from(item: &str) -> Self {
        PayloadValue::String(item.to_owned())
    }
}

impl From<&i8> for PayloadValue {
    fn from(item: &i8) -> Self {
        PayloadValue::Int8(item.clone())
    }
}

impl From<&i16> for PayloadValue {
    fn from(item: &i16) -> Self {
        PayloadValue::Int16(item.clone())
    }
}

impl From<&i32> for PayloadValue {
    fn from(item: &i32) -> Self {
        PayloadValue::Int32(item.clone())
    }
}

impl From<&i64> for PayloadValue {
    fn from(item: &i64) -> Self {
        PayloadValue::Int64(item.clone())
    }
}

impl From<&i128> for PayloadValue {
    fn from(item: &i128) -> Self {
        PayloadValue::Int128(item.clone())
    }
}

impl From<&u8> for PayloadValue {
    fn from(item: &u8) -> Self {
        PayloadValue::Uint8(item.clone())
    }
}

impl From<&u16> for PayloadValue {
    fn from(item: &u16) -> Self {
        PayloadValue::Uint16(item.clone())
    }
}

impl From<&u32> for PayloadValue {
    fn from(item: &u32) -> Self {
        PayloadValue::Uint32(item.clone())
    }
}

impl From<&u64> for PayloadValue {
    fn from(item: &u64) -> Self {
        PayloadValue::Uint64(item.clone())
    }
}

impl From<&u128> for PayloadValue {
    fn from(item: &u128) -> Self {
        PayloadValue::Uint128(item.clone())
    }
}

impl From<&f32> for PayloadValue {
    fn from(item: &f32) -> Self {
        PayloadValue::Float32(item.clone())
    }
}

impl From<&f64> for PayloadValue {
    fn from(item: &f64) -> Self {
        PayloadValue::Float64(item.clone())
    }
}

impl fmt::Display for PayloadValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            PayloadValue::String(val) => val.to_owned(),
            PayloadValue::Int8(val) => val.to_string(),
            PayloadValue::Int16(val) => val.to_string(),
            PayloadValue::Int32(val) => val.to_string(),
            PayloadValue::Int64(val) => val.to_string(),
            PayloadValue::Int128(val) => val.to_string(),
            PayloadValue::Uint8(val) => val.to_string(),
            PayloadValue::Uint16(val) => val.to_string(),
            PayloadValue::Uint32(val) => val.to_string(),
            PayloadValue::Uint64(val) => val.to_string(),
            PayloadValue::Uint128(val) => val.to_string(),
            PayloadValue::Float32(val) => val.to_string(),
            PayloadValue::Float64(val) => val.to_string(),
        };
        write!(f, "{}", value)
    }
}
