/*
Creates a batch job and task using the data plane APIs

cargo run --package azure_svc_batch --example create_task
*/

use azure_identity::token_credentials::AzureCliCredential;
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
    let credential = Arc::new(AzureCliCredential {});
    let client = azure_svc_batch::ClientBuilder::new(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let pool_id = Some(pool_id);
    let pool_info = PoolInformation {
        pool_id,
        auto_pool_specification: None,
    };

    let job_params = JobAddParameter {
        id: job_id.to_string(),
        display_name: None,
        priority: None,
        max_parallel_tasks: None,
        constraints: None,
        job_manager_task: None,
        job_preparation_task: None,
        job_release_task: None,
        common_environment_settings: vec![],
        pool_info,
        on_all_tasks_complete: None,
        on_task_failure: None,
        metadata: vec![],
        uses_task_dependencies: None,
        network_configuration: None,
    };

    println!("creating job");
    client.job().add(job_params).into_future().await?;

    let constraints = None;
    let command_line = "echo hello there".to_string();
    let task = TaskAddParameter {
        affinity_info: None,
        application_package_references: vec![],
        authentication_token_settings: None,
        container_settings: None,
        constraints,
        command_line,
        display_name: None,
        environment_settings: vec![],
        depends_on: None,
        exit_conditions: None,
        id: task_id.to_string(),
        multi_instance_settings: None,
        required_slots: None,
        resource_files: vec![],
        output_files: vec![],
        user_identity: None,
    };

    println!("creating task");
    client.task().add(job_id, task).into_future().await?;

    Ok(())
}
