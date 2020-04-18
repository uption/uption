use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};

use super::Collector;
use crate::error::{Result, ResultError};
use crate::message::Message;
use crate::url::HttpUrl;

pub struct HTTP {
    url: HttpUrl,
    timeout: u64,
}

impl HTTP {
    pub fn new(url: HttpUrl, timeout: u64) -> HTTP {
        HTTP { url, timeout }
    }

    fn send_request(&self) -> Result<Response> {
        let client = Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .danger_accept_invalid_certs(true)
            .build()?;

        let resp = client.head(self.url.as_str()).send()?;
        Ok(resp)
    }
}

impl Collector for HTTP {
    fn collect(&self) -> Result<Message> {
        let now = Instant::now();
        let resp = self.send_request().source("http_collector")?;
        let latency = now.elapsed().as_millis();

        let mut message = Message::new("http");
        message.insert_metric("latency", latency);
        message.insert_metric("status_code", resp.status().as_u16());
        message.insert_tag("url", self.url.as_str());

        Ok(message)
    }
}
