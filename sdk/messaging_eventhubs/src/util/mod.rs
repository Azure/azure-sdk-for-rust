pub(crate) mod time;
pub(crate) mod error;

pub trait IntoAzureCoreError {
    fn into_azure_core_error(self) -> azure_core::Error;
}
