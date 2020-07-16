//! InfluxDB API v1 exporter implementation.
use std::str;
use std::time::Duration;

use reqwest::blocking::{RequestBuilder, Response};
use reqwest::StatusCode;
use serde::Deserialize;

use super::InfluxDB;
use crate::config::{Configure, Timeout, UptionConfig};
use crate::error::{Error, Result, ResultError};
use crate::exporters::Exporter;
use crate::message::Message;
use crate::url::HttpUrl;

/// Schema for error response returned from InfluxDB v1 API.
#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

pub struct InfluxDBv1 {
    url: HttpUrl,
    username: String,
    password: String,
    timeout: Duration,
}

impl InfluxDBv1 {
    pub fn new(
        url: &HttpUrl,
        database: &str,
        username: &str,
        password: &str,
        timeout: Timeout,
    ) -> InfluxDBv1 {
        let mut url = url.clone();
        url.set_path("write");
        url.query_pairs_mut().append_pair("db", database);
        url.query_pairs_mut().append_pair("precision", "ms");

        InfluxDBv1 {
            url,
            username: username.to_string(),
            password: password.to_string(),
            timeout: Duration::from_secs(timeout.into()),
        }
    }
}

impl InfluxDB for InfluxDBv1 {
    fn set_authentication(&self, req: RequestBuilder) -> RequestBuilder {
        req.basic_auth(&self.username, Some(&self.password))
    }

    fn handle_response_errors(resp: Response) -> Result<()> {
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => {
                let err = Error::new(&format!(
                    "InfluxDB server returned unexpected HTTP status '{}'",
                    resp.status().as_u16()
                ));

                // Set error field to error context if returned
                let err = match resp.json::<ErrorResponse>() {
                    Ok(body) => err.set_context(&body.error),
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

impl Exporter for InfluxDBv1 {
    fn export(&self, message: &Message) -> Result<()> {
        self.send_to_influxdb(message)
            .set_source("influxdb_v1_exporter")
    }
}

impl Configure for InfluxDBv1 {
    fn from_config(config: &UptionConfig) -> Self {
        InfluxDBv1::new(
            config.exporters.influxdb.url.as_ref().unwrap(),
            config.exporters.influxdb.database.as_ref(),
            config.exporters.influxdb.username.as_ref(),
            config.exporters.influxdb.password.as_ref(),
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
        message.insert_metric("field1", "1");
        message.insert_metric("field2", "2");
        message
    }

    #[rstest]
    fn export_successful(message: Message) {
        let m = mockito::mock("POST", "/write?db=uption&precision=ms")
            .with_status(204)
            .with_header("content-type", "text/plain")
            .with_header("authorization", "Basic token")
            .match_body(Regex(
                r"^measurement,tag1=1,tag2=2 field1=1,field2=2 \d{13}$".to_string(),
            ))
            .create();

        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let exporter = InfluxDBv1::new(&url, "uption", "user", "pass", Timeout(1));
        let result = exporter.export(&message);

        m.assert();
        assert!(result.is_ok());
    }

    #[rstest]
    fn export_failed(message: Message) {
        let m = mockito::mock("POST", "/write?db=uption&precision=ms")
            .with_status(500)
            .with_body("{\"error\": \"error message\"}")
            .create();

        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let exporter = InfluxDBv1::new(&url, "uption", "user", "pass", Timeout(1));
        let err = exporter.export(&message).unwrap_err();

        assert_eq!(err.context().as_ref().unwrap(), "error message");
        assert_eq!(err.source().as_ref().unwrap(), "influxdb_v1_exporter");
        assert!(err.cause().is_none());
        m.assert();
    }
}
