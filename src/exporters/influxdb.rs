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
        let payload = self.message_to_line_protocol(message);
        self.send_request(&payload)
    }

    fn message_to_line_protocol(&self, msg: &Message) -> String {
        let mut lines = String::new();
        for (key, value) in msg.payload().iter() {
            lines.push_str(&format!(
                "{} {}={} {}\n",
                msg.source(),
                key,
                value,
                msg.timestamp().timestamp_millis()
            ))
        }
        lines
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
