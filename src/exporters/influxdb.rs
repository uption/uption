//! InfluxDB exporter.
use std::str;
use std::time::Duration;

use http_req::request::{Method, Request};
use http_req::response::StatusCode;

use super::Exporter;
use crate::error::{Error, Result, ResultError};
use crate::message::Message;
use crate::url::HttpUrl;

pub struct InfluxDB {
    url: HttpUrl,
    token: String,
    timeout: u64,
}

impl InfluxDB {
    pub fn new(
        url: &HttpUrl,
        bucket: &str,
        organization: &str,
        token: &str,
        timeout: u64,
    ) -> InfluxDB {
        let mut url = url.clone();
        url.set_path("api/v2/write");
        url.query_pairs_mut().append_pair("bucket", bucket);
        url.query_pairs_mut().append_pair("org", organization);
        url.query_pairs_mut().append_pair("precision", "ms");

        InfluxDB {
            url,
            token: String::from(token),
            timeout,
        }
    }

    fn send_to_influxdb(&self, message: &Message) -> Result<()> {
        let payload = Self::message_to_line_protocol(message);
        self.send_request(&payload)
    }

    fn message_to_line_protocol(msg: &Message) -> String {
        let mut tags = vec![msg.source().to_string()];
        for (key, value) in msg.tags().iter() {
            tags.push(format!("{}={}", key, value));
        }

        let mut fields = Vec::new();
        for (key, value) in msg.metrics().iter() {
            fields.push(format!("{}={}", key, value));
        }

        format!(
            "{} {} {}",
            tags.join(","),
            fields.join(","),
            msg.timestamp().timestamp_millis()
        )
    }

    fn send_request(&self, payload: &str) -> Result<()> {
        let mut writer = Vec::new();
        let resp = Request::new(&self.url.as_str().parse().unwrap())
            .method(Method::POST)
            .body(&payload.as_bytes())
            .header("Authorization", &self.format_token())
            .header("Content-Type", "text/plain")
            .header("Content-Length", &payload.as_bytes().len())
            .connect_timeout(Some(Duration::from_secs(self.timeout)))
            .read_timeout(Some(Duration::from_secs(self.timeout)))
            .write_timeout(Some(Duration::from_secs(self.timeout)))
            .send(&mut writer)
            .map_err(|e| Error::new("Failed to contact InfluxDB server").context(&e.to_string()))?;

        if resp.status_code() != StatusCode::new(204) {
            return Err(Error::new(&format!(
                "InfluxDB server returned invalid HTTP status '{}'",
                resp.status_code()
            ))
            .context(str::from_utf8(&writer).unwrap()));
        }
        Ok(())
    }

    fn format_token(&self) -> String {
        format!("Token {}", self.token)
    }
}

impl Exporter for InfluxDB {
    fn export(&self, message: &Message) -> Result<()> {
        self.send_to_influxdb(message).source("influxdb_exporter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Message;
    #[test]
    fn test_message_to_line_protocol() {
        let mut msg = Message::new("measurement");
        msg.insert_tag("tag1", "1");
        msg.insert_tag("tag2", "2");
        msg.insert_metric("field1", "1");
        msg.insert_metric("field2", "2");

        let line = InfluxDB::message_to_line_protocol(&msg);

        assert_eq!(
            line,
            format!(
                "measurement,tag1=1,tag2=2 field1=1,field2=2 {}",
                msg.timestamp().timestamp_millis()
            )
        );
    }
}
