use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ReceiverError {
    ConnectionError(String),
    CollectionError(String),
}

impl fmt::Display for ReceiverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match self {
            ReceiverError::ConnectionError(err) => format!("ConnectionError: {}", err),
            ReceiverError::CollectionError(err) => format!("CollectionError: {}", err),
        };
        write!(f, "{}", err)
    }
}

impl error::Error for ReceiverError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
