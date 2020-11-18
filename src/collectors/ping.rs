//! Ping collector sends ICMP echo message to a defined host and records the
//! round-trip-time. This collector uses the OS `ping` command and parses the
//! response from stdout.
use std::process::Command;

use regex::Regex;

use super::Collector;
use crate::config::Timeout;
use crate::error::{Error, Result, ResultError};
use crate::message::Message;
use crate::url::Host;

pub struct Ping {
    host: Host,
    timeout: u64,
}

impl Ping {
    pub fn new(host: Host, timeout: Timeout) -> Ping {
        Ping {
            host,
            timeout: timeout.into(),
        }
    }

    fn get_ping_latency(&self) -> Result<f64> {
        let output = self.execute_ping_on_command_line()?;
        self.parse_latency_from_ping_output(&output)
    }

    fn execute_ping_on_command_line(&self) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ping {} -q -c 1 -w {}",
                &self.host.to_string(),
                self.timeout
            ))
            .output()
            .map_err(|e| Error::new("Failed to execute ping command").set_cause(e))?;

        let stderr = String::from_utf8(output.stderr).expect("Ping output not valid utf8");
        if !stderr.is_empty() {
            return Err(Error::new("Ping command returned an error").set_context(stderr.trim()));
        }

        let output = String::from_utf8(output.stdout).expect("Ping output not valid utf8");

        Ok(output)
    }

    fn parse_latency_from_ping_output(&self, ping_output: &str) -> Result<f64> {
        let last_line = ping_output
            .lines()
            .last()
            .ok_or_else(|| Error::new("Empty ping command output"))?;

        let captures = Self::latency_regex().captures(last_line).ok_or_else(|| {
            Error::new("Failed to parse ping command output").set_context(last_line.trim())
        })?;

        let latency = captures.get(1).map(|m| m.as_str()).unwrap();

        latency.parse().map_err(|e| {
            Error::new("Failed to convert ping latency to a number")
                .set_cause(e)
                .set_context(latency)
        })
    }

    fn latency_regex() -> Regex {
        Regex::new(r#"= (\d+\.\d+)/"#).expect("Failed to compile regular expression")
    }
}

impl Collector for Ping {
    fn collect(&self) -> Result<Vec<Message>> {
        let latency = self.get_ping_latency().set_source("ping_collector")?;

        let mut message = Message::new("ping");
        message.insert_metric("latency", latency);
        message.insert_tag("host", &self.host.to_string());
        Ok(vec![message])
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[fixture]
    fn ping_output() -> &'static str {
        "PING localhost (127.0.0.1) 56(84) bytes of data.

        --- 127.0.0.1 ping statistics ---
        1 packets transmitted, 1 received, 0% packet loss, time 0ms
        rtt min/avg/max/mdev = 10.192/10.192/10.192/0.000 ms"
    }

    #[rstest]
    #[allow(clippy::float_cmp)]
    fn ping_output_parsing_successful(ping_output: &str) {
        let ping = Ping::new("localhost".parse().unwrap(), Timeout(1));
        let result = ping.parse_latency_from_ping_output(ping_output).unwrap();

        assert_eq!(result, 10.192);
    }

    #[test]
    #[ignore]
    fn ping_collect() {
        let ping = Ping::new("localhost".parse().unwrap(), Timeout(1));
        let msg = ping.collect().unwrap().pop().unwrap();

        assert_eq!(msg.source(), "ping");
        assert!(msg.metrics().get("latency").is_some());
        assert_eq!(msg.tags()["host"], "localhost");
    }
}
