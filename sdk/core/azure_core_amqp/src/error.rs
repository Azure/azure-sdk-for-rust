// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp

#[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
use crate::fe2o3::error::Fe2o3AmqpError;

pub enum ErrorKind {
    #[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
    IronOxideError { source: Fe2o3AmqpError },
    #[cfg(any(not(feature = "enable-fe2o3-amqp"), target_arch = "wasm32"))]
    NoopError,
}

pub struct AmqpError {
    kind: ErrorKind,
}

impl AmqpError {
    #[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
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
            #[cfg(all(feature = "enable-fe2o3-amqp", not(target_arch = "wasm32")))]
            ErrorKind::IronOxideError { source } => {
                write!(f, "AMQP Transport Error {:?}", source)
            }
            #[cfg(any(not(feature = "enable-fe2o3-amqp"), target_arch = "wasm32"))]
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
