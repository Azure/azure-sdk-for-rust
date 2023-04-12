use std::sync::Arc;
use azure_identity::DefaultAzureCredential;
use azure_mgmt_resourcegraph::models::{QueryRequest, QueryRequestOptions};
use serde_json::{json, Value};
use azure_mgmt_resourcegraph::Client as AzureResourceGraphClient;
use azure_mgmt_resourcegraph::ClientBuilder as AzureResourceGraphClientBuilder;

/// Fetching Virtual Machines using Azure Resource Graph

#[derive(Debug)]
pub enum GenericError {
    InvalidRequest,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

async fn fetch_azure_records() -> Result<Value, CustomError> {

    let azure_creds = Arc::new(DefaultAzureCredential::default());

    let az_rg_client = AzureResourceGraphClient::builder(azure_creds).build();

    // fetch first 10 records
    let query_options = QueryRequestOptions {
        top: Option::from(10),
        skip: Option::from(0),
        ..Default::default()
    };

    // in the below request, we are fetching resources of type (virtual machines)
    let custom_query_request = QueryRequest {
        subscriptions: vec![],
        management_groups: vec![],
        query: "Resources | where type == 'microsoft.compute/virtualmachines'".to_string(),
        options: Option::from(query_options),
        facets: vec![],
    };

    let query_request = QueryRequest::from(custom_query_request);

    let resources = az_rg_client.resources(query_request).await;

    let result = match resources {
        Ok(d) => {
            let elements = d.data.as_array().unwrap();
            let obj = json!(elements);
            obj
        }
        Err(e) => {
            let actual_error_message = format!("could not fetch data from azure resource graph : {:?}", e.to_string());
            let custom_err = CustomError {
                err_msg: actual_error_message,
                err_type: GenericError::InvalidRequest
            };
            return Err(custom_err)
        }
    };
    Ok(result)
}

/// Make sure you export the following environment variables before running the program
/// export AZURE_CLIENT_ID="00000000-0000-0000-0000-000000000000"
/// export AZURE_CLIENT_SECRET="<SOME_SECRET>"
/// export AZURE_TENANT_ID="00000000-0000-0000-0000-000000000000

/// Building the release binary & executing the program
///     cargo build --release
/// You can also pass the environment variables this way & run execute the program
///     AZURE_CLIENT_ID="<CID>" AZURE_CLIENT_SECRET="<SECRET>" AZURE_TENANT_ID="<TID>" target/release/<BINARY>
#[tokio::main]
async fn main() {
    let _records = match fetch_azure_records().await {
        Ok(d) => {
            // actual azure records
            println!("{}", serde_json::to_string_pretty(&d).unwrap());
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    };
}