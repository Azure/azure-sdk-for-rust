use azure_sdk_storage_account::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let future = {
        client.get_account_information().finalize().map(|response| {
            println!("{:?}", response);
        })
    };

    core.run(future)?;

    Ok(())
}
