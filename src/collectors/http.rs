//! HTTP collector sends a HEAD request to a defined URL and records the latency
//! and status code of the returned response.
use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};

use super::Collector;
use crate::config::Timeout;
use crate::error::{Result, ResultError};
use crate::message::Message;
use crate::url::HttpUrl;

pub struct HTTP {
    url: HttpUrl,
    timeout: Duration,
}

impl HTTP {
    pub fn new(url: HttpUrl, timeout: Timeout) -> HTTP {
        HTTP {
            url,
            timeout: Duration::from_secs(timeout.into()),
        }
    }

    fn send_request(&self) -> Result<Response> {
        let client = Client::builder()
            .timeout(self.timeout)
            .danger_accept_invalid_certs(true)
            .build()?;

        let resp = client.head(self.url.as_str()).send()?;
        Ok(resp)
    }
}

impl Collector for HTTP {
    fn collect(&self) -> Result<Vec<Message>> {
        let now = Instant::now();
        let resp = self.send_request().set_source("http_collector")?;
        let latency = now.elapsed().as_millis();

        let mut message = Message::new("http");
        message.insert_metric("latency", latency);
        message.insert_metric("status_code", resp.status().as_u16());
        message.insert_tag("url", self.url.as_str());

        Ok(vec![message])
    }
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    use super::*;
    use crate::message::PayloadValue;
    #[test]
    fn collect_successful() {
        let m = mockito::mock("HEAD", "/").with_status(201).create();
        let url: HttpUrl = mockito::server_url().parse().unwrap();
        let http = HTTP::new(url.clone(), Timeout(1));
        let msg = http.collect().unwrap().pop().unwrap();

        assert_eq!(msg.source(), "http");
        assert_eq!(msg.metrics()["status_code"], PayloadValue::Uint16(201));
        assert!(msg.metrics().get("latency").is_some());
        assert_eq!(msg.tags()["url"], url.to_string());
        m.assert();
    }

    #[test]
    fn collect_failed() {
        let url: HttpUrl = "http://localhost:12345".parse().unwrap();
        let http = HTTP::new(url, Timeout(1));
        let err = http.collect().unwrap_err();

        assert_eq!(err.source().as_ref().unwrap(), "http_collector");
        assert!(err.context().is_some());
        assert!(err.cause().is_none());
    }
}
