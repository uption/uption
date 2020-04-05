use std::any;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String,
    source: Option<String>,
    cause: Option<Box<dyn error::Error>>,
    context: Option<String>,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
            source: None,
            cause: None,
            context: None,
        }
    }

    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_string());
        self
    }

    pub fn cause(mut self, cause: impl error::Error + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error_info = Vec::new();

        if let Some(source) = self.source.as_ref() {
            error_info.push(format!("source=\"{}\"", source));
        }

        if let Some(context) = self.context.as_ref() {
            error_info.push(format!("context=\"{}\"", context));
        }

        if let Some(err) = self.cause.as_ref() {
            error_info.push(format!("cause=\"{}\"", err.to_string()));
        }

        write!(f, "{} ({})", self.msg, error_info.join(" "))
    }
}

impl<E: error::Error + 'static> From<E> for Error {
    fn from(err: E) -> Self {
        Error::new(&err.to_string()).context(any::type_name::<E>())
    }
}

pub trait ResultError {
    fn source(self, source: &str) -> Self;

    fn cause(self, cause: impl error::Error + 'static) -> Self;

    fn context(self, context: &str) -> Self;
}

impl<T> ResultError for Result<T> {
    fn source(self, source: &str) -> Self {
        self.map_err(|e| e.source(source))
    }

    fn cause(self, cause: impl error::Error + 'static) -> Self {
        self.map_err(|e| e.cause(cause))
    }

    fn context(self, context: &str) -> Self {
        self.map_err(|e| e.context(context))
    }
}
