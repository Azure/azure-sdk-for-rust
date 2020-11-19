#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

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
async fn user_defined_function00() -> Result<(), Box<dyn Error>> {
    const DATABASE_NAME: &str = "test-cosmos-db-udf";
    const COLLECTION_NAME: &str = "test-udf";
    const USER_DEFINED_FUNCTION_NAME: &str = "test";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .with_database_name(&DATABASE_NAME)
        .execute()
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a temp collection
    let _create_collection_response = {
        let indexes = IncludedPathIndex {
            kind: KeyKind::Hash,
            data_type: DataType::String,
            precision: Some(3),
        };

        let ip = IncludedPath {
            path: "/*".to_owned(),
            indexes: Some(vec![indexes]),
        };

        let ip = IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![ip],
            excluded_paths: vec![],
        };

        database_client
            .create_collection()
            .with_collection_name(&COLLECTION_NAME)
            .with_partition_key(&("/id".into()))
            .with_offer(Offer::Throughput(400))
            .with_indexing_policy(&ip)
            .execute()
            .await
            .unwrap()
    };

    let collection_client = database_client.into_collection_client(COLLECTION_NAME);
    let user_defined_function_client =
        collection_client.with_user_defined_function_client(USER_DEFINED_FUNCTION_NAME);

    let ret = user_defined_function_client
        .create_user_defined_function()
        .with_body("body")
        .execute()
        .await?;

    let stream = collection_client
        .list_user_defined_functions()
        .with_max_item_count(3)
        .with_consistency_level((&ret).into());
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        assert_eq!(ret.item_count, 1);
    }

    let ret = user_defined_function_client
        .replace_user_defined_function()
        .with_consistency_level((&ret).into())
        .with_body(FN_BODY)
        .execute()
        .await?;

    let query_stmt = format!("SELECT udf.{}(100)", USER_DEFINED_FUNCTION_NAME);
    let ret: QueryDocumentsResponseRaw<serde_json::Value> = collection_client
        .query_documents()
        .with_query(&(&query_stmt as &str).into())
        .with_consistency_level((&ret).into())
        .with_max_item_count(2)
        .execute()
        .await?
        .into_raw();

    assert_eq!(ret.item_count, 1);

    let fn_return = ret.results[0].as_object().unwrap();
    let value = fn_return.iter().take(1).next().unwrap().1.as_f64().unwrap();
    assert_eq!(value, 10.0);

    let query_stmt = format!("SELECT udf.{}(10000)", USER_DEFINED_FUNCTION_NAME);
    let ret: QueryDocumentsResponseRaw<serde_json::Value> = collection_client
        .query_documents()
        .with_query(&(&query_stmt as &str).into())
        .with_consistency_level((&ret).into())
        .with_max_item_count(2)
        .execute()
        .await?
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

    let _ret = user_defined_function_client
        .delete_user_defined_function()
        .with_consistency_level((&ret).into())
        .execute()
        .await?;

    // delete the database
    database_client.delete_database().execute().await?;

    Ok(())
}
