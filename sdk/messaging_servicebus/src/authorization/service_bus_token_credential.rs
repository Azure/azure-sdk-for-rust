use azure_core::auth::{TokenCredential, TokenResponse};

use crate::constants::DEFAULT_RESOURCE;

use super::shared_access_credential::SharedAccessCredential;

/// Token-based credential for Service Bus.
///
/// This supports `SharedAccessCredential` and other credential types that
/// implement `TokenCredential` (eg. `azure_identity::DefaultAzureCredential`).
///
/// # Example
///
/// ```rust,no_run
/// use azure_identity::DefaultAzureCredential;
/// use azure_messaging_servicebus::authorization::ServiceBusTokenCredential;
///
/// let credential = ServiceBusTokenCredential::from(DefaultAzureCredential::default());
/// ```
pub enum ServiceBusTokenCredential {
    /// Shared Access Signature credential.
    ///
    /// FIXME: This is a temporary workaround until specialization is stablized.
    SharedAccessCredential(SharedAccessCredential),

    /// Other credential types.
    ///
    /// TODO: Is the use of trait object here justified?
    Other(Box<dyn TokenCredential>),
}

impl std::fmt::Debug for ServiceBusTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SharedAccessCredential(_arg0) => f.debug_tuple("SharedAccessCredential").finish(),
            Self::Other(_arg0) => f.debug_tuple("Other").finish(),
        }
    }
}

impl From<SharedAccessCredential> for ServiceBusTokenCredential {
    fn from(source: SharedAccessCredential) -> Self {
        Self::SharedAccessCredential(source)
    }
}

impl<TC> From<TC> for ServiceBusTokenCredential
where
    TC: TokenCredential + 'static,
{
    fn from(source: TC) -> Self {
        Self::Other(Box::new(source) as Box<dyn TokenCredential>)
    }
}

impl ServiceBusTokenCredential {
    /// Creates a new `ServiceBusTokenCredential` from the given credential. This is an alias for
    /// `From::from`.
    pub fn new(source: impl Into<Self>) -> Self {
        source.into()
    }

    /// Indicates whether the credential is based on an Service Bus
    /// shared access policy.
    pub fn is_shared_access_credential(&self) -> bool {
        matches!(self, ServiceBusTokenCredential::SharedAccessCredential(_))
    }
}

impl ServiceBusTokenCredential {
    /// Gets a `TokenResponse` for the specified resource
    pub(crate) async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        match self {
            ServiceBusTokenCredential::SharedAccessCredential(credential) => {
                credential.get_token(resource).await
            }
            ServiceBusTokenCredential::Other(credential) => credential.get_token(resource).await,
        }
    }

    pub(crate) async fn get_token_using_default_resource(&self) -> azure_core::Result<TokenResponse> {
        self.get_token(DEFAULT_RESOURCE).await
    }
}

cfg_not_wasm32! {
    #[cfg(test)]
    mod tests {
        use azure_core::auth::AccessToken;
        use time::macros::datetime;

        use crate::authorization::{
            shared_access_credential::SharedAccessCredential,
            shared_access_signature::SharedAccessSignature, tests::MockTokenCredential,
        };

        use super::ServiceBusTokenCredential;

        #[tokio::test]
        async fn get_token_delegates_to_the_source_credential() {
            let token_value = "token";
            let mut mock_credentials = MockTokenCredential::new();
            let resource = "the resource value";
            let token_response = azure_core::auth::TokenResponse {
                token: AccessToken::new(token_value),
                expires_on: datetime!(2015-10-27 00:00:00).assume_utc(),
            };
            mock_credentials
                .expect_get_token()
                .times(1)
                .returning(move |_resource| Ok(token_response.clone()));

            let credential = ServiceBusTokenCredential::from(mock_credentials);
            let token_result = credential.get_token(resource).await;
            assert_eq!(token_result.unwrap().token.secret(), token_value);
        }

        #[tokio::test]
        async fn is_shared_access_credential_recognized_as_sas_credentials() {
            let signature = SharedAccessSignature::try_from_parts(
                "sb-name",
                "keyName",
                "key",
                Some(std::time::Duration::from_secs(4 * 60 * 60)),
            )
            .unwrap();
            let sas_credential = SharedAccessCredential::from(signature);
            let credential = ServiceBusTokenCredential::new(sas_credential);
            assert!(credential.is_shared_access_credential());
        }
    }
}
