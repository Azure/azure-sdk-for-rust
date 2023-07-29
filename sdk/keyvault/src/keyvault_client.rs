use std::sync::Arc;

use azure_core::auth::TokenCredential;
use azure_svc_keyvault::{
    create_key::Response,
    models::{
        key_create_parameters::{Crv, Kty},
        KeyAttributes, KeyCreateParameters, KeyReleasePolicy,
    },
};

pub struct KeyvaultClient {
    client: azure_svc_keyvault::Client,
}

impl KeyvaultClient {
    pub fn new(
        credentials: Arc<dyn TokenCredential>,
        vault_name: impl Into<String>,
    ) -> KeyvaultClient {
        let endpoint = format!("https://{}.vault.azure.net", vault_name.into());
        let scopes = &["https://vault.azure.net"];

        let client = azure_svc_keyvault::Client::builder(credentials)
            .scopes(scopes)
            .endpoint(endpoint)
            .build();

        Self { client }
    }

    pub async fn create_key(
        &self,
        key_name: impl Into<String>,
        key_type: impl Into<Kty>,
        key_size: Option<i32>,
        public_exponent: Option<i32>,
        key_ops: Vec<String>,
        attributes: Option<KeyAttributes>,
        tags: Option<serde_json::Value>,
        crv: Option<Crv>,
        release_policy: Option<KeyReleasePolicy>,
    ) -> azure_core::Result<Response> {
        self.client
            .create_key(
                key_name,
                KeyCreateParameters {
                    kty: key_type.into(),
                    key_size,
                    public_exponent,
                    key_ops,
                    attributes,
                    tags,
                    crv,
                    release_policy,
                },
            )
            .send()
            .await
    }
}
