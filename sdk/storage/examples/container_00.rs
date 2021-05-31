use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = new_http_client();
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let container = storage_account.as_container_client(container_name);

    let max_results = NonZeroU32::new(3).unwrap();
    let iv = storage_account
        .list_containers()
        .max_results(max_results)
        .execute()
        .await?;
    println!(
        "List containers returned {} containers.",
        iv.incomplete_vector.len()
    );
    for cont in iv.incomplete_vector.iter() {
        println!("\t{}", cont.name);
    }

    let iv = container
        .list_blobs()
        .max_results(max_results)
        .execute()
        .await?;

    println!("List blob returned {} blobs.", iv.blobs.blobs.len());
    for cont in iv.blobs.blobs.iter() {
        println!(
            "\t{}\t{} MB",
            cont.name,
            cont.properties.content_length / (1024 * 1024)
        );
    }

    Ok(())
}
