/*
Performs an Application Insights query

Example:

$ cargo run --release --example query -- $APP_INSIGHTS_INSTANCE 'traces | take 2 | project severityLevel, message'
severityLevel:1 message:"Executing 'Functions.agent_commands' (Reason='This function was programmatically called via the host APIs.', Id=4253c319-dc36-4981-850a-d4a2584b65aa)"
severityLevel:1 message:"Executed 'Functions.agent_commands' (Succeeded, Id=4253c319-dc36-4981-850a-d4a2584b65aa, Duration=19ms)"
$

*/

use azure_identity::token_credentials::AzureCliCredential;
use azure_svc_applicationinsights::{models::QueryBody, operations::query};

const ENDPOINT: &str = "https://api.applicationinsights.io";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::args().nth(1).expect("please specify application id");
    let query = std::env::args().nth(2).expect("please specify query");
    let timespan = std::env::args().nth(3);

    let base_path = format!("{}/v1", ENDPOINT);
    let http_client = azure_core::new_http_client();
    let token_credential = Box::new(AzureCliCredential {});
    let config = &azure_svc_applicationinsights::config(http_client, token_credential)
        .base_path(base_path)
        .token_credential_resource(ENDPOINT)
        .build();

    let body = &QueryBody {
        query,
        timespan,
        applications: None,
    };

    let response = query::execute(config, &app_id, body).await?;

    let unnamed = "unnamed".to_string();

    for table in &response.tables {
        for row in table.rows.as_array().unwrap().iter() {
            for (j, value) in row.as_array().unwrap().iter().enumerate() {
                print!("{}:{} ", table.columns[j].name.as_ref().unwrap_or_else(|| &unnamed), value);
            }
            println!();
        }
    }

    Ok(())
}
