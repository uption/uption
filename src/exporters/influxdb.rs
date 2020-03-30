use std::io;
use std::str;
use std::time::Duration;

use http_req::request::{Method, Request};
use http_req::response::{Response, StatusCode};

use super::Sink;
use crate::message::Message;
use crate::url::HttpUrl;

pub struct InfluxDB {
    url: HttpUrl,
    token: String,
    timeout: u64,
}

impl InfluxDB {
    pub fn new(url: &HttpUrl, bucket: &str, org: &str, token: &str, timeout: u64) -> InfluxDB {
        let mut url = url.clone();
        url.set_path("api/v2/write");
        url.query_pairs_mut().append_pair("bucket", bucket);
        url.query_pairs_mut().append_pair("org", org);
        url.query_pairs_mut().append_pair("precision", "ms");

        InfluxDB {
            url,
            token: String::from(token),
            timeout,
        }
    }

    fn format_token(&self) -> String {
        format!("Token {}", self.token)
    }

    fn message_to_line_protocol(&self, msg: &Message) -> String {
        let mut lines = String::new();
        for (key, value) in msg.payload.iter() {
            lines.push_str(&format!(
                "{} {}={} {}\n",
                msg.source,
                key,
                value,
                msg.timestamp.timestamp_millis()
            ))
        }
        lines
    }

    fn send_request(&self, payload: &str) -> Response {
        let mut writer = Vec::new();
        Request::new(&self.url.as_str().parse().unwrap())
            .method(Method::POST)
            .body(&payload.as_bytes())
            .header("Authorization", &self.format_token())
            .header("Content-Type", "text/plain")
            .header("Content-Length", &payload.as_bytes().len())
            .connect_timeout(Some(Duration::from_secs(self.timeout)))
            .read_timeout(Some(Duration::from_secs(self.timeout)))
            .write_timeout(Some(Duration::from_secs(self.timeout)))
            .send(&mut writer)
            .unwrap()
    }
}

impl Sink for InfluxDB {
    fn export(&self, msg: &Message) -> Result<(), io::Error> {
        let payload = self.message_to_line_protocol(&msg);
        let resp = self.send_request(&payload);

        if resp.status_code() != StatusCode::new(204) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("invalid status code {}", resp.status_code()),
            ));
        }
        Ok(())
    }
}
