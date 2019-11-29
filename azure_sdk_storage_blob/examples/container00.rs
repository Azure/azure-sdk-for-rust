use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = Client::new(&account, &master_key)?;

    let iv = client.list_containers().finalize().await?;
    println!(
        "List containers returned {} containers.",
        iv.incomplete_vector.len()
    );
    for cont in iv.incomplete_vector.iter() {
        println!("\t{}", cont.name);
    }

    let iv = client
        .list_blobs()
        .with_container_name(&container)
        .finalize()
        .await?;

    println!("List blob returned {} blobs.", iv.incomplete_vector.len());
    for cont in iv.incomplete_vector.iter() {
        println!(
            "\t{}\t{} MB",
            cont.name,
            cont.content_length / (1024 * 1024)
        );
    }

    Ok(())
}
