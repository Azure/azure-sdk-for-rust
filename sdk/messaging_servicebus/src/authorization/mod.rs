//! Authorization primitives.

mod azure_named_key_credential;
mod azure_sas_credential;
pub(crate) mod service_bus_claim;
pub(crate) mod service_bus_token_credential;
pub(crate) mod shared_access_credential;
pub(crate) mod shared_access_signature;

pub use azure_named_key_credential::AzureNamedKeyCredential;
pub use azure_sas_credential::AzureSasCredential;
pub use shared_access_credential::SharedAccessCredential;
pub use service_bus_token_credential::ServiceBusTokenCredential;

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
