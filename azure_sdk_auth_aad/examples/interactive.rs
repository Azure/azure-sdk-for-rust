use azure_sdk_auth_aad::*;
use futures::executor::block_on;
use oauth2::{ClientId, ClientSecret, TokenResponse};
use std::env;
use url::Url;

fn main() {
    block_on(main_async());
}

async fn main_async() {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
    );
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let subscription_id =
        env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");

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
    // with the token. This naive server is blocking
    // so you should use something better.
    let code = naive_server(&c, 3003).unwrap();

    println!("code received: {:?}", code);

    // Exchange the token with one that can be
    // used for authorization
    let token = exchange(c, code).await.unwrap();

    println!("token received: {:?}", token);

    // Let's enumerate the Azure SQL Databases instances
    // in the subscription. Note: this way of calling the REST API
    // will be different (and easier) using other Azure Rust SDK
    // crates, this is just an example.
    let url = Url::parse(&format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
            subscription_id
        )).unwrap();

    let resp = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("\n\nresp {:?}", resp);

    //let request = Request::builder()
    //    .method("GET")
    //    .header("Authorization", format!("Bearer {}", token.access_token().secret()))
    //    //        .uri(format!("https://management.azure.com/subscriptions/{}/resourcegroups?api-version=2017-05-10", subscription_id))
    //    .uri(format!(
    //        "https://management.azure.com/subscriptions/{}/providers/Microsoft.Sql/servers?api-version=2015-05-01-preview",
    //        subscription_id
    //    ))
    //    .body(Body::from(""))
    //    .unwrap();

    //let fut =
    //    perform_http_request(&client, request, http::status::StatusCode::OK).and_then(|resp| {
    //        println!("\n\nresp {:?}", resp);
    //        ok(())
    //    });
    //core.run(fut).unwrap();
}
