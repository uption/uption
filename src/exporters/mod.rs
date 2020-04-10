//! Data exporters.
mod influxdb;
mod stdout;

use std::{thread, time::Duration};

extern crate rand;
use crossbeam_channel::Receiver;
use log::{debug, error, info, warn};
use rand::Rng;

use crate::config::{Configure, ExporterSelection, UptionConfig};
use crate::error::{Error, Result};
use crate::message::Message;
pub use influxdb::InfluxDB;
pub use stdout::Stdout;

const ZERO_DURATION: Duration = Duration::from_secs(0);

pub struct ExporterScheduler {
    exporter: Box<dyn Exporter + Send>,
    retry_buffer: RetryItem,
}

impl ExporterScheduler {
    pub fn new(exporter: impl Exporter + Send + 'static) -> ExporterScheduler {
        ExporterScheduler {
            exporter: Box::new(exporter),
            retry_buffer: RetryItem::new(Duration::from_secs(120)),
        }
    }

    pub fn start(&mut self, receiver: Receiver<Message>) {
        info!("Exporter scheduler started");

        while let Some(message) = self.receive(&receiver) {
            self.export(message);
            self.backoff_sleep();
        }

        error!("Collectors disconnected. Stopping exporter.");
    }

    fn receive(&mut self, receiver: &Receiver<Message>) -> Option<Message> {
        match self.retry_buffer.take() {
            Some(message) => Some(message),
            None => match receiver.recv() {
                Ok(message) => Some(message),
                Err(_) => None,
            },
        }
    }

    fn export(&mut self, message: Message) {
        match self.exporter.export(&message) {
            Ok(_) => {
                debug!("Exported message from {} collector", message.source());
                self.retry_buffer.decrement_error_count();
            }
            Err(err) => self.handle_export_error(message, err),
        }
    }

    fn handle_export_error(&mut self, message: Message, err: Error) {
        error!("{}", err);
        self.retry_buffer.set(message);
        self.retry_buffer.increment_error_count();
    }

    fn backoff_sleep(&self) {
        if self.retry_buffer.error_count > 0 {
            let backoff = self.retry_buffer.backoff_duration(true);
            warn!("Exporting again in {:?}", backoff);
            thread::sleep(backoff);
        }
    }
}

pub trait Exporter {
    fn export(&self, msg: &Message) -> Result<()>;
}

struct RetryItem {
    message: Option<Message>,
    error_count: u64,
    max_backoff: Duration,
}

impl RetryItem {
    fn new(max_backoff: Duration) -> Self {
        RetryItem {
            message: None,
            error_count: 0,
            max_backoff,
        }
    }

    fn take(&mut self) -> Option<Message> {
        self.message.take()
    }

    fn set(&mut self, message: Message) {
        self.message = Some(message);
    }

    fn increment_error_count(&mut self) {
        if self.backoff_duration(false) < self.max_backoff {
            self.error_count += 1;
        }
    }

    fn decrement_error_count(&mut self) {
        if self.error_count > 0 {
            self.error_count -= 1;
        }
    }

    fn backoff_duration(&self, jitter: bool) -> Duration {
        match self.error_count {
            0 => ZERO_DURATION,
            _ => {
                let base = 200.0;
                let multiplier = 1.5_f64;
                let n = self.error_count as f64;

                let mut backoff =
                    (base * (multiplier.powf(n) - 1.0)).min(self.max_backoff.as_millis() as f64);

                if jitter {
                    let mut rng = rand::thread_rng();
                    backoff += backoff * rng.gen_range(-0.1, 0.1);
                }

                Duration::from_millis(backoff as u64)
            }
        }
    }
}

impl Configure for ExporterScheduler {
    fn from_config(config: &UptionConfig) -> Self {
        match config.exporters.exporter {
            ExporterSelection::InfluxDB => ExporterScheduler::new(InfluxDB::new(
                config.exporters.influxdb.url.as_ref().unwrap(),
                config.exporters.influxdb.bucket.as_ref().unwrap(),
                config.exporters.influxdb.organization.as_ref().unwrap(),
                config.exporters.influxdb.token.as_ref().unwrap(),
                config.exporters.influxdb.timeout,
            )),
            ExporterSelection::Stdout => ExporterScheduler::new(Stdout::new()),
        }
    }
}
