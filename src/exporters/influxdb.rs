//! InfluxDB exporter.
use std::str;
use std::time::Duration;

use reqwest::blocking::{Client, Response};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::Deserialize;

use super::Exporter;
use crate::error::{Error, Result, ResultError};
use crate::message::Message;
use crate::url::HttpUrl;

pub struct InfluxDB {
    url: HttpUrl,
    token: String,
    timeout: u64,
}

/// Schema for error response returned from InfluxDB API.
#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
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
        let resp = self.send_request(payload)?;
        Self::handle_response_errors(resp)
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

    fn send_request(&self, payload: String) -> Result<Response> {
        let client = Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .build()?;

        let resp = client
            .post(self.url.as_str())
            .headers(self.construct_headers())
            .body(payload)
            .send()?;

        Ok(resp)
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&self.format_token()).unwrap(),
        );
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers
    }

    fn format_token(&self) -> String {
        format!("Token {}", self.token)
    }

    fn handle_response_errors(resp: Response) -> Result<()> {
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => {
                let err = Error::new(&format!(
                    "InfluxDB server returned unexpected HTTP status '{}'",
                    resp.status().as_u16()
                ));

                // Set message field to error context if returned
                let err = match resp.json::<ErrorResponse>() {
                    Ok(body) => err.context(&body.message),
                    Err(_) => err,
                };

                Err(err)
            }
        }
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
