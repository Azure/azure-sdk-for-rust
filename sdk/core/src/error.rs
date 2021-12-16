use std::{borrow::Cow, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;

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

#[derive(Debug)]
pub struct Error {
    context: Context,
}

impl Error {
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

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
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

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    HttpStatus { status: u16 },
    Encoding,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::HttpStatus { status } => write!(f, "HttpStatus({})", status),
            ErrorKind::Encoding => write!(f, "Encoding"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

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
