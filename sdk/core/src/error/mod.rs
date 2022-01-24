use std::borrow::Cow;
use std::fmt::{Debug, Display};

mod azure_core_errors;
mod hyperium_http;
mod macros;

/// A convience alias for `Result` where the error type is hard coded to `Error`
pub type Result<T, O = std::convert::Infallible> = std::result::Result<T, Error<O>>;

/// The kind of error
///
/// The classification of error is intentionally fairly coarse.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind<O> {
    /// An error specific to a certain operation
    Operation(O),
    /// An HTTP status code that was not expected
    UnexpectedOperation { status: u16 },
    /// An error performing IO
    Io,
    /// An error converting data
    DataConversion,
    /// An error getting an API credential token
    Credential,
    /// A catch all for other kinds of errors
    Other,
}

impl<O> ErrorKind<O> {
    pub fn unexpected(status: u16) -> Self {
        Self::UnexpectedOperation { status }
    }
}

impl<O: Display> Display for ErrorKind<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Operation(o) => write!(f, "Operation({})", o),
            ErrorKind::UnexpectedOperation { status } => {
                write!(f, "UnexpectedOperation({})", status)
            }
            ErrorKind::Io => write!(f, "Io"),
            ErrorKind::DataConversion => write!(f, "DataConversion"),
            ErrorKind::Credential => write!(f, "Credential"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

/// An error encountered from interfacing with Azure
#[derive(Debug)]
pub struct Error<O> {
    context: Context<O>,
}

impl<O: Clone + Copy> Error<O> {
    /// Create a new `Error` based on a specific error kind and an underlying error cause
    pub fn new<E>(kind: ErrorKind<O>, error: E) -> Self
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
    pub fn with_message<C>(kind: ErrorKind<O>, message: C) -> Self
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

    /// Create an `Error` based on an error kind and some sort of message
    // NOTE(rylev,yoshuawuyts): we're not thrilled about the internal
    // representation of this, but we're ok with it for now
    pub fn with_data<C>(kind: ErrorKind<O>, message: C, data: String) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        let error: Error<std::convert::Infallible> = Error::new(ErrorKind::Other, data);
        Error {
            context: Context::Full(
                Custom {
                    error: Box::new(error),
                    kind,
                },
                message.into(),
            ),
        }
    }

    /// Get the `ErrorKind` of this `Error`
    pub fn kind(&self) -> ErrorKind<O> {
        match self.context {
            Context::Simple(kind) => kind,
            Context::Message { kind, .. } => kind,
            Context::Custom(Custom { kind, .. }) => kind,
            Context::Full(Custom { kind, .. }, _) => kind,
        }
    }

    /// Consumes the Error, returning its inner error (if any).
    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        match self.context {
            Context::Custom(Custom { error, .. }) => Some(error),
            Context::Full(Custom { error, .. }, _) => Some(error),
            _ => None,
        }
    }

    /// Consumes the error attempting to downcast the inner error as the type provided
    pub fn into_downcast<T: std::error::Error + 'static>(self) -> Option<T> {
        self.into_inner()?.downcast().ok().map(|t| *t)
    }

    /// Returns a reference to the inner error wrapped by this error (if any).
    pub fn get_ref(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) => Some(error.as_ref()),
            Context::Full(Custom { error, .. }, _) => Some(error.as_ref()),
            _ => None,
        }
    }

    /// Returns a reference to the inner error (if any) downcasted to the type provided
    pub fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T> {
        self.get_ref()?.downcast_ref()
    }

    /// Returns a mutable reference to the inner error wrapped by this error (if any).
    pub fn get_mut(&mut self) -> Option<&mut (dyn std::error::Error + Send + Sync + 'static)> {
        match &mut self.context {
            Context::Custom(Custom { error, .. }) => Some(error.as_mut()),
            Context::Full(Custom { error, .. }, _) => Some(error.as_mut()),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner error (if any) downcasted to the type provided
    pub fn downcast_mut<T: std::error::Error + 'static>(&mut self) -> Option<&mut T> {
        self.get_mut()?.downcast_mut()
    }
}

impl<O: Debug + Display> std::error::Error for Error<O> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) => error.source(),
            Context::Full(Custom { error, .. }, _) => error.source(),
            _ => None,
        }
    }
}

impl<O> From<ErrorKind<O>> for Error<O> {
    fn from(kind: ErrorKind<O>) -> Self {
        Self {
            context: Context::Simple(kind),
        }
    }
}

