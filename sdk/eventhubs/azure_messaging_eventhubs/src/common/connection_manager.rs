                .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
                        .get_token(&[EVENTHUBS_AUTHORIZATION_SCOPE])
    use azure_core::{http::Url, Result};
        async fn get_token(&self, _scopes: &[&str]) -> Result<AccessToken> {
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions>,
        ) -> Result<AccessToken> {
