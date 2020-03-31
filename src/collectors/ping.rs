use std::process::Command;

use regex::Regex;

use super::error::CollectorError;
use super::Collector;
use crate::message::Message;
use crate::url::Host;

pub struct Ping {
    host: Host,
    timeout: u64,
}

impl Ping {
    pub fn new(host: Host, timeout: u64) -> Ping {
        Ping { host, timeout }
    }

    fn get_ping_latency(&self) -> f64 {
        let output = self.execute_ping_on_command_line();
        self.parse_latency_from_ping_output(&output)
    }

    fn execute_ping_on_command_line(&self) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ping {} -c 1 -w {}",
                &self.host.to_string(),
                self.timeout
            ))
            .output()
            .expect("failed to execute process");

        String::from_utf8(output.stdout).expect("Failed to parse")
    }

    fn parse_latency_from_ping_output(&self, ping_output: &str) -> f64 {
        let captures = self.latency_regex().captures(ping_output).unwrap();
        let latency = captures.get(1).map(|m| m.as_str()).unwrap();

        latency.parse().unwrap()
    }

    fn latency_regex(&self) -> Regex {
        Regex::new(r#"time=(\d+\.\d+) ms"#).unwrap()
    }
}

impl Collector for Ping {
    fn collect(&self) -> Result<Message, CollectorError> {
        let latency = self.get_ping_latency();

        let mut message = Message::new("ping");
        message.insert_data("latency", latency);
        Ok(message)
    }
}
