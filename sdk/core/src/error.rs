use std::borrow::Cow;
use std::fmt::{Debug, Display};

/// A convience alias for `Result` where the error type is hard coded to `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// A convenient way to create a new error using the normal formatting infrastructure
#[macro_export]
macro_rules! format_err {
    ($kind:expr, $msg:literal $(,)?) => {{
        // Handle $:literal as a special case to make cargo-expanded code more
        // concise in the common case.
        $crate::error::Error::new($kind, $msg)
    }};
    ($kind:expr, $msg:expr $(,)?) => {{
        $crate::error::Error::new($kind, $msg)
    }};
    ($kind:expr, $msg:expr, $($arg:tt)*) => {{
        $crate::error::Error::new($kind, format!($msg, $($arg)*))
    }};
}

/// An error encountered from interfacing with Azure
#[derive(Debug)]
pub struct Error {
    context: Context,
}

impl Error {
    /// Create a new `Error` based on a specific error kind and an underlying error cause
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            context: Context::Custom(Custom {
                kind,
                error: error.into(),
            }),
        }
    }

    /// Create an `Error` based on an error kind and some sort of message
    pub fn with_message<C>(kind: ErrorKind, message: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: Context::Message {
                kind,
                message: message.into(),
            },
        }
    }

    /// Get the `ErrorKind` of this `Error`
    pub fn kind(&self) -> ErrorKind {
        match self.context {
            Context::Simple(kind) => kind,
            Context::Message { kind, .. } => kind,
            Context::Custom(Custom { kind, .. }) => kind,
            Context::Full(Custom { kind, .. }, _) => kind,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) => Some(error.as_ref()),
            Context::Full(Custom { error, .. }, _) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            context: Context::Simple(kind),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            context: Context::Custom(Custom {
                kind: ErrorKind::Io,
                error: Box::new(error),
            }),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self {
            context: Context::Custom(Custom {
                kind: ErrorKind::Deserialization,
                error: Box::new(error),
            }),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Context::Simple(kind) => write!(f, "{}", kind),
            Context::Message { kind, message } => write!(f, "{}: {}", kind, message),
            Context::Custom(Custom { kind, error }) => write!(f, "{}: {}", kind, error),
            Context::Full(Custom { kind, error }, message) => {
                write!(f, "{}: {}\n{}", kind, message, error)
            }
        }
    }
}

/// The kind of error
///
/// The classification of error is intentionally fairly coarse.
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    HttpStatus { status: u16 },
    Io,
    Serialization,
    Deserialization,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::HttpStatus { status } => write!(f, "HttpStatus({})", status),
            ErrorKind::Io => write!(f, "Io"),
            ErrorKind::Serialization => write!(f, "Serialization"),
            ErrorKind::Deserialization => write!(f, "Deserialization"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

/// An extention to the `Result` type that easy allows creating `Error` values from exsiting errors
///
/// This trait should not be implemented on custom types and is meant for usage with `Result`
pub trait ResultExt<T> {
    fn context<C>(self, kind: ErrorKind, message: C) -> Result<T>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>;

    fn with_context<F, C>(self, kind: ErrorKind, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, kind: ErrorKind, message: C) -> Result<T>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>,
    {
        self.map_err(|e| Error {
            context: Context::Full(
                Custom {
                    error: Box::new(e),
                    kind,
                },
                message.into(),
            ),
        })
    }

    fn with_context<F, C>(self, kind: ErrorKind, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        self.context(kind, f())
    }
}

#[derive(Debug)]
enum Context {
    Simple(ErrorKind),
    Message {
        kind: ErrorKind,
        message: Cow<'static, str>,
    },
    Custom(Custom),
    Full(Custom, Cow<'static, str>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[derive(thiserror::Error, Debug)]
    enum IntermediateError {
        #[error("an intermediate error occurred")]
        Io(#[from] std::io::Error),
    }

    #[test]
    fn errors_display_properly() {
        let inner = io::Error::new(io::ErrorKind::BrokenPipe, "There was an error");
        let inner: IntermediateError = inner.into();
        let inner = io::Error::new(io::ErrorKind::ConnectionAborted, inner);
        let error = Error::new(ErrorKind::Io, inner);
        let display = format!("{}", error);
        assert_eq!(display, "Io: an intermediate error occurred");
    }
}
