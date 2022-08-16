/*
Prints the name of pools using the data plane APIs

cargo run --package azure_svc_batch --example list_pools
*/

use azure_identity::AzureCliCredential;
use futures::stream::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let account_name = std::env::args().nth(1).expect("please specify batch account");
    let region = std::env::args().nth(2).expect("please specify region");

    let endpoint = format!("https://{}.{}.batch.azure.com", account_name, region);
    let scopes = &["https://batch.core.windows.net/"];
    let credential = Arc::new(AzureCliCredential::new());
    let client = azure_svc_batch::Client::builder(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let mut stream = client.job_client().list().into_stream();
    while let Some(jobs) = stream.next().await {
        let jobs = jobs?;
        for job in jobs.value {
            println!("{:?}", job);
        }
    }

    Ok(())
}
