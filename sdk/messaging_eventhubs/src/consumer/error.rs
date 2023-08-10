//! Errors associated with consumer

/// The offset string is empty
#[derive(Debug)]
pub struct OffsetIsEmpty;

impl From<OffsetIsEmpty> for azure_core::Error {
    fn from(error: OffsetIsEmpty) -> Self {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, error)
    }
}

impl std::fmt::Display for OffsetIsEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset must not be empty or whitespace")
    }
}

impl std::error::Error for OffsetIsEmpty {}
