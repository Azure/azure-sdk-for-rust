/*
Creates a batch job and task using the data plane APIs

This example shows how to do error handling when you wish
to match any error from azure_svc_batch crate.

cargo run --package azure_svc_batch --example create_task_thiserror
*/

use azure_identity::AzureCliCredential;
use azure_svc_batch::models::{JobAddParameter, PoolInformation, TaskAddParameter};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("please specify batch account")]
    AccountNameRequired,
    #[error("please specify region")]
    RegionRequired,
    #[error("please specify pool id")]
    PoolIdRequired,
    #[error("please specify job id")]
    JobIdRequired,
    #[error("please specify task id")]
    TaskIdRequired,
    #[error("batch error")]
    Batch(#[source] azure_svc_batch::Error),
}

/// Any azure_svc_batch error ths is not mapped to a specific error
/// is mapped to Error::Batch by default with a `?`, which performs an `into()`.
impl<T: Into<azure_svc_batch::Error>> From<T> for Error {
    fn from(error: T) -> Self {
        Self::Batch(error.into())
    }
}

async fn run() -> Result<(), Error> {
    let account_name = std::env::args().nth(1).ok_or(Error::AccountNameRequired)?;
    let region = std::env::args().nth(2).ok_or(Error::RegionRequired)?;
    let pool_id = std::env::args().nth(3).ok_or(Error::PoolIdRequired)?;
    let job_id = std::env::args().nth(4).ok_or(Error::JobIdRequired)?;
    let task_id = std::env::args().nth(5).ok_or(Error::TaskIdRequired)?;

    let endpoint = format!("https://{}.{}.batch.azure.com", account_name, region);
    let scopes = &["https://batch.core.windows.net/"];
    let credential = Arc::new(AzureCliCredential {});
    let client = azure_svc_batch::ClientBuilder::new(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let pool_info = PoolInformation {
        pool_id: Some(pool_id),
        ..PoolInformation::new()
    };

    println!("creating job");
    let job_params = JobAddParameter::new(job_id.clone(), pool_info);
    client.job().add(job_params).into_future().await?;

    println!("creating task");
    let command_line = "echo hello there".to_string();
    let task = TaskAddParameter::new(task_id.to_string(), command_line);
    client.task().add(job_id, task).into_future().await?;

    Ok(())
}

/*
The eyre crate is recommended for printing the report with the full error chain.
You can use it instead of the below print_error_chain.

#[tokio::main]
async fn main() -> eyre::Result<()> {
    Ok(run().await?)
}
*/

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(error) => {
            print_error_chain(error);
            std::process::exit(1);
        }
    }
}

fn print_error_chain(error: impl std::error::Error) {
    println!("- {}", error.to_string());
    if let Some(source) = error.source() {
        print_error_chain(source);
    }
}
