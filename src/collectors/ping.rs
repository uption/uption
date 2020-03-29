use std::process::Command;

use regex::Regex;

use super::error::CollectorError;
use super::Collector;
use crate::message::Message;
use crate::url::Host;

pub struct Ping {
    host: Host,
    ping_count: u8,
}

impl Ping {
    pub fn new(host: Host, ping_count: u8) -> Ping {
        Ping { host, ping_count }
    }
}

impl Collector for Ping {
    fn collect(&self) -> Result<Message, CollectorError> {
        let ping_output = execute_ping_on_command_line(self);
        Ok(parse_ping_output_to_message(ping_output))
    }
}

fn execute_ping_on_command_line(ping: &Ping) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "ping {} -c {} -q",
            ping.host.to_string(),
            ping.ping_count
        ))
        .output()
        .expect("failed to execute process");
    std::str::from_utf8(&output.stdout)
        .expect("Failed to parse")
        .into()
}

fn parse_ping_output_to_message(ping_output: String) -> Message {
    let lines: Vec<&str> = ping_output.lines().rev().collect();
    let mut message = Message::new("ping");

    let mut fields = parse_packets_line(lines[1]);
    fields.append(&mut parse_rtt_line(lines[0]));

    for (key, value) in fields.iter() {
        message.insert_data(key, *value);
    }

    message
}

fn parse_packets_line(text: &str) -> Vec<(&str, &str)> {
    let packets_data = get_packet_line_regex().captures(text).unwrap();
    vec![
        ("pkt_tx", packets_data.get(1).map_or("", |m| m.as_str())),
        ("pkt_rx", packets_data.get(2).map_or("", |m| m.as_str())),
        ("pkt_loss", packets_data.get(3).map_or("", |m| m.as_str())),
        ("test_time", packets_data.get(4).map_or("", |m| m.as_str())),
    ]
}

fn parse_rtt_line(text: &str) -> Vec<(&str, &str)> {
    let rtt_data = get_rtt_line_regex().captures(text).unwrap();
    vec![
        ("rtt_min", rtt_data.get(1).map_or("", |m| m.as_str())),
        ("rtt_avg", rtt_data.get(2).map_or("", |m| m.as_str())),
        ("rtt_max", rtt_data.get(3).map_or("", |m| m.as_str())),
        ("rtt_mdev", rtt_data.get(4).map_or("", |m| m.as_str())),
    ]
}

#[cfg(target_os = "linux")]
fn get_packet_line_regex() -> Regex {
    Regex::new(r#"([\d]+)[\s\w]+,\s([\d]+)[\s\w]+,\s([\d]{1,3})%[\s\w]+,[\s\w]+\s([\d]+)"#).unwrap()
}

#[cfg(target_os = "macos")]
fn get_packet_line_regex() -> Regex {
    Regex::new(
        r#"([\d]+)[\s\w]+[\s\w]+,\s([\d]+)[\s\w]+[\s\w]+,\s([\d]{1,3}\.[\d]+)%[\s\w]+[\s\w]+"#,
    )
    .unwrap()
}

fn get_rtt_line_regex() -> Regex {
    Regex::new(r#"([\d\.]+?)/([\d\.]+?)/([\d\.]+?)/([\d\.]+)"#).unwrap()
}
