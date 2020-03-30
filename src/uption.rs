use std::thread;

use crossbeam_channel::unbounded;

use crate::collectors::{CollectorScheduler, Ping, HTTP};
use crate::config::{ExporterSelection, UptionConfig};
use crate::exporters::{ExporterScheduler, InfluxDB, Stdout};

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
        let (s, r) = unbounded();

        let mut collector_scheduler = CollectorScheduler::new(self.config.collectors.interval);
        self.register_ping_collectors(&mut collector_scheduler);
        self.register_http_collectors(&mut collector_scheduler);
        let collector_scheduler = thread::spawn(move || collector_scheduler.start(s));

        let mut export_scheduler = ExporterScheduler::new();
        self.register_exporters(&mut export_scheduler);
        let export_scheduler = thread::spawn(move || export_scheduler.start(r));

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
                scheduler.register(Ping::new(host.clone(), 1));
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

    fn register_exporters(&self, scheduler: &mut ExporterScheduler) {
        let export = &self.config.exporters;
        match export.exporter {
            ExporterSelection::InfluxDB => scheduler.register(InfluxDB::new(
                export.influxdb.url.as_ref().unwrap(),
                &export.influxdb.bucket.as_ref().unwrap(),
                &export.influxdb.organization.as_ref().unwrap(),
                &export.influxdb.token.as_ref().unwrap(),
                export.influxdb.timeout,
            )),
            ExporterSelection::Stdout => scheduler.register(Stdout::new()),
        };
    }
}
