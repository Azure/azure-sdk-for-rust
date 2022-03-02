#[cfg(feature = "auto_refreshing_credential")]
pub mod auto_refreshing_token;
pub mod shared_key;
pub mod token_credential;

#[cfg(feature = "auto_refreshing_credential")]
pub use auto_refreshing_token::AutoRefreshingTokenCredential;
pub use shared_key::SharedKeyAuthorizationPolicy;
pub use token_credential::TokenCredentialAuthorizationPolicy;
