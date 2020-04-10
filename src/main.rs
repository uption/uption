mod collectors;
mod config;
mod error;
mod exporters;
mod logging;
mod message;
mod uption;
mod url;

use std::process;

use log::{error, warn};

use crate::config::UptionConfig;
use uption::Uption;

fn main() {
    set_ctrl_c_handler();

    let config = UptionConfig::new().unwrap_or_else(|err| {
        println!("Configuration error: {}", err);
        process::exit(1);
    });

    let uption = Uption::new(config);
    uption.start();
}

fn set_ctrl_c_handler() {
    ctrlc::set_handler(|| {
        warn!("Shutdown Uption");
        process::exit(0);
    })
    .unwrap_or_else(|err| {
        error!("Error setting Ctrl+C handler: {}", err);
        process::exit(2);
    });
}
