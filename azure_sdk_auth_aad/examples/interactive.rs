use azure_sdk_auth_aad::*;
use azure_sdk_core::perform_http_request;
use futures::future::{ok, Future};
use hyper::{Body, Client, Request};
use oauth2::{ClientId, ClientSecret, TokenResponse};
use std::env;
use std::sync::Arc;
use tokio_core::reactor::Core;
use url::Url;

fn main() {
    let client_id = ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."));
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id = env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

    let mut core = Core::new().unwrap();
    let client: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> =
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::new(4));
    let client = Arc::new(client);

    // Create URL to browse for initial authorization
    let c = authorize_delegate(
        client_id,
        client_secret,
        &tenant_id,
        Url::parse("http://localhost:3003/redirect").unwrap(),
        "https://management.azure.com/",
    );

    println!("c == {:?}", c);
    println!("\nbrowse this url:\n{}", c.authorize_url);

    // Start a naive server to receive the redirect
    // with the token
    let code = naive_server(&c, 3003).unwrap();

    println!("code received: {:?}", code);

    // Exchange the token with one that can be
    // used for authorization
    let token = exchange(c, code).unwrap();

    println!("token received: {:?}", token);

    // Let's enumerate the Azure SQL Databases instances
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let request = Request::builder()
        .method("GET")
        .header("Authorization", format!("Bearer {}", token.access_token().secret()))
        //        .uri(format!("https://management.azure.com/subscriptions/{}/resourcegroups?api-version=2017-05-10", subscription_id))
        .uri(format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
            subscription_id
        ))
        .body(Body::from(""))
        .unwrap();

    let fut = perform_http_request(&client, request, http::status::StatusCode::OK).and_then(|resp| {
        println!("\n\nresp {:?}", resp);
        ok(())
    });
    core.run(fut).unwrap();
}
