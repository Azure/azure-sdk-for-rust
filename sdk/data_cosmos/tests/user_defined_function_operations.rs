use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;
use tracing::info;

mod setup_mock;

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

#[tokio::test]
async fn user_defined_function_operations() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();
    const DATABASE_NAME: &str = "test-cosmos-db-udf";
    const COLLECTION_NAME: &str = "test-udf";
    const USER_DEFINED_FUNCTION_NAME: &str = "test";

    let client = setup_mock::initialize("user_defined_function_operations")?;

    info!("creating database");
    // create a temp database
    let _ = client.create_database(DATABASE_NAME).await?;
    info!("created database");

    let database = client.database_client(DATABASE_NAME);

    // create a temp collection
    info!("creating collection");
    let _ = database.create_collection(COLLECTION_NAME, "/id").await?;
    info!("created collection");

    let collection = database.collection_client(COLLECTION_NAME);
    let user_defined_function = collection.user_defined_function_client(USER_DEFINED_FUNCTION_NAME);

    info!("creating user defined function");
    let ret = user_defined_function
        .create_user_defined_function("body")
        .await?;
    info!("created user defined function");

    info!("listing user defined functions");
    let stream = collection
        .list_user_defined_functions()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = stream.into_stream();
    while let Some(ret) = stream.next().await {
        assert_eq!(ret?.item_count, 1);
    }
    info!("listed user defined functions");

    info!("replacing user defined functions");
    let ret = user_defined_function
        .replace_user_defined_function(FN_BODY)
        .consistency_level(&ret)
        .await?;
    info!("replaced user defined functions");

    info!("querying documents");
    let query_stmt = format!("SELECT udf.{USER_DEFINED_FUNCTION_NAME}(100)");
    let ret = collection
        .query_documents(Query::new(query_stmt))
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;

    assert_eq!(ret.item_count, 1);

    let fn_return = ret.documents().next().unwrap().as_object().unwrap();
    let value = fn_return.iter().take(1).next().unwrap().1.as_f64().unwrap();
    assert_eq!(value, 10.0);
    info!("queried documents");

    info!("querying documents again");
    let query_stmt = format!("SELECT udf.{USER_DEFINED_FUNCTION_NAME}(10000)");
    let ret = collection
        .query_documents(Query::new(query_stmt))
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream::<serde_json::Value>()
        .next()
        .await
        .unwrap()?;

    assert_eq!(ret.item_count, 1);

    let fn_return = ret.documents().next().unwrap().as_object().unwrap();
    let value = fn_return
        .into_iter()
        .take(1)
        .next()
        .unwrap()
        .1
        .as_f64()
        .unwrap();
    assert_eq!(value, 4000.0);
    info!("queried documents again");

    info!("deleting test resources");
    let _ret = user_defined_function
        .delete_user_defined_function()
        .consistency_level(&ret)
        .await?;

    // delete the database
    database.delete_database().await?;
    info!("deleted test resources");

    Ok(())
}
