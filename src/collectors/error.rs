use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CollectorError {
    ConnectionError(String),
    CollectionError(String),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match self {
            CollectorError::ConnectionError(err) => format!("ConnectionError: {}", err),
            CollectorError::CollectionError(err) => format!("CollectionError: {}", err),
        };
        write!(f, "{}", err)
    }
}

impl error::Error for CollectorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
