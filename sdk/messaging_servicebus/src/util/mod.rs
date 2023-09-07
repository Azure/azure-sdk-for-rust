pub(crate) mod error;
pub(crate) mod time;

/// A helper trait to convert a type into an `azure_core::Error`.
pub(crate) trait IntoAzureCoreError {
    /// Converts the type into an `azure_core::Error`.
    fn into_azure_core_error(self) -> azure_core::Error;
}
