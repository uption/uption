use std::thread;

use crossbeam_channel::unbounded;

use crate::config::{ExporterSelection, UptionConfig};
use crate::exporters::{Exporter, InfluxDB, Stdout};
use crate::receivers::{Ping, Receiver, HTTP};

pub struct Uption {
    config: UptionConfig,
}

impl Uption {
    pub fn new(config: UptionConfig) -> Self {
        Uption { config }
    }

    pub fn start(&self) {
        println!("Uption started");
        let (s, r) = unbounded();

        let mut receiver = Receiver::new(self.config.receivers.interval);
        self.register_ping_receivers(&mut receiver);
        self.register_http_receivers(&mut receiver);
        let collect_scheduler = thread::spawn(move || receiver.start(s));

        let mut exporter = Exporter::new();
        self.register_exporters(&mut exporter);
        let export_scheduler = thread::spawn(move || exporter.start(r));

        collect_scheduler
            .join()
            .expect("The collector scheduler thread has panicked");
        export_scheduler
            .join()
            .expect("The export scheduler thread has panicked");
        println!("Uption stopped");
    }

    fn register_ping_receivers(&self, receiver: &mut Receiver) {
        let ping_config = &self.config.receivers.ping;

        if ping_config.enabled {
            for host in ping_config.hosts.iter() {
                receiver.register(Ping::new(host.clone(), 1));
            }
        }
    }

    fn register_http_receivers(&self, receiver: &mut Receiver) {
        let http_config = &self.config.receivers.http;

        if http_config.enabled {
            for url in http_config.urls.iter() {
                receiver.register(HTTP::new(url.clone(), http_config.timeout));
            }
        }
    }

    fn register_exporters(&self, exporter: &mut Exporter) {
        let export = &self.config.exporters;
        match export.exporter {
            ExporterSelection::InfluxDB => exporter.register(InfluxDB::new(
                export.influxdb.url.as_ref().unwrap(),
                &export.influxdb.bucket.as_ref().unwrap(),
                &export.influxdb.organization.as_ref().unwrap(),
                &export.influxdb.token.as_ref().unwrap(),
                export.influxdb.timeout,
            )),
            ExporterSelection::Stdout => exporter.register(Stdout::new()),
        };
    }
}
