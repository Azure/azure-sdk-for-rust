use std::sync::Arc;

use azure_core::auth::TokenCredential;

pub struct KeyvaultClient<T: TokenCredential> {
    credentials: Arc<T>,
    vault_name: String,
}

// impl KeyvaultClient {
// pub fn new<T: TokenCredential>(credentials: Arc<T>, vault_name: &str) -> KeyvaultClient<T> {
// let keyvault_name = std::env::args()
//     .nth(1)
//     .expect("please specify an existing keyvault");
// let key_name = std::env::args()
//     .nth(2)
//     .expect("please specify the name of the key to create");
// let endpoint = format!("https://{keyvault_name}.vault.azure.net");
// let scopes = &["https://vault.azure.net"];
// let client = azure_svc_keyvault::Client::builder(credential)
//     .endpoint(endpoint)
//     .scopes(scopes)
//     .build();

// // Configure the not-before (nbf) and expiration (exp) dates
// let nbf = OffsetDateTime::now_utc();
// let exp = nbf + date::duration_from_days(90);

// let mut key_attributes = KeyAttributes::new();
// key_attributes.attributes = Attributes::new();
// key_attributes.attributes.nbf = Some(nbf.unix_timestamp());
// key_attributes.attributes.exp = Some(exp.unix_timestamp());

// // Configure key type and size
// let mut key_create_params = KeyCreateParameters::new(Kty::Rsa);
// key_create_params.key_size = Some(2048);
// key_create_params.attributes = Some(key_attributes);

// println!("Creating key '{key_name}' in key vault '{keyvault_name}'.");
// client
//     .create_key(&key_name, key_create_params)
//     .send()
//     .await?;
// println!("Key '{key_name}' created!");

// Ok(())
// }
// }

#[cfg(test)]
pub mod test {
    use std::{str::FromStr, sync::Arc};

    use azure_core::new_http_client;
    use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
    use azure_svc_keyvault::models::{key_create_parameters::Kty, KeyCreateParameters};

    #[test]
    fn test_create_key() {
        let client = new_http_client();

        let tenant_id = std::env::var("TENANT_ID").expect("Missing TENANT_ID env var");
        let client_id = std::env::var("CLIENT_ID").expect("Missing CLIENT_ID env var");
        let client_secret = std::env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET env var");

        let credentials = ClientSecretCredential::new(
            client,
            tenant_id,
            client_id,
            client_secret,
            TokenCredentialOptions::default(),
        );

        let keyvault_client = azure_svc_keyvault::Client::builder(Arc::new(credentials)).build();

        keyvault_client.create_key(
            "test_key_2",
            KeyCreateParameters::new(Kty::from_str("RSA").unwrap()),
        );
    }
}
