use crate::url::{Host, HttpUrl};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UptionConfig {
    pub general: GeneralConfig,
    pub collectors: CollectorsConfig,
    pub exporters: ExportersConfig,
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
pub enum ExporterSelection {
    InfluxDB,
    Stdout,
}

#[derive(Debug, Deserialize)]
pub struct InfluxDBConfig {
    pub url: Option<HttpUrl>,
    pub bucket: Option<String>,
    pub organization: Option<String>,
    pub token: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

impl UptionConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("uption"))?;

        // Add local configuration file
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
        return match exporters.exporter {
            ExporterSelection::InfluxDB => UptionConfig::validate_influxdb(&exporters.influxdb),
            ExporterSelection::Stdout => Ok(()),
        };
    }

    fn validate_influxdb(influxdb: &InfluxDBConfig) -> Result<(), ConfigError> {
        if influxdb.url.is_none() {
            return Err(ConfigError::NotFound("influxdb.url".to_string()));
        }
        if influxdb.bucket.is_none() {
            return Err(ConfigError::NotFound("influxdb.bucket".to_string()));
        }
        if influxdb.organization.is_none() {
            return Err(ConfigError::NotFound("influxdb.organization".to_string()));
        }
        if influxdb.token.is_none() {
            return Err(ConfigError::NotFound("influxdb.token".to_string()));
        }

        Ok(())
    }
}

fn default_timeout() -> u64 {
    30
}
