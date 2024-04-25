/// An example showcasing how to use a certificate for AAD app authentication.
/// This example fetches a certificate from Azure Keyvault and then uses it to
/// authenticate the app. If you are using subject name validation for the app
/// please make sure to set the `send_certificate_chain` option to true otherwise
/// the authentication will fail.
use azure_core::auth::{Secret, TokenCredential};
use azure_identity::{ClientCertificateCredential, ClientCertificateCredentialOptions};
use azure_security_keyvault::KeyvaultClient;
use std::env::var;
use url::Url;

async fn get_certficate(vault_name: &str, certificate_name: &str) -> azure_core::Result<Secret> {
    let credential = azure_identity::create_credential()?;
    let client = KeyvaultClient::new(
        format!("https://{}.vault.azure.net", vault_name).as_str(),
        credential,
    )?
    .certificate_client();
    let response = client.get(certificate_name).await?;
    println!("GOT {response:#?}");
    Ok(response.cer)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let client_id = var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    let keyvault_name = var("KEYVAULT_NAME").expect("Missing KEYVAULT_NAME environment variable.");
    let cert_name = var("CERT_NAME").expect("Missing CERT_NAME environment variable.");
    let cert = get_certficate(&keyvault_name, &cert_name).await?;

    let mut options = ClientCertificateCredentialOptions::default();
    // set as true to to send certificate chain
    options.set_send_certificate_chain(true);

    // pass is empty by default when certificate is fetched from keyvault
    let creds =
        ClientCertificateCredential::new(tenant_id, client_id, cert, String::new(), options);

    let res = creds?
        .get_token(&["https://management.azure.com/.default"])
        .await?;
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
