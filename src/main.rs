mod config;
mod exporters;
mod message;
mod receivers;
mod uption;
mod url;

use std::process;

use crate::config::UptionConfig;
use uption::Uption;

fn main() {
    set_ctrl_c_handler();

    println!("Reading configuration");
    let config = UptionConfig::new().unwrap_or_else(|err| {
        println!("Configuration error: {}", err);
        process::exit(1);
    });

    let uption = Uption::new(config);
    uption.start();
}

fn set_ctrl_c_handler() {
    ctrlc::set_handler(|| {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .unwrap_or_else(|err| {
        println!("Error setting Ctrl+C handler: {}", err);
        process::exit(2);
    });
}
