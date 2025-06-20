use azure_core::{error::Error, http::StatusCode};

/// Custom error type for queue operations
#[derive(Debug)]
pub enum QueueError {
    NotFound(&'static str),
    Forbidden(&'static str),
    Other(Error),
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::NotFound(msg) => write!(f, "Not found: {}", msg),
            QueueError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            QueueError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for QueueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QueueError::Other(e) => Some(e),
            _ => None,
        }
    }
}

impl From<Error> for QueueError {
    fn from(err: Error) -> Self {
        match err.http_status() {
            Some(StatusCode::NotFound) => QueueError::NotFound("Resource not found"),
            Some(StatusCode::Forbidden) => {
                QueueError::Forbidden("Access forbidden - check credentials")
            }
            _ => QueueError::Other(err),
        }
    }
}
