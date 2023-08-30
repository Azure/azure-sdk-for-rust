use azure_core::auth::{TokenCredential, TokenResponse};

use super::shared_access_credential::SharedAccessCredential;

// FIXME: This is an exact copy from the Service Bus crate. This should probably moved
// to a common crate.
/// Provides a generic token-based credential for a given Event Hub instance.
///
/// This supports [`SharedAccessCredential`] and any other credential type that implements
/// [`TokenCredential`], eg. [`azure_identity::DefaultAzureCredential`].
///
/// # Example
///
/// ```rust
/// use azure_identity::DefaultAzureCredential;
///
/// let credential = EventHubTokenCredential::from(DefaultAzureCredential::default());
/// ```
pub enum EventHubTokenCredential {
    // FIXME: This is a temporary workaround until specialization is stablized.
    /// Shared Access Signature credential.
    SharedAccessCredential(SharedAccessCredential),

    // TODO: Is the use of trait object here justified?
    /// Other credential types.
    Other(Box<dyn TokenCredential>),
}

impl std::fmt::Debug for EventHubTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SharedAccessCredential(_arg0) => f.debug_tuple("SharedAccessCredential").finish(),
            Self::Other(_arg0) => f.debug_tuple("Other").finish(),
        }
    }
}

impl From<SharedAccessCredential> for EventHubTokenCredential {
    fn from(source: SharedAccessCredential) -> Self {
        Self::SharedAccessCredential(source)
    }
}

impl<TC> From<TC> for EventHubTokenCredential
where
    TC: TokenCredential + 'static,
{
    fn from(source: TC) -> Self {
        Self::Other(Box::new(source) as Box<dyn TokenCredential>)
    }
}

impl EventHubTokenCredential {
    /// Creates a new instance of [`EventHubTokenCredential`]. This is simply an alias for
    /// [`From::from`]
    pub fn new(source: impl Into<Self>) -> Self {
        source.into()
    }

    /// Indicates whether the credential is based on an Event Hubs
    /// shared access policy.
    pub fn is_shared_access_credential(&self) -> bool {
        matches!(self, EventHubTokenCredential::SharedAccessCredential(_))
    }
}

impl EventHubTokenCredential {
    pub(crate) const DEFAULT_SCOPE: &str = "https://eventhubs.azure.net/.default";

    /// Gets a `TokenResponse` for the specified resource
    pub(crate) async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        match self {
            EventHubTokenCredential::SharedAccessCredential(credential) => {
                credential.get_token(resource).await
            }
            EventHubTokenCredential::Other(credential) => credential.get_token(resource).await,
        }
    }

    pub(crate) async fn get_token_using_default_scope(&self) -> azure_core::Result<TokenResponse> {
        self.get_token(Self::DEFAULT_SCOPE).await
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

        use super::EventHubTokenCredential;

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

            let credential = EventHubTokenCredential::from(mock_credentials);
            let token_result = credential.get_token(resource).await;
            assert_eq!(token_result.unwrap().token.secret(), token_value);
        }

        #[test]
        fn is_shared_access_credential_recognized_as_sas_credentials() {
            let signature = SharedAccessSignature::try_from_parts(
                "sb-name",
                "keyName",
                "key",
                Some(std::time::Duration::from_secs(4 * 60 * 60)),
            )
            .unwrap();
            let sas_credential = SharedAccessCredential::from(signature);
            let credential = EventHubTokenCredential::new(sas_credential);
            assert!(credential.is_shared_access_credential());
        }

        #[tokio::test]
        async fn create_credential_with_azure_identity() {
            use azure_identity::DefaultAzureCredential;

            let default_credential = DefaultAzureCredential::default();
            let _event_hub_token_credential = EventHubTokenCredential::from(default_credential);
        }
    }
}
