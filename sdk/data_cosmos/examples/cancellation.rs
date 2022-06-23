use azure_data_cosmos::prelude::*;
use stop_token::prelude::*;
use stop_token::StopSource;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    // First we retrieve the account name and access key from environment variables, and
    // create an authorization token.
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let primary_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");
    let authorization_token = AuthorizationToken::primary_from_base64(&primary_key)?;

    // Create a new Cosmos client.
    let options = CosmosOptions::default();
    let client = CosmosClient::new(account.clone(), authorization_token.clone(), options);

    // Create a new database, and time out if it takes more than 1 second.
    let future = client.create_database("my_database").into_future();
    let deadline = Instant::now() + Duration::from_secs(1);
    match future.timeout_at(deadline).await {
        Ok(Ok(r)) => println!("successful response: {:?}", r),
        Ok(Err(e)) => println!("request was made but failed: {:?}", e),
        Err(_) => println!("request timed out!"),
    };

    // Create multiple new databases, and cancel them if they don't complete before
    // they're sent a stop signal.
    let source = StopSource::new();
    for _ in 1..10 {
        let client = client.clone();
        // Clone the stop token for each request.
        let deadline = source.token();
        tokio::spawn(async move {
            let future = client.create_database("my_database").into_future();
            match future.timeout_at(deadline).await {
                Ok(Ok(r)) => println!("successful response: {:?}", r),
                Ok(Err(e)) => println!("request was made but failed: {:?}", e),
                Err(_) => println!("request was cancelled!"),
            };
        });
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
    // This causes all cancel tokens to fire. Any request tied to a stop token created
    // from this source will be canceled.
    println!("cancelling all requests");
    drop(source);
    // Any request that has not yet completed will be canceled at this point

    // Keep the program alive for a bit longer so the tasks get a chance to
    // print before exiting.
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}
