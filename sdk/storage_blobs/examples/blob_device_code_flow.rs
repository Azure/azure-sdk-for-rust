use azure_core::{error::ErrorKind, Error};
use azure_identity::{device_code_flow, refresh_token};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use std::env::{args, var};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let client_id = var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = args()
        .nth(1)
        .expect("please specify the storage account name as first command line parameter");

    let http_client = azure_core::new_http_client();

    // the process requires two steps. The first is to ask for
    // the code to show to the user. This is done with the following
    // function. Notice you can pass as many scopes as you want.
    // Since we are asking for the "offline_access" scope we will
    // receive the refresh token as well.
    // We are requesting access to the storage account passed as parameter.
    let device_code_flow = device_code_flow::start(
        http_client.clone(),
        &tenant_id,
        &client_id,
        &[
            &format!("https://{storage_account_name}.blob.core.windows.net/user_impersonation"),
            "offline_access",
        ],
    )
    .await?;

    // now we must show the user the authentication message. It
    // will point the user to the login page and show the code
    // they have to specify.
    println!("{}", device_code_flow.message());

    // now we poll the auth endpoint until the user
    // completes the authentication. The following stream can
    // return, besides errors, a success meaning either
    // Success or Pending. The loop will continue until we
    // get either a Success or an error.
    let mut stream = device_code_flow.stream();
    let authorization = loop {
        match stream.next().await {
            Some(Ok(authorization)) => break authorization,
            Some(Err(_)) => continue,
            None => {
                return Err(Error::with_message(ErrorKind::Credential, || {
                    "device flow stream ended unexpectedly"
                }))
            }
        }
    };

    // we can now spend the access token in other crates. In this example we are
    // creating an Azure Storage client using the access token.
    let storage_credentials =
        StorageCredentials::bearer_token(authorization.access_token().secret());
    let blob_service_client = BlobServiceClient::new(storage_account_name, storage_credentials);

    // now we enumerate the containers in the specified storage account.
    let containers = blob_service_client
        .list_containers()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("\nList containers completed succesfully: {containers:?}");

    // If we want to use the refresh token to get a new access token (such as if
    // we wanted to bump the expiry window on the token), we can do the
    // following
    if let Some(refresh_token) = authorization.refresh_token() {
        let refreshed_token =
            refresh_token::exchange(http_client, &tenant_id, &client_id, None, refresh_token)
                .await?;
        println!("refreshed token == {refreshed_token:#?}");
    }

    Ok(())
}
