/*
Hard deletes AAD app registrations that are deleted

cargo run --example purge_deleted_apps -- "startswith(displayName,'fretang')"
*/

use azure_identity::AzureCliCredential;
use azure_svc_graphrbac::ClientBuilder;
use futures::stream::StreamExt;
use std::{env::args, sync::Arc};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let filter = args().nth(1).expect("missing filter");
    let credential = Arc::new(AzureCliCredential::new());
    let tenant_id = AzureCliCredential::get_tenant()?;

    let client = ClientBuilder::new(credential).build().deleted_applications_client();

    let mut stream = client.list(&tenant_id).filter(filter).into_stream();
    while let Some(apps) = stream.next().await {
        let apps = apps?;
        for app in apps.value {
            println!("{:?}", app.display_name);
            let obj_id = app.directory_object.object_id.expect("missing object id");
            client.hard_delete(obj_id, &tenant_id).into_future().await?;
        }
    }

    Ok(())
}
