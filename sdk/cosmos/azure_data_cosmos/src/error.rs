// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{borrow::Cow, fmt::Display};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// Indicates that the query plan or other information gathered from the gateway is invalid.
    ///
    /// This error is not recoverable and indicates a bug in the gateway.
    InvalidGatewayResponse,

    /// Indicates a deserialization failure, the details of which should be available in [`Error::source`](std::error::Error::source).
    ///
    /// This error is not recoverable and indicates a bug in the Gateway, as it should not be possible to receive a response that cannot be deserialized.
    DeserializationError,

    /// Indicates that a call specified a partition key range ID that is not known to the query pipeline.
    ///
    /// The error is not recoverable and indicates a bug in the language binding or backend, since it should not be possible to specify a partition key range ID that is not known.
    UnknownPartitionKeyRange,

    /// Indicates an internal error in the query pipeline.
    ///
    /// This error is not recoverable, and indicates a bug in the client engine. We return this error only to allow the calling SDK to log the error and report it to the user.
    InternalError,

    /// Indicates a feature is not currently supported by the query engine.
    ///
    /// This error is not recoverable, and indicates a missing or unimplemented feature that can't be used at the moment.
    UnsupportedFeature,

    /// Indicates that the query plan requires features that are not supported by the query engine.
    ///
    /// This error is not recoverable, and should be very rare (or even impossible).
    /// The [`SUPPORTED_FEATURES`](crate::query::SUPPORTED_FEATURES) constant reports the features supported by the engine, and the language binding must provide that information to the gateway when generating a query plan.
    /// The gateway will return an error if the query requires features not listed in the supported features.
    /// We provide this error to cover cases where the language binding is incorrectly reporting the supported features, or edge cases where the engine is not correctly reporting the features it supports.
    UnsupportedQueryPlan,

    /// Indicates that a string parameter is not valid UTF-8.
    ///
    /// This error indicates either a bug in the language binding, or invalid data returned by the backend.
    InvalidUtf8String,

    /// Indicates that one of the provided arguments was null.
    ArgumentNull,

    /// Indicates that an arithmetic overflow occurred during query execution.
    ArithmeticOverflow,

    /// Indicates that a request ID provided to [`QueryPipeline::provide_data`](crate::query::QueryPipeline::provide_data) was invalid.
    InvalidRequestId,

    /// Indicates that the query cannot be executed by this pipeline.
    InvalidQuery,

    /// Indicates that a Python error occurred. The source of the error will be the original Python error.
    PythonError,

    /// Indicates that an illegal argument was provided.
    IllegalArgumentError,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidGatewayResponse => write!(f, "invalid data received from gateway"),
            ErrorKind::DeserializationError => write!(f, "deserialization error"),
            ErrorKind::UnknownPartitionKeyRange => write!(f, "unknown partition key range"),
            ErrorKind::InternalError => write!(f, "internal client engine error"),
            ErrorKind::UnsupportedQueryPlan => write!(f, "unsupported query plan"),
            ErrorKind::InvalidUtf8String => write!(f, "invalid UTF-8 string"),
            ErrorKind::ArgumentNull => write!(f, "provided argument was null"),
            ErrorKind::ArithmeticOverflow => write!(f, "arithmetic overflow occurred"),
            ErrorKind::InvalidRequestId => write!(f, "invalid request ID provided"),
            ErrorKind::InvalidQuery => write!(f, "invalid query"),
            ErrorKind::PythonError => write!(f, "python error"),
            ErrorKind::IllegalArgumentError => write!(f, "illegal argument provided"),
            ErrorKind::UnsupportedFeature => write!(f, "unsupported feature"),
        }
    }
}

impl ErrorKind {
    pub fn with_source(self, source: impl std::error::Error + Send + Sync + 'static) -> Error {
        Error::with_source(self, source)
    }

    pub fn with_message(self, message: impl Into<Cow<'static, str>>) -> Error {
        Error::with_message(self, message)
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
    message: Option<Cow<'static, str>>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        let err = Self {
            kind,
            source: None,
            message: None,
        };

        #[cfg(debug_assertions)]
        panic_if_internal_error(&err);

        err
    }
}

impl Error {
    pub fn with_source(
        kind: ErrorKind,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        let err = Self {
            kind,
            source: Some(Box::new(source)),
            message: None,
        };

        #[cfg(debug_assertions)]
        panic_if_internal_error(&err);

        err
    }

    pub fn with_message(kind: ErrorKind, message: impl Into<Cow<'static, str>>) -> Self {
        let err = Self {
            kind,
            source: None,
            message: Some(message.into()),
        };

        #[cfg(debug_assertions)]
        panic_if_internal_error(&err);

        err
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn into_source(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.source
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{message}"),
            None => write!(f, "{}", self.kind),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let source = self.source.as_ref()?;
        Some(&**source)
    }
}

#[cfg(feature = "python_conversions")]
impl From<pyo3::PyErr> for Error {
    fn from(err: pyo3::PyErr) -> Self {
        ErrorKind::PythonError.with_source(err)
    }
}

#[cfg(feature = "python_conversions")]
impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        use std::error::Error;
        if err.kind() == ErrorKind::PythonError {
            if err.source().is_none() {
                return pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string());
            }
            let err = err.into_source().expect("we just checked that it was Some");
            let err = err
                .downcast::<pyo3::PyErr>()
                .expect("PythonError's source must be a PyErr");
            *err
        } else {
            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
        }
    }
}

#[cfg(debug_assertions)]
fn panic_if_internal_error(err: &Error) {
    // Internal errors are critical enough to warrant a panic in debug builds.
    // The only reason we create an InternalError is if we encounter a panic-worthy error somewhere we can return a `Result`.
    // We do that, because panics across FFI boundaries make things really messy.
    // That doesn't mean we're "panic-free" as there are places we still panic because we can't return an error,
    // and situations where the language injects a panic (bounds checks, for example)
    // but it does avoid a lot of potential panics.
    if err.kind() == ErrorKind::InternalError {
        panic!("internal error: {}", err);
    }
}
