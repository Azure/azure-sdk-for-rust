/*
Creates a batch job and task using the data plane APIs

cargo run --package azure_svc_batch --example create_task
*/

use azure_identity::AzureCliCredential;
use azure_svc_batch::models::{JobAddParameter, PoolInformation, TaskAddParameter};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_name = std::env::args().nth(1).expect("please specify batch account");
    let region = std::env::args().nth(2).expect("please specify region");
    let pool_id = std::env::args().nth(3).expect("please specify pool");
    let job_id = std::env::args().nth(4).expect("please specify job_id");
    let task_id = std::env::args().nth(5).expect("please specify task_id");

    let endpoint = format!("https://{}.{}.batch.azure.com", account_name, region);
    let scopes = &["https://batch.core.windows.net/"];
    let credential = Arc::new(AzureCliCredential::new());
    let client = azure_svc_batch::Client::builder(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let pool_info = PoolInformation {
        pool_id: Some(pool_id),
        ..PoolInformation::new()
    };

    println!("creating job");
    let job_params = JobAddParameter::new(job_id.clone(), pool_info);
    client.job_client().add(job_params).into_future().await?;

    println!("creating task");
    let command_line = "echo hello there".to_string();
    let task = TaskAddParameter::new(task_id.to_string(), command_line);
    client.task_client().add(job_id, task).into_future().await?;

    Ok(())
}
