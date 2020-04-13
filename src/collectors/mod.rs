//! Data collectors.
mod http;
mod ping;

use std::{thread, time::Duration};

use crossbeam_channel::Sender;

use crate::config::{Configure, UptionConfig};
use crate::error::Result;
use crate::message::Message;
pub use http::HTTP;
pub use ping::Ping;

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
            println!("No collectors configured!");
            return;
        }
        println!("Collector scheduler started");

        loop {
            for collector in self.collectors.iter() {
                let mut msg = match collector.collect() {
                    Ok(msg) => msg,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };

                msg.insert_tag("hostname", &hostname);

                match sender.send(msg) {
                    Ok(msg) => msg,
                    Err(_) => {
                        println!("Exporter disconnected. Stopping collectors.");
                        return;
                    }
                };
            }
            thread::sleep(self.interval);
        }
    }
}

pub trait Collector {
    fn collect(&self) -> Result<Message>;
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
                scheduler.register(HTTP::new(url.clone(), http_config.timeout));
            }
        }

        scheduler
    }
}
