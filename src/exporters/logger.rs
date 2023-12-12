//! Log exporter uses Uption logger to export messages.
use std::fmt::Write;

use log::info;

use super::Exporter;
use crate::error::Result;
use crate::message::Message;

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }

    fn format_message(msg: &Message) -> String {
        let formatted_tags: String = msg.tags().iter().filter(|(k, _)| *k != "hostname").fold(
            String::new(),
            |mut output, (k, v)| {
                let _ = write!(output, " {}=\"{}\"", k, v);
                output
            },
        );

        let formatted_metrics: String =
            msg.metrics()
                .iter()
                .fold(String::new(), |mut output, (k, v)| {
                    let _ = write!(output, " {}=\"{}\"", k, v);
                    output
                });

        format!(
            "timestamp=\"{}\" source=\"{}\"{}{}",
            msg.timestamp().to_rfc3339(),
            msg.source(),
            formatted_tags,
            formatted_metrics
        )
    }
}

impl Exporter for Logger {
    fn export(&self, msg: &Message) -> Result<()> {
        info!("{}", Logger::format_message(msg));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_message() {
        let mut msg = Message::new("measurement");
        msg.insert_tag("tag1", "1");
        msg.insert_tag("tag2", "2");
        msg.insert_metric("field1", "1");
        msg.insert_metric("field2", "2");

        assert_eq!(
            Logger::format_message(&msg),
            format!(
                "timestamp=\"{}\" source=\"measurement\" tag1=\"1\" tag2=\"2\" field1=\"1\" field2=\"2\"",
                msg.timestamp().to_rfc3339()
            )
        );
    }
}
