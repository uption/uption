//! Uption configuration.
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;

use crate::url::{Host, HttpUrl};
use config::{Config, ConfigError, Environment, File};
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct UptionConfig {
    pub general: GeneralConfig,
    pub collectors: CollectorsConfig,
    pub exporters: ExportersConfig,
    pub logging: LoggerConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub hostname: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            hostname: get_hostname("uption-host"),
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CollectorsConfig {
    pub interval: u64,
    pub dns: DnsConfig,
    pub http: HttpConfig,
    pub ping: PingConfig,
}

impl Default for CollectorsConfig {
    fn default() -> Self {
        CollectorsConfig {
            interval: 300, // 5 minutes
            dns: DnsConfig::default(),
            http: HttpConfig::default(),
            ping: PingConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PingConfig {
    pub enabled: bool,
    pub hosts: Vec<Host>,
    pub timeout: Timeout,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct HttpConfig {
    pub enabled: bool,
    pub urls: Vec<HttpUrl>,
    pub timeout: Timeout,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct DnsConfig {
    pub enabled: bool,
    pub dns_servers: Vec<Ipv4Addr>,
    pub hosts: Vec<Host>,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
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
#[serde(default)]
pub struct LoggerConfig {
    #[serde(with = "LevelFilterDef")]
    pub level: LevelFilter,
    pub enable_stdout: bool,
    pub log_file: Option<String>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            level: LevelFilter::Info,
            enable_stdout: true,
            log_file: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExporterSelection {
    InfluxDB,
    Stdout,
    Logger,
}

impl Default for ExporterSelection {
    fn default() -> Self {
        ExporterSelection::Stdout
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InfluxDBVersion {
    V1,
    V2,
}

impl Default for InfluxDBVersion {
    fn default() -> Self {
        InfluxDBVersion::V2
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct InfluxDBConfig {
    pub url: Option<HttpUrl>,
    pub bucket: Option<String>,
    pub organization: Option<String>,
    pub token: Option<String>,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub version: InfluxDBVersion,
    pub timeout: Timeout,
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Timeout(pub u64);

impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

impl Into<u64> for Timeout {
    fn into(self) -> u64 {
        self.0
    }
}

// Reads hostname from /etc/hostname and fallback to default on failure.
fn get_hostname(default: &str) -> String {
    let hostname = fs::read_to_string("/etc/hostname").map_or_else(
        |_| default.to_owned(),
        |s| s.lines().take(1).collect::<String>().trim().to_owned(),
    );
    if hostname.is_empty() || hostname.len() > 255 {
        return default.to_owned();
    }
    hostname
}

/// Allows instantiating structs from Uption configuration.
pub trait Configure {
    fn from_config(config: &UptionConfig) -> Self;
}
