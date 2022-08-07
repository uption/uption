use std::fmt;
use std::str;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use url::{Host as UrlHost, ParseError};

#[derive(Debug, Clone)]
pub enum Host {
    Host(UrlHost),
}

impl Host {
    pub fn parse(input: &str) -> Result<Host, ParseError> {
        let host = UrlHost::parse(input)?;
        Ok(Host::Host(host))
    }
}

impl str::FromStr for Host {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Host, ParseError> {
        Host::parse(input)
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Host::Host(host) => write!(f, "{}", host),
        }
    }
}

impl<'de> Deserialize<'de> for Host {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostVisitor;

        impl<'de> Visitor<'de> for HostVisitor {
            type Value = Host;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("host or IP address as a string")
            }

            fn visit_str<E>(self, host: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                UrlHost::parse(host)
                    .map(Host::Host)
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(HostVisitor)
    }
}
