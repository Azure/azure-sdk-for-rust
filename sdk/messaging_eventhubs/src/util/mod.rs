pub(crate) mod error;
pub(crate) mod time;

pub trait IntoAzureCoreError {
    fn into_azure_core_error(self) -> azure_core::Error;
}
