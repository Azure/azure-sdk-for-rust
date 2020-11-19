use azure_cosmos::prelude::*;
use azure_cosmos::responses::QueryDocumentsResponse;
use std::error::Error;
#[macro_use]
extern crate serde_derive;
use azure_cosmos::Query;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStructOwned {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct MySecondSampleStructOwned {
    id: String,
    color: String,
    #[serde(rename = "myvalue")]
    my_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");
    let query = std::env::args()
        .nth(3)
        .expect("please specify requested query");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let query_obj = Query::new(&query);

    let respo: QueryDocumentsResponse<serde_json::Value> = client
        .query_documents()
        .with_query(&query_obj)
        .with_query_cross_partition(true)
        .with_max_item_count(3)
        .execute()
        .await?;
    println!("as json == {:?}", respo);

    let respo: QueryDocumentsResponse<MySecondSampleStructOwned> = client
        .query_documents()
        .with_query(&query_obj)
        .with_query_cross_partition(true)
        .with_parallelize_cross_partition_query(true)
        .with_max_item_count(2)
        .execute()
        .await?;
    println!("as items == {:?}", respo);

    //let ret = client
    //    .query_documents(
    //        &database_name,
    //        &collection_name,
    //        Query::from(query.as_ref()),
    //    )
    //    .execute_json()
    //    .await?;

    //println!("As JSON:\n{:?}", ret);

    //for doc in ret.results {
    //    println!("{}", doc.result);
    //}

    //let ret = client
    //    .query_documents(
    //        &database_name,
    //        &collection_name,
    //        Query::from(query.as_ref()),
    //    )
    //    .execute::<MySampleStructOwned>()
    //    .await?;

    //println!("\nAs entities:\n{:?}", ret);

    //for doc in ret.results {
    //    println!("{:?}", doc);
    //}

    //// test continuation token
    //// only if we have more than 2 records
    //let ret = client
    //    .query_documents(
    //        &database_name,
    //        &collection_name,
    //        Query::from(query.as_ref()),
    //    )
    //    .max_item_count(2u64)
    //    .execute::<MySampleStructOwned>()
    //    .await?;

    //println!(
    //    "Received {} entries. Continuation token is == {:?}",
    //    ret.results.len(),
    //    ret.additional_headers.continuation_token
    //);

    //if let Some(ct) = ret.additional_headers.continuation_token {
    //    let ret = {
    //        // if we have more, let's get them
    //        client
    //            .query_documents(
    //                &database_name,
    //                &collection_name,
    //                Query::from(query.as_ref()),
    //            )
    //            .continuation_token(ct)
    //            .execute::<MySampleStructOwned>()
    //            .await?
    //    };
    //    println!(
    //        "Received {} entries. Continuation token is == {:?}",
    //        ret.results.len(),
    //        ret.additional_headers.continuation_token
    //    );
    //}

    Ok(())
}
