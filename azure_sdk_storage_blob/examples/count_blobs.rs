use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::Stream;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<dyn Error>> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    
    let mut core = Core::new()?;
    let client = Client::new(&account, &master_key)?;

    let mut count = 0;
    let list_blobs = client.stream_list_blobs().with_container_name(&container);
    let future = list_blobs.finalize().for_each(|_blob| {
        count += 1;
        Ok(())
    });
    core.run(future)?;

    println!("blob count {}", count);

    Ok(())
}
