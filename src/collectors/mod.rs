//! This module contains data collecting logic and implementations for different
//! collectors. Collectors gather different metrics which are sent to exporter.
mod dns;
mod http;
mod ping;
mod wireless;

use std::{thread, time::Duration};

use crossbeam_channel::Sender;
use log::{error, info};

use crate::config::{Configure, UptionConfig};
use crate::error::Result;
use crate::message::Message;
pub use dns::Dns;
pub use http::Http;
pub use ping::Ping;
pub use wireless::Wireless;

/// Schedules the execution of different collectors. Collectors are not executed
/// in parallel by design so that they would not interfere with each other.
pub struct CollectorScheduler {
    collectors: Vec<Box<dyn Collector + Send>>,
    interval: Duration,
}

impl CollectorScheduler {
    pub fn new(interval: u64) -> CollectorScheduler {
        CollectorScheduler {
            collectors: Vec::new(),
            interval: Duration::from_secs(interval),
        }
    }

    pub fn register(&mut self, collector: impl Collector + Send + 'static) {
        self.collectors.push(Box::new(collector));
    }

    pub fn start(&self, sender: Sender<Message>, hostname: String) {
        if self.collectors.is_empty() {
            error!("No collectors configured!");
            return;
        }
        info!("Collector scheduler started");

        loop {
            for collector in self.collectors.iter() {
                let messages = match collector.collect() {
                    Ok(msg) => msg,
                    Err(err) => {
                        error!("{}", err);
                        continue;
                    }
                };

                for mut message in messages {
                    message.insert_tag("hostname", &hostname);

                    match sender.send(message) {
                        Ok(_) => (),
                        Err(_) => {
                            error!("Exporter disconnected. Stopping collectors.");
                            return;
                        }
                    };
                }
            }
            thread::sleep(self.interval);
        }
    }
}

/// All data collectors need to implement this trait. Collector scheduler uses
/// methods in this trait to start data collection in different collectors.
pub trait Collector {
    /// Starts data collection in the collector implementation and returns a
    /// message that will be sent to exporter.
    fn collect(&self) -> Result<Vec<Message>>;
}

impl Configure for CollectorScheduler {
    fn from_config(config: &UptionConfig) -> Self {
        let mut scheduler = CollectorScheduler::new(config.collectors.interval);

        let ping_config = &config.collectors.ping;
        if ping_config.enabled {
            for host in ping_config.hosts.iter() {
                scheduler.register(Ping::new(host.clone(), ping_config.timeout));
            }
        }

        let http_config = &config.collectors.http;
        if http_config.enabled {
            for url in http_config.urls.iter() {
                scheduler.register(Http::new(url.clone(), http_config.timeout));
            }
        }

        let dns_config = &config.collectors.dns;
        if dns_config.enabled {
            for server in dns_config.dns_servers.iter() {
                for host in dns_config.hosts.iter() {
                    scheduler.register(Dns::new(*server, host.clone(), dns_config.timeout));
                }
            }
        }

        let wireless_config = &config.collectors.wireless;
        if wireless_config.enabled {
            scheduler.register(Wireless::new());
        }

        scheduler
    }
}
