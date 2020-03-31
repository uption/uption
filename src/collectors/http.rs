use std::time::{Duration, Instant};

use http_req::error::Error as HttpError;
use http_req::request::{Method, Request};

use super::error::CollectorError;
use super::Collector;
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
}

impl From<HttpError> for CollectorError {
    fn from(error: HttpError) -> Self {
        let err = match error {
            HttpError::IO(err) => err.to_string(),
            HttpError::Tls => String::from("TLS error"),
            HttpError::Parse(err) => return CollectorError::CollectionError(err.to_string()),
        };
        CollectorError::ConnectionError(format!("{}", err))
    }
}

impl Collector for HTTP {
    fn collect(&self) -> Result<Message, CollectorError> {
        let mut writer = Vec::new();

        let now = Instant::now();
        let resp = Request::new(&self.url.as_str().parse().unwrap())
            .method(Method::HEAD)
            .connect_timeout(Some(Duration::from_secs(self.timeout)))
            .read_timeout(Some(Duration::from_secs(self.timeout)))
            .write_timeout(Some(Duration::from_secs(self.timeout)))
            .send(&mut writer)?;
        let latency = now.elapsed().as_millis();

        let mut message = Message::new("http");
        message.insert_data("latency", latency);
        message.insert_data("status_code", resp.status_code().to_string());

        Ok(message)
    }
}
