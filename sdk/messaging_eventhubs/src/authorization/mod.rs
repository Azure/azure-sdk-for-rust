//! Types related to auth.
//!
//! FIXME: Many are exact copies from the Event Hubs crate. This should probably moved
//! to a common crate.

pub use azure_named_key_credential::AzureNamedKeyCredential;
pub use azure_sas_credential::AzureSasCredential;

mod azure_named_key_credential;
mod azure_sas_credential;
pub(crate) mod event_hub_claim;
pub(crate) mod event_hub_token_credential;
pub(crate) mod shared_access_credential;
pub(crate) mod shared_access_signature;

cfg_not_wasm32! {
    #[cfg(test)]
    pub(crate) mod tests {
        use mockall::mock;

        mock! {
            pub TokenCredential {}

            #[async_trait::async_trait]
            impl azure_core::auth::TokenCredential for TokenCredential {
                async fn get_token(&self, resource: &str) -> azure_core::Result<azure_core::auth::TokenResponse>;
            }
        }
    }
}
