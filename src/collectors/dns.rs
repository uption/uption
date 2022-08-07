//! DNS collector makes a DNS query (A record) to a hostname and records the
//! time to finish the query.
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::{Duration, Instant};

use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

use super::Collector;
use crate::config::Timeout;
use crate::error::{Result, ResultError};
use crate::message::Message;
use crate::url::Host;

pub struct Dns {
    server: Ipv4Addr,
    host: Host,
    timeout: Duration,
}

impl Dns {
    pub fn new(server: Ipv4Addr, host: Host, timeout: Timeout) -> Self {
        Self {
            server,
            host,
            timeout: Duration::from_secs(timeout.into()),
        }
    }

    fn make_dns_query(&self) -> Result<u128> {
        let server_address = format!("{}:53", self.server).parse()?;
        let conn = UdpClientConnection::with_timeout(server_address, self.timeout)?;
        let client = SyncClient::new(conn);
        let name = Name::from_str(&self.host.to_string())?;

        let now = Instant::now();
        client.query(&name, DNSClass::IN, RecordType::A)?;
        Ok(now.elapsed().as_millis())
    }
}

impl Collector for Dns {
    fn collect(&self) -> Result<Vec<Message>> {
        let latency = self.make_dns_query().set_source("dns_collector")?;

        let mut message = Message::new("dns");
        message.insert_metric("latency", latency);
        message.insert_tag("dns_server", &self.server.to_string());
        message.insert_tag("host", &self.host.to_string());
        Ok(vec![message])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn dns_collect() {
        let dns = Dns::new(
            "8.8.8.8".parse().unwrap(),
            "www.google.com".parse().unwrap(),
            Timeout(1),
        );
        let msg = dns.collect().unwrap().pop().unwrap();

        assert_eq!(msg.source(), "dns");
        assert!(msg.metrics().get("latency").is_some());
        assert_eq!(msg.tags()["dns_server"], "8.8.8.8");
        assert_eq!(msg.tags()["host"], "www.google.com");
    }
}
