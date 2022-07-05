use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The cosmos query you're trying to execute
    query: String,
}

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
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args
        .auth
        .into_client()?
        .database_client(args.database_name)
        .collection_client(args.collection_name);

    let query_obj = Query::new(args.query);

    let respo: QueryDocumentsResponse<serde_json::Value> = client
        .query_documents(query_obj.clone())
        .query_cross_partition(true)
        .max_item_count(3i32)
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("as json == {:?}", respo);

    let respo: QueryDocumentsResponse<MySecondSampleStructOwned> = client
        .query_documents(query_obj)
        .query_cross_partition(true)
        .parallelize_cross_partition_query(true)
        .max_item_count(2)
        .into_stream()
        .next()
        .await
        .unwrap()?;
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
