use std::time::{Duration, Instant};

use http_req::error::Error as HttpError;
use http_req::request::{Method, Request};

use super::error::ReceiverError;
use super::Collector;
use crate::message::Message;
use crate::url::HttpUrl;

pub struct HTTP {
    url: HttpUrl,
}

impl HTTP {
    pub fn new(url: HttpUrl) -> HTTP {
        HTTP { url }
    }
}

impl From<HttpError> for ReceiverError {
    fn from(error: HttpError) -> Self {
        let err = match error {
            HttpError::IO(err) => err.to_string(),
            HttpError::Tls => String::from("TLS error"),
            HttpError::Parse(err) => return ReceiverError::CollectionError(err.to_string()),
        };
        ReceiverError::ConnectionError(format!("{}", err))
    }
}

impl Collector for HTTP {
    fn collect(&self) -> Result<Message, ReceiverError> {
        let mut writer = Vec::new();
        let timeout = Some(Duration::from_secs(10));

        let now = Instant::now();
        let resp = Request::new(&self.url.as_str().parse().unwrap())
            .method(Method::HEAD)
            .timeout(timeout)
            .read_timeout(timeout)
            .send(&mut writer)?;
        let latency = now.elapsed().as_millis();

        let mut message = Message::new("http");
        message.insert_data("latency", &latency);
        message.insert_data("status_code", &resp.status_code().to_string());

        Ok(message)
    }
}
