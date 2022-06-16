use azure_core::error::Result;
use azure_identity::{device_code_flow, refresh_token};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use oauth2::ClientId;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");

    let storage_account_name = std::env::args()
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
            &format!(
                "https://{}.blob.core.windows.net/user_impersonation",
                storage_account_name
            ),
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
    let mut stream = Box::pin(device_code_flow.stream());
    let mut authorization = None;
    while let Some(Ok(auth)) = stream.next().await {
        println!("{:?}", auth);
        authorization = Some(auth);
    }

    // remove the option (this is safe since we
    // unwrapped the errors before).
    let authorization = authorization.unwrap();

    println!(
        "\nReceived valid bearer token: {}",
        &authorization.access_token().secret()
    );

    if let Some(refresh_token) = authorization.refresh_token().as_ref() {
        println!("Received valid refresh token: {}", &refresh_token.secret());
    }

    // we can now spend the access token in other crates. In
    // this example we are creating an Azure Storage client
    // using the access token.

    let storage_account_client = StorageAccountClient::new_bearer_token(
        http_client.clone(),
        &storage_account_name,
        authorization.access_token().secret() as &str,
    );
    let blob_service_client = storage_account_client.as_blob_service_client();

    // now we enumerate the containers in the
    // specified storage account.
    let containers = blob_service_client.list_containers().execute().await?;
    println!("\nList containers completed succesfully: {:?}", containers);

    // now let's refresh the token, if available
    if let Some(refresh_token) = authorization.refresh_token() {
        let refreshed_token =
            refresh_token::exchange(http_client, &tenant_id, &client_id, None, refresh_token)
                .await?;
        println!("refreshed token == {:#?}", refreshed_token);
    }

    Ok(())
}
