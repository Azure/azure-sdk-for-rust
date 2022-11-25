use azure_core::auth::{TokenCredential, TokenResponse};

use super::shared_access_credential::SharedAccessCredential;

/// <summary>
///   Provides a generic token-based credential for a given Service Bus entity instance.
/// </summary>
///
/// <seealso cref="Azure.Core.TokenCredential" />
///
/// This requires the user to choose the type of token credential because specialization
/// is not supported in stable yet, and thus this may see a revamp in the future once
/// specialization becomes stablized.
///
/// TODO: A temporary work around that could be applied is to only implement the `TokenCredential`
/// trait on `SharedAccessCredential` when it is placed inside a private wrapper type.
pub enum ServiceBusTokenCredential {
    SharedAccessCredential(SharedAccessCredential),

    // TODO: Is the use of trait object here justified?
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
    /// <summary>
    ///   Indicates whether the credential is based on an Service Bus
    ///   shared access policy.
    /// </summary>
    ///
    /// <value><c>true</c> if the credential should be considered a SAS credential; otherwise,
    /// <c>false</c>.</value>
    ///
    pub fn is_shared_access_credential(&self) -> bool {
        match self {
            ServiceBusTokenCredential::SharedAccessCredential(_) => true,
            ServiceBusTokenCredential::Other(_) => false,
        }
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
}

#[cfg(test)]
mod tests {
    use azure_core::auth::AccessToken;
    use time::macros::datetime;

    use crate::authorization::tests::MockTokenCredential;

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
}
