// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp

#[cfg(feature = "enable-fe2o3-amqp")]
use crate::fe2o3::error::Fe2o3AmqpError;

pub enum ErrorKind {
    #[cfg(feature = "enable-fe2o3-amqp")]
    IronOxideError { source: Fe2o3AmqpError },
    #[cfg(not(feature = "enable-fe2o3-amqp"))]
    NoopError,
}

pub struct AmqpError {
    kind: ErrorKind,
}

impl AmqpError {
    #[cfg(feature = "enable-fe2o3-amqp")]
    pub fn new_iron_oxide_error(source: Fe2o3AmqpError) -> Self {
        Self {
            kind: ErrorKind::IronOxideError { source },
        }
    }
}
impl std::error::Error for AmqpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            #[cfg(feature = "enable-fe2o3-amqp")]
            ErrorKind::IronOxideError { source } => {
                write!(f, "AMQP Transport Error {:?}", source)
            }
            #[cfg(not(feature = "enable-fe2o3-amqp"))]
            ErrorKind::NoopError => write!(f, "Noop Error"),
        }
    }
}

impl std::fmt::Debug for AmqpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AMQP Transport Error: {}", self)
    }
}

impl From<AmqpError> for azure_core::Error {
    fn from(e: AmqpError) -> Self {
        Self::new(azure_core::error::ErrorKind::Other, Box::new(e))
    }
}

impl From<ErrorKind> for azure_core::Error {
    fn from(e: ErrorKind) -> Self {
        Self::new(
            azure_core::error::ErrorKind::Other,
            Box::new(AmqpError { kind: e }),
        )
    }
}
