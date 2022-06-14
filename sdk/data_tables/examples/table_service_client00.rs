use azure_data_tables::prelude::{table_service_client::TableServiceClient, AuthorizationToken};
use futures::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let table_service = create_service_table_client()?;
    //let table_service = TableServiceClient::emulator();

    for i in 1..=20 {
        let response = table_service
            .create_table(format!("table{:02}", i))
            .into_future()
            .await?;
        println!("{} created", response.table_name);
    }

    let mut stream = table_service.query_tables().max_per_page(5).into_stream();
    while let Some(Ok(tables)) = stream.next().await {
        for table in tables {
            let _ = table_service
                .delete_table(&table.name)
                .into_future()
                .await?;
            println!("{} deleted", table.name);
        }
    }

    Ok(())
}

fn create_service_table_client() -> Result<TableServiceClient, Box<dyn Error>> {
    let account = std::env::vars()
        .find(|i| i.0 == "STORAGE_ACCOUNT")
        .map(|i| i.1)
        .ok_or("STORAGE_ACCOUNT environment variable not found")?;

    let key = std::env::vars()
        .find(|i| i.0 == "STORAGE_MASTER_KEY")
        .map(|i| i.1)
        .ok_or("STORAGE_MASTER_KEY environment variable not found")?;

    let table_client = TableServiceClient::new(
        account.clone(),
        AuthorizationToken::SharedKeyToken { account, key },
    );
    Ok(table_client)
}
