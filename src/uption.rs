use std::thread;

use crossbeam_channel::{unbounded, Receiver};

use crate::collectors::{CollectorScheduler, Ping, HTTP};
use crate::config::{ExporterSelection, UptionConfig};
use crate::exporters::{ExporterScheduler, InfluxDB, Stdout};
use crate::message::Message;

const UPTION_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Uption {
    config: UptionConfig,
}

impl Uption {
    pub fn new(config: UptionConfig) -> Self {
        Uption { config }
    }

    pub fn start(&self) {
        println!("Uption v{} started", UPTION_VERSION.unwrap_or("-unknown"));
        let (sender, receiver) = unbounded();

        let mut collector_scheduler = CollectorScheduler::new(self.config.collectors.interval);
        self.register_ping_collectors(&mut collector_scheduler);
        self.register_http_collectors(&mut collector_scheduler);
        let collector_scheduler = thread::spawn(move || collector_scheduler.start(sender));

        let mut export_scheduler = self.create_export_scheduler(receiver);
        let export_scheduler = thread::spawn(move || export_scheduler.start());

        collector_scheduler
            .join()
            .expect("The collector scheduler thread has panicked");
        export_scheduler
            .join()
            .expect("The export scheduler thread has panicked");
        println!("Uption stopped");
    }

    fn register_ping_collectors(&self, scheduler: &mut CollectorScheduler) {
        let ping_config = &self.config.collectors.ping;

        if ping_config.enabled {
            for host in ping_config.hosts.iter() {
                scheduler.register(Ping::new(host.clone(), ping_config.timeout));
            }
        }
    }

    fn register_http_collectors(&self, scheduler: &mut CollectorScheduler) {
        let http_config = &self.config.collectors.http;

        if http_config.enabled {
            for url in http_config.urls.iter() {
                scheduler.register(HTTP::new(url.clone(), http_config.timeout));
            }
        }
    }

    fn create_export_scheduler(&self, receiver: Receiver<Message>) -> ExporterScheduler {
        let export = &self.config.exporters;
        match export.exporter {
            ExporterSelection::InfluxDB => ExporterScheduler::new(
                InfluxDB::new(
                    export.influxdb.url.as_ref().unwrap(),
                    &export.influxdb.bucket.as_ref().unwrap(),
                    &export.influxdb.organization.as_ref().unwrap(),
                    &export.influxdb.token.as_ref().unwrap(),
                    export.influxdb.timeout,
                ),
                receiver,
            ),
            ExporterSelection::Stdout => ExporterScheduler::new(Stdout::new(), receiver),
        }
    }
}
