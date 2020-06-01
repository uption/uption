//! Uption configuration.
use std::path::Path;

use crate::url::{Host, HttpUrl};
use config::{Config, ConfigError, Environment, File};
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UptionConfig {
    pub general: GeneralConfig,
    pub collectors: CollectorsConfig,
    pub exporters: ExportersConfig,
    pub logging: LoggerConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub hostname: String,
}
#[derive(Debug, Deserialize)]
pub struct CollectorsConfig {
    pub interval: u64,
    pub ping: PingConfig,
    pub http: HttpConfig,
}

#[derive(Debug, Deserialize)]
pub struct PingConfig {
    pub enabled: bool,
    pub hosts: Vec<Host>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub enabled: bool,
    pub urls: Vec<HttpUrl>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExportersConfig {
    pub exporter: ExporterSelection,
    pub influxdb: InfluxDBConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "LevelFilter")]
#[serde(rename = "log_level")]
pub enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    #[serde(with = "LevelFilterDef")]
    pub level: LevelFilter,

    pub enable_stdout: bool,
    pub log_file: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExporterSelection {
    InfluxDB,
    Stdout,
    Logger,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InfluxDBVersion {
    V1,
    V2,
}

#[derive(Debug, Deserialize)]
pub struct InfluxDBConfig {
    pub url: Option<HttpUrl>,
    pub bucket: Option<String>,
    pub organization: Option<String>,
    pub token: Option<String>,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_influxdb_version")]
    pub version: InfluxDBVersion,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

impl UptionConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        if Path::new("/etc/uption").exists() {
            s.merge(File::with_name("/etc/uption/uption"))?;
        } else {
            s.merge(File::with_name("uption"))?;
        }
        // Read development config only for debug builds
        #[cfg(debug_assertions)]
        s.merge(File::with_name("uption.local.").required(false))?;

        // Add in settings from the environment (with a prefix of UPTION)
        let env = Environment::with_prefix("uption");
        let env = env.separator("_");
        s.merge(env)?;

        let config: Self = s.try_into()?;
        config.validate()?;

        Ok(config)
    }

    /// Performs additional validation after configuration is deserialized.
    fn validate(&self) -> Result<(), ConfigError> {
        UptionConfig::validate_exporters(&self.exporters)?;
        Ok(())
    }

    fn validate_exporters(exporters: &ExportersConfig) -> Result<(), ConfigError> {
        match exporters.exporter {
            ExporterSelection::InfluxDB => UptionConfig::validate_influxdb(&exporters.influxdb),
            ExporterSelection::Stdout => Ok(()),
            ExporterSelection::Logger => Ok(()),
        }
    }

    fn validate_influxdb(influxdb: &InfluxDBConfig) -> Result<(), ConfigError> {
        if influxdb.url.is_none() {
            return Err(ConfigError::NotFound("influxdb.url".to_string()));
        }

        if influxdb.version == InfluxDBVersion::V1 {
            if influxdb.database.is_none() {
                return Err(ConfigError::NotFound("influxdb.database".to_string()));
            }
            if influxdb.username.is_none() {
                return Err(ConfigError::NotFound("influxdb.username".to_string()));
            }
            if influxdb.password.is_none() {
                return Err(ConfigError::NotFound("influxdb.password".to_string()));
            }
        }

        if influxdb.version == InfluxDBVersion::V2 {
            if influxdb.bucket.is_none() {
                return Err(ConfigError::NotFound("influxdb.bucket".to_string()));
            }
            if influxdb.organization.is_none() {
                return Err(ConfigError::NotFound("influxdb.organization".to_string()));
            }
            if influxdb.token.is_none() {
                return Err(ConfigError::NotFound("influxdb.token".to_string()));
            }
        }

        Ok(())
    }
}

fn default_timeout() -> u64 {
    30
}

fn default_influxdb_version() -> InfluxDBVersion {
    InfluxDBVersion::V2
}

/// Allows instantiating structs from Uption configuration.
pub trait Configure {
    fn from_config(config: &UptionConfig) -> Self;
}
