/// Fetching Virtual Machines using Azure Resource Graph
///
/// The following example is similar to using the [Azure CLI's Resource Graph extension](https://learn.microsoft.com/en-us/azure/governance/resource-graph/first-query-azurecli#add-the-resource-graph-extension).
///
/// `az graph query -q "Resources | where type == 'microsoft.compute/virtualmachines'" | jq .data[].id`
///
/// Ref: <https://learn.microsoft.com/en-us/rest/api/azureresourcegraph/resourcegraph(2021-03-01)/resources/resources?tabs=HTTP>
///
use azure_identity::DefaultAzureCredential;
use azure_mgmt_resourcegraph::{
    models::{QueryRequest, QueryRequestOptions},
    Client,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let azure_creds = Arc::new(DefaultAzureCredential::default());
    let client = Client::builder(azure_creds).build()?;

    // query 10 records at a time
    let options = Some(QueryRequestOptions {
        top: Option::from(10),
        ..Default::default()
    });

    let mut query_request = QueryRequest {
        subscriptions: vec![],
        management_groups: vec![],
        query: "Resources | where type == 'microsoft.compute/virtualmachines'".to_string(),
        options,
        facets: vec![],
    };

    loop {
        let response = client.resources(query_request.clone()).await?;
        if let Some(as_array) = response.data.as_array() {
            for entry in as_array {
                if let Some(resource_id) = entry.get("id").and_then(|x| x.as_str()) {
                    println!("{resource_id:?}");
                }
            }
        }

        // The documentation describes skip token as "Continuation token for pagination, capturing
        // the next page size and offset, as well as the context of the query."
        //
        // As such, if the response contains a skip_token, we use that for subsequent queries.
        // Otherwise, we're done.
        if response.skip_token.is_none() {
            break;
        }
        query_request.options = Some(QueryRequestOptions {
            skip_token: response.skip_token,
            ..Default::default()
        });
    }
    Ok(())
}
