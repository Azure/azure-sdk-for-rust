#![cfg(all(test, feature = "test_e2e"))]
use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

mod setup;

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
async fn user_defined_function00() -> Result<(), azure_data_cosmos::Error> {
    const DATABASE_NAME: &str = "test-cosmos-db-udf";
    const COLLECTION_NAME: &str = "test-udf";
    const USER_DEFINED_FUNCTION_NAME: &str = "test";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client.create_database(DATABASE_NAME).await.unwrap();

    let database = client.database_client(DATABASE_NAME);

    // create a temp collection
    let _create_collection_response = database
        .create_collection(COLLECTION_NAME, "/id")
        .await
        .unwrap();

    let collection = database.collection_client(COLLECTION_NAME);
    let user_defined_function = collection.user_defined_function_client(USER_DEFINED_FUNCTION_NAME);

    let ret = user_defined_function
        .create_user_defined_function("body")
        .await?;

    let stream = collection
        .list_user_defined_functions()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = stream.into_stream();
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        assert_eq!(ret.item_count, 1);
    }

    let ret = user_defined_function
        .replace_user_defined_function(FN_BODY)
        .consistency_level(&ret)
        .await?;

    let query_stmt = format!("SELECT udf.{}(100)", USER_DEFINED_FUNCTION_NAME);
    let ret: QueryDocumentsResponseRaw<serde_json::Value> = collection
        .query_documents(Query::new(query_stmt))
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream()
        .next()
        .await
        .unwrap()?
        .into_raw();

    assert_eq!(ret.item_count, 1);

    let fn_return = ret.results[0].as_object().unwrap();
    let value = fn_return.iter().take(1).next().unwrap().1.as_f64().unwrap();
    assert_eq!(value, 10.0);

    let query_stmt = format!("SELECT udf.{}(10000)", USER_DEFINED_FUNCTION_NAME);
    let ret: QueryDocumentsResponseRaw<serde_json::Value> = collection
        .query_documents(Query::new(query_stmt))
        .consistency_level(&ret)
        .max_item_count(2i32)
        .into_stream()
        .next()
        .await
        .unwrap()?
        .into_raw();

    assert_eq!(ret.item_count, 1);

    let fn_return = ret.results[0].as_object().unwrap();
    let value = fn_return
        .into_iter()
        .take(1)
        .next()
        .unwrap()
        .1
        .as_f64()
        .unwrap();
    assert_eq!(value, 4000.0);

    let _ret = user_defined_function
        .delete_user_defined_function()
        .consistency_level(&ret)
        .await?;

    // delete the database
    database.delete_database().await?;

    Ok(())
}
