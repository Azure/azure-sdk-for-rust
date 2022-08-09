use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

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
    env_logger::init();
    const DATABASE_NAME: &str = "test-cosmos-db-udf";
    const COLLECTION_NAME: &str = "test-udf";
    const USER_DEFINED_FUNCTION_NAME: &str = "test";

    let client = setup_mock::initialize("user_defined_function_operations")?;

    log::info!("creating database");
    // create a temp database
    let _ = client.create_database(DATABASE_NAME).into_future().await?;
    log::info!("created database");

    let database = client.database_client(DATABASE_NAME);

    // create a temp collection
    log::info!("creating collection");
    let _ = database
        .create_collection(COLLECTION_NAME, "/id")
        .into_future()
        .await?;
    log::info!("created collection");

    let collection = database.collection_client(COLLECTION_NAME);
    let user_defined_function = collection.user_defined_function_client(USER_DEFINED_FUNCTION_NAME);

    log::info!("creating user defined function");
    let ret = user_defined_function
        .create_user_defined_function("body")
        .into_future()
        .await?;
    log::info!("created user defined function");

    log::info!("listing user defined functions");
    let stream = collection
        .list_user_defined_functions()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = stream.into_stream();
    while let Some(ret) = stream.next().await {
        assert_eq!(ret?.item_count, 1);
    }
    log::info!("listed user defined functions");

    log::info!("replacing user defined functions");
    let ret = user_defined_function
        .replace_user_defined_function(FN_BODY)
        .consistency_level(&ret)
        .into_future()
        .await?;
    log::info!("replaced user defined functions");

    log::info!("querying documents");
    let query_stmt = format!("SELECT udf.{}(100)", USER_DEFINED_FUNCTION_NAME);
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
    log::info!("queried documents");

    log::info!("querying documents again");
    let query_stmt = format!("SELECT udf.{}(10000)", USER_DEFINED_FUNCTION_NAME);
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
    log::info!("queried documents again");

    log::info!("deleting test resources");
    let _ret = user_defined_function
        .delete_user_defined_function()
        .consistency_level(&ret)
        .into_future()
        .await?;

    // delete the database
    database.delete_database().into_future().await?;
    log::info!("deleted test resources");

    Ok(())
}
