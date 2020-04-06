use std::time::{Duration, Instant};

use http_req::request::{Method, Request};
use http_req::response::Response;

use super::Collector;
use crate::error::{Error, Result, ResultError};
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
        let mut writer = Vec::new();
        Request::new(&self.url.as_str().parse().unwrap())
            .method(Method::HEAD)
            .connect_timeout(Some(Duration::from_secs(self.timeout)))
            .read_timeout(Some(Duration::from_secs(self.timeout)))
            .write_timeout(Some(Duration::from_secs(self.timeout)))
            .send(&mut writer)
            .map_err(|e| Error::new("Failed to send HTTP request").context(&e.to_string()))
    }
}

impl Collector for HTTP {
    fn collect(&self) -> Result<Message> {
        let now = Instant::now();
        let resp = self.send_request().source("http_collector")?;
        let latency = now.elapsed().as_millis();

        let mut message = Message::new("http");
        message.insert_metric("latency", latency);
        message.insert_metric("status_code", resp.status_code().to_string());
        message.insert_tag("url", self.url.as_str());

        Ok(message)
    }
}
