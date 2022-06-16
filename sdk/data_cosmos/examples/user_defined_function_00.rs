use azure_core::error::Result;
use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

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
async fn main() -> Result<()> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database = client.database_client(database);
    let collection = database.collection_client(collection);
    let user_defined_function = collection.user_defined_function_client("test15");

    let ret = user_defined_function
        .create_user_defined_function("body")
        .into_future()
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
        .into_future()
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let ret = collection
        .query_documents("SELECT udf.test15(100)")
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?
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

    let ret = user_defined_function
        .delete_user_defined_function()
        .consistency_level(&ret)
        .into_future()
        .await?;

    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
