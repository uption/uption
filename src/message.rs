//! Implementation for messages which are passed between collectors and
//! exporters.
use std::collections::BTreeMap;
use std::fmt;

use chrono::{DateTime, Utc};
use csv::Writer;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    timestamp: DateTime<Utc>,
    source: String,
    tags: BTreeMap<String, String>,
    metrics: BTreeMap<String, PayloadValue>,
}

impl Message {
    pub fn new(source: &str) -> Message {
        Message {
            timestamp: Utc::now(),
            source: String::from(source),
            tags: BTreeMap::new(),
            metrics: BTreeMap::new(),
        }
    }

    pub fn insert_tag(&mut self, name: &str, value: &str) {
        self.tags.insert(String::from(name), value.to_string());
    }

    pub fn insert_metric(&mut self, name: &str, value: impl Into<PayloadValue>) {
        self.metrics.insert(String::from(name), value.into());
    }

    pub fn tags(&self) -> &BTreeMap<String, String> {
        &self.tags
    }

    pub fn metrics(&self) -> &BTreeMap<String, PayloadValue> {
        if self.metrics.is_empty() {
            panic!("Message has no metrics!")
        }
        &self.metrics
    }

    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn format_tags(&self) -> String {
        self.tags()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn format_metrics(&self) -> String {
        self.metrics()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Malformed message")
    }

    pub fn to_csv(&self) -> String {
        let mut writer = Writer::from_writer(vec![]);
        let data = [
            self.timestamp.to_rfc3339(),
            self.source.to_string(),
            self.format_tags(),
            self.format_metrics(),
        ];
        writer.write_record(&data).expect("Malformed message");
        String::from_utf8(writer.into_inner().expect("Malformed message"))
            .expect("Malformed message")
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, source={} {}] {}",
            self.timestamp.to_rfc3339(),
            self.source,
            self.format_tags(),
            self.format_metrics()
        )
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
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

impl From<String> for PayloadValue {
    fn from(item: String) -> Self {
        PayloadValue::String(item)
    }
}

impl From<&str> for PayloadValue {
    fn from(item: &str) -> Self {
        PayloadValue::String(item.to_owned())
    }
}

impl From<i8> for PayloadValue {
    fn from(item: i8) -> Self {
        PayloadValue::Int8(item)
    }
}

impl From<i16> for PayloadValue {
    fn from(item: i16) -> Self {
        PayloadValue::Int16(item)
    }
}

impl From<i32> for PayloadValue {
    fn from(item: i32) -> Self {
        PayloadValue::Int32(item)
    }
}

impl From<i64> for PayloadValue {
    fn from(item: i64) -> Self {
        PayloadValue::Int64(item)
    }
}

impl From<i128> for PayloadValue {
    fn from(item: i128) -> Self {
        PayloadValue::Int128(item)
    }
}

impl From<u8> for PayloadValue {
    fn from(item: u8) -> Self {
        PayloadValue::Uint8(item)
    }
}

impl From<u16> for PayloadValue {
    fn from(item: u16) -> Self {
        PayloadValue::Uint16(item)
    }
}

impl From<u32> for PayloadValue {
    fn from(item: u32) -> Self {
        PayloadValue::Uint32(item)
    }
}

impl From<u64> for PayloadValue {
    fn from(item: u64) -> Self {
        PayloadValue::Uint64(item)
    }
}

impl From<u128> for PayloadValue {
    fn from(item: u128) -> Self {
        PayloadValue::Uint128(item)
    }
}

impl From<f32> for PayloadValue {
    fn from(item: f32) -> Self {
        PayloadValue::Float32(item)
    }
}

impl From<f64> for PayloadValue {
    fn from(item: f64) -> Self {
        PayloadValue::Float64(item)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display_message() {
        let mut msg = Message::new("measurement");
        msg.insert_tag("tag1", "1");
        msg.insert_tag("tag2", "2");
        msg.insert_metric("field1", "1");
        msg.insert_metric("field2", "2");

        assert_eq!(
            msg.to_string(),
            format!(
                "[{}, source=measurement tag1=1 tag2=2] field1=1 field2=2",
                msg.timestamp.to_rfc3339()
            )
        );
    }
}
