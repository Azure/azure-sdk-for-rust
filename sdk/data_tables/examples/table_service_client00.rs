use azure_data_tables::prelude::{table_service_client::TableServiceClient, AuthorizationToken};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let table_service = create_service_table_client()?;

    let _ = table_service.create_table("table").into_future().await?;

    let response = table_service
        .query_tables()
        .filter("TableName eq 'table'")
        .into_future()
        .await?;
    println!("query response: {:#?}", response);

    let _ = table_service.delete_table("table").into_future().await?;

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
