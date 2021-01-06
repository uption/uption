//! InfluxDB API v2 exporter implementation.
use std::str;
use std::time::Duration;

use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::Deserialize;

use super::InfluxDB;
use crate::config::{Configure, Timeout, UptionConfig};
use crate::error::{Error, Result, ResultError};
use crate::exporters::Exporter;
use crate::message::Message;
use crate::url::HttpUrl;

/// Schema for error response returned from InfluxDB v2 API.
#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
}

pub struct InfluxDBv2 {
    url: HttpUrl,
    token: String,
    timeout: Duration,
}

impl InfluxDBv2 {
    pub fn new(
        url: &HttpUrl,
        bucket: &str,
        organization: &str,
        token: &str,
        timeout: Timeout,
    ) -> InfluxDBv2 {
        let mut url = url.clone();
        url.set_path("api/v2/write");
        url.query_pairs_mut().append_pair("bucket", bucket);
        url.query_pairs_mut().append_pair("org", organization);
        url.query_pairs_mut().append_pair("precision", "ms");

        InfluxDBv2 {
            url,
            token: String::from(token),
            timeout: Duration::from_secs(timeout.into()),
        }
    }

    fn format_token(&self) -> String {
        format!("Token {}", self.token)
    }
}

impl InfluxDB for InfluxDBv2 {
    fn set_authentication(&self, req: RequestBuilder) -> RequestBuilder {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&self.format_token()).unwrap(),
        );
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        req.headers(headers)
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

    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn url(&self) -> &HttpUrl {
        &self.url
    }
}

impl Exporter for InfluxDBv2 {
    fn export(&self, message: &Message) -> Result<()> {
        self.send_to_influxdb(message)
            .set_source("influxdb_v2_exporter")
    }
}

impl Configure for InfluxDBv2 {
    fn from_config(config: &UptionConfig) -> Self {
        InfluxDBv2::new(
            config.exporters.influxdb.url.as_ref().unwrap(),
            config.exporters.influxdb.bucket.as_ref(),
            config.exporters.influxdb.organization.as_ref(),
            config.exporters.influxdb.token.as_ref(),
            config.exporters.influxdb.timeout,
        )
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
        message.insert_metric("field1", "foo");
        message.insert_metric("field2", 2);
        message
    }

    #[rstest]
    fn export_successful(message: Message) {
        let m = mockito::mock("POST", "/api/v2/write?bucket=bucket&org=org&precision=ms")
            .with_status(204)
            .with_header("content-type", "text/plain")
            .with_header("authorization", "Token token")
            .match_body(Regex(
                r#"^measurement,tag1=1,tag2=2 field1="foo",field2=2 \d{13}$"#.to_string(),
            ))
            .create();

        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let exporter = InfluxDBv2::new(&url, "bucket", "org", "token", Timeout(1));
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
        let exporter = InfluxDBv2::new(&url, "bucket", "org", "token", Timeout(1));
        let err = exporter.export(&message).unwrap_err();

        assert_eq!(err.context().as_ref().unwrap(), "error message");
        assert_eq!(err.source().as_ref().unwrap(), "influxdb_v2_exporter");
        assert!(err.cause().is_none());
        m.assert();
    }
}
