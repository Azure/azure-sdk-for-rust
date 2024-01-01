/*
List blob containers in a storage account
cargo run --package azure_svc_blobstorage --example list_containers $STORAGE_ACCOUNT_NAME

This is similar to `az storage container list --account-name $STORAGE_ACCOUNT_NAME`
https://docs.microsoft.com/cli/azure/storage/container?view=azure-cli-latest#az-storage-container-list
*/

use azure_core::Url;
use azure_svc_blobstorage::Client;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

    let account_name = std::env::args().nth(1).expect("please specify storage account");

    let endpoint = Url::parse(&format!("https://{account_name}.blob.core.windows.net"))?;
    let scopes = &["https://storage.azure.com/.default"];
    let credential = azure_identity::new_credential();
    let client = Client::builder(credential).endpoint(endpoint).scopes(scopes).build()?;

    let mut pages = client.service_client().list_containers_segment().maxresults(1).into_stream();
    while let Some(page) = pages.next().await {
        let page = page?;
        if let Some(containers) = page.containers {
            for container in containers.items {
                println!("{}", container.name);
            }
        }
    }

    Ok(())
}
