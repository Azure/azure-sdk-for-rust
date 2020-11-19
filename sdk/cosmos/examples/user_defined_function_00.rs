use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

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
async fn main() -> Result<(), Box<dyn Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let database_client = client.into_database_client(database);
    let collection_client = database_client.into_collection_client(collection);
    let user_defined_function_client = collection_client
        .clone()
        .into_user_defined_function_client("test15");

    let ret = user_defined_function_client
        .create_user_defined_function()
        .with_body("body")
        .execute()
        .await?;
    println!("Creeate response object:\n{:#?}", ret);

    let stream = collection_client
        .list_user_defined_functions()
        .with_max_item_count(3)
        .with_consistency_level((&ret).into());
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        println!(
            "List loop received {} items. Object:\n{:#?}",
            ret.item_count, ret
        );
    }

    let ret = user_defined_function_client
        .replace_user_defined_function()
        .with_consistency_level((&ret).into())
        .with_body(FN_BODY)
        .execute()
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let ret = collection_client
        .query_documents()
        .with_query(&"SELECT udf.test15(100)".into())
        .with_consistency_level((&ret).into())
        .with_max_item_count(2)
        .execute::<serde_json::Value>()
        .await?
        .into_raw();
    println!("Query response object:\n{:#?}", ret);

    // this code extracts the first object
    let fn_return = &ret.results[0].as_object().unwrap();
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

    let ret = user_defined_function_client
        .delete_user_defined_function()
        .with_consistency_level((&ret).into())
        .execute()
        .await?;

    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
