use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use url::form_urlencoded::Serializer;
use url::{ParseError as UrlParseError, Url, UrlQuery};

#[derive(Debug, Clone)]
pub struct HttpUrl {
    url: Url,
}

#[derive(Debug)]
pub enum ParseError {
    UrlParseError(UrlParseError),
    InvalidUrlScheme,
}

impl From<UrlParseError> for ParseError {
    fn from(error: UrlParseError) -> Self {
        ParseError::UrlParseError(error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match self {
            ParseError::UrlParseError(err) => err.to_string(),
            ParseError::InvalidUrlScheme => "URL scheme not http or https".to_string(),
        };
        write!(f, "{}", err)
    }
}

impl HttpUrl {
    pub fn parse(input: &str) -> Result<HttpUrl, ParseError> {
        let url = Url::parse(input)?;

        match url.scheme() {
            "http" | "https" => (),
            _ => return Err(ParseError::InvalidUrlScheme),
        };

        Ok(HttpUrl { url })
    }

    pub fn as_str(&self) -> &str {
        &self.url.as_str()
    }

    pub fn query_pairs_mut(&mut self) -> Serializer<UrlQuery> {
        self.url.query_pairs_mut()
    }

    pub fn set_path(&mut self, path: &str) {
        self.url.set_path(path)
    }
}

impl<'de> Deserialize<'de> for HttpUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HttpUrlVisitor;

        impl<'de> Visitor<'de> for HttpUrlVisitor {
            type Value = HttpUrl;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("HTTP or HTTPS URL as a string")
            }

            fn visit_str<E>(self, url: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                HttpUrl::parse(url).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(HttpUrlVisitor)
    }
}
