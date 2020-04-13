use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::collectors::CollectorScheduler;
use crate::config::{Configure, UptionConfig};
use crate::exporters::ExporterScheduler;
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

        let (sender, receiver) = crossbeam_channel::unbounded();
        let collector_scheduler = self.start_collector_scheduler(sender);
        let exporter_scheduler = self.start_exporter_scheduler(receiver);

        collector_scheduler
            .join()
            .expect("The collector scheduler thread has panicked");
        exporter_scheduler
            .join()
            .expect("The export scheduler thread has panicked");

        println!("Uption stopped");
    }

    fn start_collector_scheduler(&self, sender: Sender<Message>) -> thread::JoinHandle<()> {
        let scheduler = CollectorScheduler::from_config(&self.config);
        let builder = thread::Builder::new().name("collector_scheduler".into());
        let hostname = self.config.general.hostname.to_owned();
        builder
            .spawn(move || scheduler.start(sender, hostname))
            .unwrap()
    }

    fn start_exporter_scheduler(&self, receiver: Receiver<Message>) -> thread::JoinHandle<()> {
        let mut scheduler = ExporterScheduler::from_config(&self.config);
        let builder = thread::Builder::new().name("export_scheduler".into());
        builder.spawn(move || scheduler.start(receiver)).unwrap()
    }
}
