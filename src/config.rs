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

impl Validate for UptionConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        self.general.validate()?;
        self.logging.validate()?;
        self.collectors.validate()?;
        self.exporters.validate()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub hostname: String,
}

impl Validate for GeneralConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.hostname.len() > 255 {
            return Err(ConfigError::Message(
                "hostname maximum length is 255 bytes".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            hostname: get_hostname("uption-host"),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
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

impl Validate for LoggerConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.log_file.is_some() && self.log_file.as_ref().unwrap().is_empty() {
            return Err(ConfigError::Message(
                "logging.log_file can't be empty".to_string(),
            ));
        }
        Ok(())
    }
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
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CollectorsConfig {
    pub interval: u64,
    pub dns: DnsConfig,
    pub http: HttpConfig,
    pub ping: PingConfig,
    pub wireless: WirelessConfig,
}

impl Validate for CollectorsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.interval < 1 || self.interval > 86400 {
            return Err(ConfigError::Message(
                "collectors.interval minimum value is 1 and maximum value is 86400".to_string(),
            ));
        }
        self.dns.validate()?;
        self.http.validate()?;
        self.ping.validate()?;
        self.wireless.validate()?;
        Ok(())
    }
}

impl Default for CollectorsConfig {
    fn default() -> Self {
        CollectorsConfig {
            interval: 300, // 5 minutes
            dns: DnsConfig::default(),
            http: HttpConfig::default(),
            ping: PingConfig::default(),
            wireless: WirelessConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct DnsConfig {
    pub enabled: bool,
    pub dns_servers: Vec<Ipv4Addr>,
    pub hosts: Vec<Host>,
    pub timeout: Timeout,
}

impl Validate for DnsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.enabled {
            if self.dns_servers.is_empty() {
                return Err(ConfigError::Message(
                    "dns.dns_servers can't be empty".to_string(),
                ));
            } else if self.hosts.is_empty() {
                return Err(ConfigError::Message("dns.hosts can't be empty".to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct HttpConfig {
    pub enabled: bool,
    pub urls: Vec<HttpUrl>,
    pub timeout: Timeout,
}

impl Validate for HttpConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.enabled && self.urls.is_empty() {
            return Err(ConfigError::Message("http.urls can't be empty".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PingConfig {
    pub enabled: bool,
    pub hosts: Vec<Host>,
    pub timeout: Timeout,
}

impl Validate for PingConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.enabled && self.hosts.is_empty() {
            return Err(ConfigError::Message(
                "ping.hosts can't be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct WirelessConfig {
    pub enabled: bool,
}

impl Validate for WirelessConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ExportersConfig {
    pub exporter: ExporterSelection,
    pub influxdb: InfluxDbConfig,
}

impl Validate for ExportersConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        match self.exporter {
            ExporterSelection::InfluxDb => self.influxdb.validate(),
            ExporterSelection::Stdout => Ok(()),
            ExporterSelection::Logger => Ok(()),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExporterSelection {
    InfluxDb,
    #[default]
    Stdout,
    Logger,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct InfluxDbConfig {
    pub url: Option<HttpUrl>,
    pub bucket: String,
    pub organization: String,
    pub token: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub version: InfluxDbVersion,
    pub timeout: Timeout,
}

impl Validate for InfluxDbConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.url.is_none() {
            return Err(ConfigError::NotFound("influxdb.url".to_string()));
        }

        if self.version == InfluxDbVersion::V1 {
            if self.database.is_empty() {
                return Err(ConfigError::NotFound("influxdb.database".to_string()));
            }
            if self.username.is_empty() {
                return Err(ConfigError::NotFound("influxdb.username".to_string()));
            }
            if self.password.is_empty() {
                return Err(ConfigError::NotFound("influxdb.password".to_string()));
            }
        }

        if self.version == InfluxDbVersion::V2 {
            if self.bucket.is_empty() {
                return Err(ConfigError::NotFound("influxdb.bucket".to_string()));
            }
            if self.organization.is_empty() {
                return Err(ConfigError::NotFound("influxdb.organization".to_string()));
            }
            if self.token.is_empty() {
                return Err(ConfigError::NotFound("influxdb.token".to_string()));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum InfluxDbVersion {
    V1,
    #[default]
    V2,
}

impl UptionConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::builder();

        if Path::new("/etc/uption").exists() {
            s = s.add_source(File::with_name("/etc/uption/uption"));
        } else {
            s = s.add_source(File::with_name("uption"));
        }
        // Read development config only for debug builds
        #[cfg(debug_assertions)]
        {
            s = s.add_source(File::with_name("uption.local.").required(false));
        }

        // Add in settings from the environment (with a prefix of UPTION)
        let env = Environment::with_prefix("uption");
        let env = env.separator("_");
        s = s.add_source(env);

        let config: Self = s.build()?.try_deserialize()?;
        config.validate()?;

        Ok(config)
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Timeout(pub u64);

impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

impl From<Timeout> for u64 {
    fn from(timeout: Timeout) -> u64 {
        timeout.0
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

/// Configuration object validation.
trait Validate {
    fn validate(&self) -> Result<(), ConfigError>;
}