impl<O> From<std::io::Error> for Error<O> {
    fn from(error: std::io::Error) -> Self {
        Self {
            context: Context::Custom(Custom {
                kind: ErrorKind::Io,
                error: Box::new(error),
            }),
        }
    }
}

impl<O> From<serde_json::Error> for Error<O> {
    fn from(error: serde_json::Error) -> Self {
        Self {
            context: Context::Custom(Custom {
                kind: ErrorKind::DataConversion,
                error: Box::new(error),
            }),
        }
    }
}

impl<O: std::fmt::Display> Display for Error<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Context::Simple(kind) => write!(f, "{}", kind),
            Context::Message { message, .. } => write!(f, "{}", message),
            Context::Custom(Custom { error, .. }) => write!(f, "{}", error),
            Context::Full(_, message) => {
                write!(f, "{}", message)
            }
        }
    }
}

/// An extention to the `Result` type that easy allows creating `Error` values from exsiting errors
///
/// This trait should not be implemented on custom types and is meant for usage with `Result`
pub trait ResultExt<T, O> {
    fn context<C>(self, kind: ErrorKind<O>, message: C) -> Result<T, O>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>;

    fn with_context<F, C>(self, kind: ErrorKind<O>, f: F) -> Result<T, O>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>;
}

impl<T, E, O> ResultExt<T, O> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, kind: ErrorKind<O>, message: C) -> Result<T, O>
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

    fn with_context<F, C>(self, kind: ErrorKind<O>, f: F) -> Result<T, O>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        self.context(kind, f())
    }
}

#[derive(Debug)]
enum Context<O> {
    Simple(ErrorKind<O>),
    Message {
        kind: ErrorKind<O>,
        message: Cow<'static, str>,
    },
    Custom(Custom<O>),
    Full(Custom<O>, Cow<'static, str>),
}

#[derive(Debug)]
struct Custom<O> {
    kind: ErrorKind<O>,
    error: Box<dyn std::error::Error + Send + Sync>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct OperationError;

    impl Display for OperationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OperationError")
        }
    }

    #[derive(thiserror::Error, Debug)]
    enum IntermediateError {
        #[error("second error")]
        Io(#[from] std::io::Error),
    }

    fn create_error() -> Error<OperationError> {
        // Create a nested std::io::Error
        let inner = io::Error::new(io::ErrorKind::BrokenPipe, "third error");
        let inner: IntermediateError = inner.into();
        let inner = io::Error::new(io::ErrorKind::ConnectionAborted, inner);

        // Wrap that io::Error in this crate's Error type
        Error::new(ErrorKind::Io, inner)
    }

    #[test]
    fn errors_display_properly() {
        let error = create_error();

        // Generate the display and error chain
        let mut error: &dyn std::error::Error = &error;
        let display = format!("{}", error);
        let mut errors = vec![display.clone()];
        while let Some(cause) = error.source() {
            errors.push(format!("{}", cause));
            error = cause;
        }

        assert_eq!(display, "second error");
        assert_eq!(errors.join(","), "second error,third error");

        let inner = io::Error::new(io::ErrorKind::BrokenPipe, "third error");
        let error: Result<(), OperationError> =
            std::result::Result::<(), std::io::Error>::Err(inner)
                .context(ErrorKind::Io, "oh no broken pipe!");
        assert_eq!(format!("{}", error.unwrap_err()), "oh no broken pipe!");
    }

    #[test]
    fn downcasting_works() {
        let error = &create_error() as &dyn std::error::Error;
        assert!(error.is::<Error<OperationError>>());
        let downcasted = error
            .source()
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .unwrap();
        assert_eq!(format!("{}", downcasted), "third error");
    }

    #[test]
    fn turn_into_inner_error() {
        let error = create_error();
        let inner = error.into_inner().unwrap();
        let inner = inner.downcast_ref::<std::io::Error>().unwrap();
        assert_eq!(format!("{}", inner), "second error");

        let error = create_error();
        let inner = error.get_ref().unwrap();
        let inner = inner.downcast_ref::<std::io::Error>().unwrap();
        assert_eq!(format!("{}", inner), "second error");

        let mut error = create_error();
        let inner = error.get_mut().unwrap();
        let inner = inner.downcast_ref::<std::io::Error>().unwrap();
        assert_eq!(format!("{}", inner), "second error");
    }
}
