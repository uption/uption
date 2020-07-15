//! InfluxDB exporter.
mod influxdb_v1;
mod influxdb_v2;

use std::time::Duration;

use reqwest::blocking::{Client, RequestBuilder, Response};

use crate::error::Result;
use crate::message::Message;
use crate::url::HttpUrl;

pub use influxdb_v1::InfluxDBv1;
pub use influxdb_v2::InfluxDBv2;

trait InfluxDB {
    fn set_authentication(&self, req: RequestBuilder) -> RequestBuilder;

    fn handle_response_errors(resp: Response) -> Result<()>;

    fn timeout(&self) -> Duration;

    fn url(&self) -> &HttpUrl;

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
        let client = Client::builder().timeout(self.timeout()).build()?;
        let req = client.post(self.url().as_str()).body(payload);
        let resp = self.set_authentication(req).send()?;

        Ok(resp)
    }
}
