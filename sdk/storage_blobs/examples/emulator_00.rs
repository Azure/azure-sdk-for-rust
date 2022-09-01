use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

    // this is how you use the emulator.
    let storage_account = BlobServiceClientBuilder::with_location(CloudLocation::Emulator {
        address: "".into(),
        port: 1000, // TODO: provide constructors for this
    })
    .build();
    let container_client = storage_account.container_client("emulcont");

    // create container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await?;

    let res = container_client
        .list_blobs()
        .include_metadata(true)
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("{:?}", res);

    Ok(())
}
