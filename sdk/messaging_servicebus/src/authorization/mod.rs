pub(crate) mod service_bus_claim;
pub(crate) mod service_bus_token_credential;
pub(crate) mod shared_access_credential;
pub(crate) mod shared_access_signature;

pub(crate) mod private {
    /// A thin wrapper that tries to "specialize" the implementation of `TokenCredential` for
    /// `SharedAccessCredential` which itself won't implement the trait until specialization is
    /// stabilized.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TokenCredentialWrapper<TC>(pub TC);
}

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
