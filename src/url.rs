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
