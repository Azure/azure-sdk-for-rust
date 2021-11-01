#![cfg(feature = "mock_transport_framework")]

use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::resources::collection::*;
use std::error::Error;

mod setup;

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::test]
async fn collection_operations() -> Result<(), BoxedError> {
    env_logger::init();

    let client = setup::initialize("collection_operations")?;
    let database_name = "test-collection-operations";
    let context = Context::new();

    client
        .create_database(context.clone(), database_name, CreateDatabaseOptions::new())
        .await?;

    // create collection!
    let db_client = client.clone().into_database_client(database_name.clone());

    let collection_name = "sample_collection";
    log::info!("Creating a collection with name '{}'...", collection_name);

    let create_collection_response = db_client
        .create_collection(
            context.clone(),
            collection_name,
            CreateCollectionOptions::new("/id"),
        )
        .await?;

    assert_eq!(create_collection_response.collection.id, collection_name);

    log::info!("Successfully created a collection");
    log::debug!(
        "The create_collection response: {:#?}",
        create_collection_response
    );

    let collection_client = db_client.clone().into_collection_client(collection_name);

    // get collection!
    let get_collection_response = collection_client
        .get_collection(context.clone(), GetCollectionOptions::new())
        .await?;

    assert_eq!(get_collection_response.collection.id, collection_name);

    log::info!("Successfully got a collection");
    log::debug!(
        "The get_collection response: {:#?}",
        get_collection_response
    );

    let indexes = IncludedPathIndex {
        kind: KeyKind::Hash,
        data_type: DataType::String,
        precision: Some(3),
    };

    let include_path = IncludedPath {
        path: "/*".to_owned(),
        indexes: Some(vec![indexes]),
    };

    let mut new_indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![include_path],
        excluded_paths: vec![],
    };

    new_indexing_policy
        .excluded_paths
        .push("/\"excludeme\"/?".to_owned().into());

    // replace collection!
    let replace_collection_response = collection_client
        .replace_collection(
            context.clone(),
            ReplaceCollectionOptions::new("/id").indexing_policy(new_indexing_policy),
        )
        .await?;

    assert_eq!(replace_collection_response.collection.id, collection_name);

    assert_eq!(
        replace_collection_response
            .collection
            .indexing_policy
            .included_paths[0]
            .path,
        "/*"
    );
    assert_eq!(
        replace_collection_response
            .collection
            .indexing_policy
            .excluded_paths[0]
            .path,
        "/\"excludeme\"/?"
    );

    log::info!("Successfully replaced collection");
    log::debug!(
        "The replace_collection response: {:#?}",
        replace_collection_response
    );

    // delete collection!
    let delete_collection_response = collection_client
        .delete_collection(context.clone(), DeleteCollectionOptions::new())
        .await?;

    log::info!("Successfully deleted collection");
    log::debug!(
        "The delete_collection response: {:#?}",
        delete_collection_response
    );

    db_client
        .delete_database(Context::new(), DeleteDatabaseOptions::new())
        .await
        .unwrap();

    Ok(())
}
