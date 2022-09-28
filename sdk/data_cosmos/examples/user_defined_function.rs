use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::stream::StreamExt;

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
}

const FN_BODY: &str = r#"
function tax(income) {
    if (income == undefined)
        throw 'no input';
    if (income < 1000)
        return income * 0.1;
    else if (income < 10000)
        return income * 0.2;
    else
        return income * 0.4;
}"#;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token);
    let database = client.database_client(args.database_name);
    let collection = database.collection_client(args.collection_name);
    let user_defined_function = collection.user_defined_function_client("test15");

    let ret = user_defined_function
        .create_user_defined_function("body")
        .await?;
    println!("Creeate response object:\n{:#?}", ret);

    let stream = collection
        .list_user_defined_functions()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = stream.into_stream();
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        println!(
            "List loop received {} items. Object:\n{:#?}",
            ret.item_count, ret
        );
    }

    let ret = user_defined_function
        .replace_user_defined_function(FN_BODY)
        .consistency_level(&ret)
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let ret = collection
        .query_documents("SELECT udf.test15(100)")
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;
    println!("Query response object:\n{:#?}", ret);

    // this code extracts the first object
    let fn_return = ret.documents().next().unwrap().as_object().unwrap();
    println!("fn_return == {:?}", fn_return);
    // and from the first object get the first value as f64
    let value = fn_return
        .into_iter()
        .take(1)
        .next()
        .unwrap()
        .1
        .as_f64()
        .unwrap();
    println!("value == {:?}", value);

    let ret = user_defined_function
        .delete_user_defined_function()
        .consistency_level(&ret)
        .await?;

    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
