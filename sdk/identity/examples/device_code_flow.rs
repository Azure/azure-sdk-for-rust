use azure_core::new_http_client;
use azure_identity::device_code_flow::start;
use futures::{future::ready, StreamExt};
use oauth2::ClientId;
use std::{env, error::Error};

const SCOPES: &[&str; 2] = &[".default", "offline_access"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));

    let client = new_http_client();

    match start(client.clone(), tenant_id, client_id.as_str(), SCOPES).await {
        Ok(response) => {
            println!("{:?}", response.message());

            response
                .stream()
                .for_each(|result| {
                    match result {
                        Ok(value) => {
                            println!("access token: {:?}", value.access_token().secret());
                            match value.refresh_token() {
                                None => {}
                                Some(tk) => println!("refresh token: {:?}", tk.secret()),
                            }
                        }
                        Err(_) => println!("waiting..."),
                    }
                    ready(())
                })
                .await
        }
        Err(err) => println!("error: {:?}", err),
    }

    Ok(())
}
