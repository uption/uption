use std::process::Command;

use regex::Regex;

use super::Collector;
use crate::error::{Error, Result, ResultError};
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

    fn get_ping_latency(&self) -> Result<f64> {
        let output = self.execute_ping_on_command_line()?;
        self.parse_latency_from_ping_output(&output)
    }

    fn execute_ping_on_command_line(&self) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ping {} -c 1 -w {}",
                &self.host.to_string(),
                self.timeout
            ))
            .output()
            .map_err(|e| Error::new("Failed to execute ping command").cause(e))?;

        let stderr = String::from_utf8(output.stderr).expect("Ping output not valid utf8");
        if !stderr.is_empty() {
            return Err(Error::new("Ping command returned an error").context(stderr.trim()));
        }

        let output = String::from_utf8(output.stdout).expect("Ping output not valid utf8");

        Ok(output)
    }

    fn parse_latency_from_ping_output(&self, ping_output: &str) -> Result<f64> {
        let captures = Self::latency_regex()
            .captures(ping_output)
            .ok_or(Error::new("Failed to parse ping command output").context(ping_output.trim()))?;

        let latency = captures.get(1).map(|m| m.as_str()).unwrap();

        latency.parse().map_err(|e| {
            Error::new("Failed to convert ping latency to a number")
                .cause(e)
                .context(latency)
        })
    }

    fn latency_regex() -> Regex {
        Regex::new(r#"time=(\d+\.\d+) ms"#).expect("Failed to compile regular expression")
    }
}

impl Collector for Ping {
    fn collect(&self) -> Result<Message> {
        let latency = self.get_ping_latency().source("ping_collector")?;

        let mut message = Message::new("ping");
        message.insert_data("latency", latency);
        Ok(message)
    }
}
