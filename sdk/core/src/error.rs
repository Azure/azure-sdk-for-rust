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

/// Return early with an error if a condition is not satisfied.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $msg:literal $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg, $($arg)*));
        }
    };
}

/// Return early with an error if two expressions are not equal to each other.
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left == $right, $kind, $msg, $($arg)*);
    };
}

/// Return early with an error if two expressions are equal to each other.
#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left != $right, $kind, $msg, $($arg)*);
    };
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

    /// Consumes the Error, returning its inner error (if any).
    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        match self.context {
            Context::Custom(Custom { error, .. }) => Some(error),
            Context::Full(Custom { error, .. }, _) => Some(error),
            _ => None,
        }
    }

    /// Returns a reference to the inner error wrapped by this error (if any).
    pub fn get_ref(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) => Some(error.as_ref()),
            Context::Full(Custom { error, .. }, _) => Some(error.as_ref()),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner error wrapped by this error (if any).
    pub fn get_mut(&mut self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        match &mut self.context {
            Context::Custom(Custom { error, .. }) => Some(error.as_mut()),
            Context::Full(Custom { error, .. }, _) => Some(error.as_mut()),
            _ => None,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) => error.source(),
            Context::Full(Custom { error, .. }, _) => error.source(),
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
            Context::Message { message, .. } => write!(f, "{}", message),
            Context::Custom(Custom { error, .. }) => write!(f, "{}", error),
            Context::Full(_, message) => {
                write!(f, "{}", message)
            }
        }
    }
}

/// The kind of error
///
/// The classification of error is intentionally fairly coarse.
#[derive(Copy, Clone, Debug, PartialEq)]
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
        #[error("second error")]
        Io(#[from] std::io::Error),
    }

    fn create_error() -> Error {
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
        let error = std::result::Result::<(), std::io::Error>::Err(inner)
            .context(ErrorKind::Io, "oh no broken pipe!");
        assert_eq!(format!("{}", error.unwrap_err()), "oh no broken pipe!");
    }

    #[test]
    fn downcasting_works() {
        let error = &create_error() as &dyn std::error::Error;
        assert!(error.is::<Error>());
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

    #[test]
    fn ensure_works() {
        fn test_ensure(predicate: bool) -> Result<()> {
            ensure!(predicate, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        fn test_ensure_eq(item1: &str, item2: &str) -> Result<()> {
            ensure_eq!(item1, item2, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        fn test_ensure_ne(item1: &str, item2: &str) -> Result<()> {
            ensure_ne!(item1, item2, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        let err = test_ensure(false).unwrap_err();
        assert_eq!(format!("{}", err), "predicate failed");
        assert_eq!(err.kind(), ErrorKind::Other);

        assert!(test_ensure(true).is_ok());

        let err = test_ensure_eq("foo", "bar").unwrap_err();
        assert_eq!(format!("{}", err), "predicate failed");
        assert_eq!(err.kind(), ErrorKind::Other);

        assert!(test_ensure_eq("foo", "foo").is_ok());

        let err = test_ensure_ne("foo", "foo").unwrap_err();
        assert_eq!(format!("{}", err), "predicate failed");
        assert_eq!(err.kind(), ErrorKind::Other);

        assert!(test_ensure_ne("foo", "bar").is_ok());
    }
}
