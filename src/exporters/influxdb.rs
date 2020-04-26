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
                    Ok(body) => err.set_context(&body.message),
                    Err(_) => err,
                };

                Err(err)
            }
        }
    }
}

impl Exporter for InfluxDB {
    fn export(&self, message: &Message) -> Result<()> {
        self.send_to_influxdb(message)
            .set_source("influxdb_exporter")
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;
    use mockito::Matcher::Regex;
    use rstest::*;

    use super::*;
    use crate::message::Message;

    #[fixture]
    fn message() -> Message {
        let mut message = Message::new("measurement");
        message.insert_tag("tag1", "1");
        message.insert_tag("tag2", "2");
        message.insert_metric("field1", "1");
        message.insert_metric("field2", "2");
        message
    }

    #[rstest]
    fn export_successful(message: Message) {
        let m = mockito::mock("POST", "/api/v2/write?bucket=bucket&org=org&precision=ms")
            .with_status(204)
            .with_header("content-type", "text/plain")
            .with_header("authorization", "Token token")
            .match_body(Regex(
                r"^measurement,tag1=1,tag2=2 field1=1,field2=2 \d{13}$".to_string(),
            ))
            .create();

        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let exporter = InfluxDB::new(&url, "bucket", "org", "token", 1);
        let result = exporter.export(&message);

        m.assert();
        assert!(result.is_ok());
    }

    #[rstest]
    fn export_failed(message: Message) {
        let m = mockito::mock("POST", "/api/v2/write?bucket=bucket&org=org&precision=ms")
            .with_status(500)
            .with_body("{\"message\": \"error message\"}")
            .create();

        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let exporter = InfluxDB::new(&url, "bucket", "org", "token", 1);
        let err = exporter.export(&message).unwrap_err();

        assert_eq!(err.context().as_ref().unwrap(), "error message");
        assert_eq!(err.source().as_ref().unwrap(), "influxdb_exporter");
        assert!(err.cause().is_none());
        m.assert();
    }
}
