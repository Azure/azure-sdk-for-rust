/// An example showcasing how to use a certificate for AAD app authentication.
/// This example fetches a certificate from Azure Keyvault and then uses it to
/// authenticate the app. If you are using subject name validation for the app
/// please make sure to set the send_certificate_chain option to true otherwise
/// the authentication will fail.
use azure_core::auth::TokenCredential;
use azure_identity::{
    CertificateCredentialOptions, ClientCertificateCredential, DefaultAzureCredential,
};
use azure_security_keyvault::KeyvaultClient;
use oauth2::ClientId;
use url::Url;

use std::env;
use std::error::Error;

async fn get_certficate(
    vault_name: &str,
    certificate_name: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let creds = DefaultAzureCredential::default();
    let client = KeyvaultClient::new(
        format!("https://{}.vault.azure.net", vault_name).as_str(),
        std::sync::Arc::new(creds),
    )?
    .secret_client();
    let secret = client.get(certificate_name).into_future().await?;
    let cert = base64::decode(secret.value)?;
    Ok(cert)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    let keyvault_uri =
        env::var("KEYVAULT_URI").expect("Missing KEYVAULT_URI environment variable.");
    let cert_name = env::var("CERT_NAME").expect("Missing CERT_NAME environment variable.");
    let cert = get_certficate(&keyvault_uri, &cert_name).await?;

    let mut options = CertificateCredentialOptions::default();
    // set as true to to send certificate chain
    options.set_send_certificate_chain(true);

    // pass is empty by default when certificate is fetched from keyvault
    let creds = ClientCertificateCredential::new(
        tenant_id.to_string(),
        client_id.to_string(),
        base64::encode(cert),
        "".to_string(),
        options,
    );

    let res = creds.get_token("https://management.azure.com/").await?;
    // Let's enumerate the Azure SQL Databases instances
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let url = Url::parse(&format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
            subscription_id
        ))?;

    let resp = reqwest::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", res.token.secret()))
        .send()
        .await?
        .text()
        .await?;

    println!("\n\nresp {:?}", resp);
    Ok(())
}
