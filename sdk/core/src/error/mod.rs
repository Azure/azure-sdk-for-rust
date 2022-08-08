use crate::StatusCode;
use std::borrow::Cow;
use std::fmt::{Debug, Display};
mod http_error;
mod macros;
pub use http_error::HttpError;

/// A convenience alias for `Result` where the error type is hard coded to `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// The kind of error
///
/// The classification of error is intentionally fairly coarse.
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    /// An HTTP status code that was not expected
    HttpResponse {
        status: StatusCode,
        error_code: Option<String>,
    },
    /// An error performing IO
    Io,
    /// An error converting data
    DataConversion,
    /// An error getting an API credential token
    Credential,
    /// An error having to do with the mock framework
    MockFramework,
    /// A catch all for other kinds of errors
    Other,
}

impl ErrorKind {
    pub fn into_error(self) -> Error {
        Error {
            context: Context::Simple(self),
        }
    }

    pub fn http_response(status: StatusCode, error_code: Option<String>) -> Self {
        Self::HttpResponse { status, error_code }
    }

    pub fn http_response_from_body(status: StatusCode, body: &[u8]) -> Self {
        let error_code = http_error::get_error_code_from_body(body);
        Self::HttpResponse { status, error_code }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::HttpResponse { status, error_code } => {
                write!(
                    f,
                    "HttpResponse({},{})",
                    status,
                    error_code.as_deref().unwrap_or("unknown")
                )
            }
            ErrorKind::Io => write!(f, "Io"),
            ErrorKind::DataConversion => write!(f, "DataConversion"),
            ErrorKind::Credential => write!(f, "Credential"),
            ErrorKind::MockFramework => write!(f, "MockFramework"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
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

    /// Create a new `Error` based on a specific error kind and an underlying error cause
    /// along with a message
    pub fn full<E, C>(kind: ErrorKind, error: E, message: C) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: Context::Full(
                Custom {
                    kind,
                    error: error.into(),
                },
                message.into(),
            ),
        }
    }

    /// Create an `Error` based on an error kind and some sort of message
    pub fn message<C>(kind: ErrorKind, message: C) -> Self
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

    /// Creates an `Error` based on an error kind and formatted message
    pub fn with_message<F, C>(kind: ErrorKind, message: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: Context::Message {
                kind,
                message: message().into(),
            },
        }
    }

    /// Get the `ErrorKind` of this `Error`
    pub fn kind(&self) -> &ErrorKind {
        match &self.context {
            Context::Simple(kind)
            | Context::Message { kind, .. }
            | Context::Custom(Custom { kind, .. })
            | Context::Full(Custom { kind, .. }, _) => kind,
        }
    }

    /// Consumes the Error, returning its inner error (if any).
    pub fn into_inner(self) -> std::result::Result<Box<dyn std::error::Error + Send + Sync>, Self> {
        match self.context {
            Context::Custom(Custom { error, .. }) | Context::Full(Custom { error, .. }, _) => {
                Ok(error)
            }
            _ => Err(self),
        }
    }

    /// Consumes the error attempting to downcast the inner error as the type provided
    ///
    /// Returns `Err(self)` if the downcast is not possible
    pub fn into_downcast<T: std::error::Error + 'static>(self) -> std::result::Result<T, Self> {
        if self.downcast_ref::<T>().is_none() {
            return Err(self);
        }
        // Unwrapping is ok here since we already check above that the downcast will work
        Ok(*self
            .into_inner()?
            .downcast()
            .expect("failed to unwrap downcast"))
    }

    /// Returns a reference to the inner error wrapped by this error (if any).
    pub fn get_ref(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) | Context::Full(Custom { error, .. }, _) => {
                Some(error.as_ref())
            }
            _ => None,
        }
    }

    /// Cast this error as an `HttpError`
    ///
    /// This searches the entire ["source" chain](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
    /// looking for an `HttpError`.
    pub fn as_http_error(&self) -> Option<&HttpError> {
        let mut error = self.get_ref()? as &(dyn std::error::Error);
        loop {
            match error.downcast_ref::<HttpError>() {
                Some(e) => return Some(e),
                None => error = error.source()?,
            }
        }
    }

    /// Returns a reference to the inner error (if any) downcasted to the type provided
    pub fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T> {
        self.get_ref()?.downcast_ref()
    }

    /// Returns a mutable reference to the inner error wrapped by this error (if any).
    pub fn get_mut(&mut self) -> Option<&mut (dyn std::error::Error + Send + Sync + 'static)> {
        match &mut self.context {
            Context::Custom(Custom { error, .. }) | Context::Full(Custom { error, .. }, _) => {
                Some(error.as_mut())
            }
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner error (if any) downcasted to the type provided
    pub fn downcast_mut<T: std::error::Error + 'static>(&mut self) -> Option<&mut T> {
        self.get_mut()?.downcast_mut()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            Context::Custom(Custom { error, .. }) | Context::Full(Custom { error, .. }, _) => {
                Some(&**error)
            }
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
        Self::new(ErrorKind::Io, error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::new(ErrorKind::DataConversion, error)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::new(ErrorKind::DataConversion, error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::new(ErrorKind::DataConversion, error)
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

/// An extension to the `Result` type that easy allows creating `Error` values from existing errors
///
/// This trait cannot be implemented on custom types and is meant for usage with `Result`
pub trait ResultExt<T>: private::Sealed {
    /// Creates a new error with the specified kind
    fn map_kind(self, kind: ErrorKind) -> Result<T>
    where
        Self: Sized;

    /// Creates a new error with the specified kind and message
    fn context<C>(self, kind: ErrorKind, message: C) -> Result<T>
    where
        Self: Sized,
        C: Into<Cow<'static, str>>;

    /// Creates a new error with the specified kind and formatted message
    fn with_context<F, C>(self, kind: ErrorKind, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>;
}

mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for std::result::Result<T, E> where E: std::error::Error + Send + Sync + 'static {}
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn map_kind(self, kind: ErrorKind) -> Result<T>
    where
        Self: Sized,
    {
        self.map_err(|e| Error::new(kind, e))
    }

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

    #[allow(dead_code, unconditional_recursion)]
    fn ensure_send<T: Send>() {
        ensure_send::<Error>();
    }

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
        let mut errors = vec![];
        while let Some(cause) = error.source() {
            errors.push(format!("{}", cause));
            error = cause;
        }

        assert_eq!(display, "second error");
        assert_eq!(errors.join(","), "second error,third error");

        let inner = io::Error::new(io::ErrorKind::BrokenPipe, "third error");
        let error: Result<()> = std::result::Result::<(), std::io::Error>::Err(inner)
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
        assert_eq!(format!("{}", downcasted), "second error");
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
    fn matching_against_http_error() {
        let kind = ErrorKind::http_response_from_body(StatusCode::ImATeapot, b"{}");

        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code: None
            }
        ));

        let kind = ErrorKind::http_response_from_body(
            StatusCode::ImATeapot,
            br#"{"error": {"code":"teepot"}}"#,
        );

        assert!(matches!(
            kind,
            ErrorKind::HttpResponse {
                status: StatusCode::ImATeapot,
                error_code
            }
            if error_code.as_deref() == Some("teepot")
        ));
    }

    #[test]
    fn set_result_kind() {
        let result = std::result::Result::<(), _>::Err(create_error());
        let result = result.map_kind(ErrorKind::Io);
        assert_eq!(&ErrorKind::Io, result.unwrap_err().kind());
    }
}
