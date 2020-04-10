extern crate fern;

use log::LevelFilter;

use crate::config::{Configure, UptionConfig};

pub struct Logger {
    dispatcher: fern::Dispatch,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        let dispatcher = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}] {}",
                    chrono::Utc::now().format("[%+]"),
                    record.level(),
                    message
                ))
            })
            .level(level);
        Logger { dispatcher }
    }

    pub fn enable_stdout(self) -> Self {
        Logger {
            dispatcher: self.dispatcher.chain(std::io::stdout()),
        }
    }

    pub fn enable_log_to_file(self, log_file: &str) -> Self {
        match fern::log_file(log_file) {
            Ok(f) => Logger {
                dispatcher: self.dispatcher.chain(f),
            },
            Err(e) => {
                println!("Setting up file logging failed: {}", e);
                self
            }
        }
    }

    pub fn start(self) {
        self.dispatcher.apply().expect("Firing up logging failed");
    }
}

impl Configure for Logger {
    fn from_config(config: &UptionConfig) -> Self {
        let logger_config = &config.logger;
        let logger = Logger::new(logger_config.level);

        let logger = if logger_config.enable_stdout {
            logger.enable_stdout()
        } else {
            logger
        };

        if let Some(log_file) = &logger_config.log_file {
            return logger.enable_log_to_file(&log_file);
        }
        logger
    }
}
