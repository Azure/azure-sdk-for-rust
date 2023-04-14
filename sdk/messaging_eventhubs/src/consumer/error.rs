//! Errors associated with consumer

use crate::util::IntoAzureCoreError;

/// The offset string is empty
#[derive(Debug)]
pub struct OffsetIsEmpty;

impl IntoAzureCoreError for OffsetIsEmpty {
    fn into_azure_core_error(self) -> azure_core::Error {
        azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            self
        )
    }
}

impl std::fmt::Display for OffsetIsEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset must not be empty or whitespace")
    }
}

impl std::error::Error for OffsetIsEmpty {}
