use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::TableService;
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
    let table_service = TableService::new(client);

    let future = table_service.list_tables().and_then(move |tables| {
        println!("Account {} has {} tables(s)", account, tables.len());
        for ref table in tables {
            println!("{}", table);
        }
        Ok(())
    });

    core.run(future)?;
    Ok(())
}