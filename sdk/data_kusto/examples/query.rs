use azure_data_kusto::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let service_url = std::env::args()
        .nth(1)
        .expect("please specify service url name as first command line parameter");

    let database = std::env::args()
        .nth(2)
        .expect("please specify database name as second command line parameter");

    let query = std::env::args()
        .nth(3)
        .expect("please specify query as third command line parameter");

    let client_id =
        std::env::var("AZURE_CLIENT_ID").expect("Set env variable AZURE_CLIENT_ID first!");
    let client_secret =
        std::env::var("AZURE_CLIENT_SECRET").expect("Set env variable AZURE_CLIENT_SECRET first!");
    let authority_id =
        std::env::var("AZURE_TENANT_ID").expect("Set env variable AZURE_TENANT_ID first!");

    let kcsb = ConnectionStringBuilder::new_with_aad_application_key_authentication(
        &service_url,
        &authority_id,
        &client_id,
        &client_secret,
    );

    let client = KustoClient::try_from(kcsb).unwrap();

    let response = client
        .execute_query(database, query)
        .into_future()
        .await
        .unwrap();

    for table in &response.tables {
        match table {
            ResultTable::DataSetHeader(header) => println!("header: {:?}", header),
            ResultTable::DataTable(table) => println!("table: {:?}", table),
            ResultTable::DataSetCompletion(completion) => println!("completion: {:?}", completion),
        }
    }

    let primary_results = response.into_primary_results().collect::<Vec<_>>();
    println!("primary results: {:?}", primary_results);

    Ok(())
}
