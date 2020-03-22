mod exporters;
mod message;
mod receivers;
mod url;

use std::env;
use std::process;
use std::thread;

use crossbeam_channel::unbounded;

use crate::url::HttpUrl;
use exporters::{Exporter, InfluxDB};
use receivers::{Ping, Receiver, HTTP};

fn main() {
    println!("Uption started");

    ctrlc::set_handler(|| {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (s, r) = unbounded();

    let url = HttpUrl::parse("https://www.google.com").unwrap();

    let mut receiver = Receiver::new();
    receiver.register(HTTP::new(url.into()));
    receiver.register(Ping::new(String::from("8.8.8.8"), 1));

    let thread1 = thread::spawn(move || receiver.start(s));

    let influxdb_url = env::var("INFLUXDB_URL").expect("INFLUXDB_URL");
    let influxdb_bucket = env::var("INFLUXDB_BUCKET").expect("INFLUXDB_BUCKET");
    let influxdb_org = env::var("INFLUXDB_ORG").expect("INFLUXDB_ORG");
    let influxdb_token = env::var("INFLUXDB_TOKEN").expect("INFLUXDB_TOKEN");

    let url = HttpUrl::parse(&influxdb_url).unwrap();
    let exporter = Exporter::new(InfluxDB::new(
        &url,
        &influxdb_bucket,
        &influxdb_org,
        &influxdb_token,
        10,
    ));
    let thread2 = thread::spawn(move || exporter.start(r));

    thread1.join().expect("The sender thread has panicked");
    thread2.join().expect("The receiver thread has panicked");
}
