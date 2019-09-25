use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::future::*;
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
    let mut next_marker: Option<String> = None;

    loop {
        let mut list_blobs = client.list_blobs().with_container_name(&container);
        if let Some(ref marker) = next_marker {
            list_blobs = list_blobs.with_next_marker(marker);
        }
        let future = list_blobs.finalize().map(|iv| {
            count += iv.incomplete_vector.len();
            next_marker = iv.incomplete_vector.token;
        });
        core.run(future)?;
        if next_marker.is_none(){
            break;
        }
    }

    println!("blob count {}", count);

    Ok(())
}
