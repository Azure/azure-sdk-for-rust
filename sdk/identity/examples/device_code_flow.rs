use azure_core::{
    error::{Error, ErrorKind},
    new_http_client,
};
use azure_identity::device_code_flow::start;
use futures::StreamExt;
use std::env::var;

const SCOPES: &[&str; 2] = &[".default", "offline_access"];

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let tenant_id = var("TENANT_ID").expect("Missing TENANT_ID environment variable");
    let client_id = var("CLIENT_ID").expect("Missing CLIENT_ID environment variable");

    let client = new_http_client();

    let response = start(client, tenant_id, &client_id, SCOPES).await?;
    println!("{}", response.message());

    let mut stream = response.stream();
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

    println!("access token: {:?}", authorization.access_token().secret());

    match authorization.refresh_token() {
        None => {}
        Some(tk) => println!("refresh token: {:?}", tk.secret()),
    }

    Ok(())
}
