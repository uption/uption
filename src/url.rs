use url::{ParseError as UrlParseError, Url};

#[derive(Debug)]
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
}
