use azure_data_cosmos::resources::collection::*;
use futures::StreamExt;
use tracing::{debug, info};

mod setup_mock;

#[tokio::test]
async fn collection_operations() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    let client = setup_mock::initialize("collection_operations")?;
    let database_name = "test-collection-operations";

    info!("Creating a database with name '{}'...", database_name);
    client.create_database(database_name).await?;
    info!("Successfully created a database");

    // create collection!
    let database = client.database_client(database_name);

    let collection_name = "sample_collection";
    info!("Creating a collection with name '{}'...", collection_name);

    let create_collection_response = database.create_collection(collection_name, "/id").await?;

    assert_eq!(create_collection_response.collection.id, collection_name);

    info!("Successfully created a collection");
    debug!(
        "The create_collection response: {:#?}",
        create_collection_response
    );

    let collection = database.collection_client(collection_name);

    // get collection!
    let get_collection = collection.get_collection().await?;

    assert_eq!(get_collection.collection.id, collection_name);

    info!("Successfully got a collection");
    debug!("The get_collection response: {:#?}", get_collection);

    let collections = database
        .list_collections()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(collections.collections.len(), 1);
    assert_eq!(
        get_collection.collection.indexing_policy,
        collections.collections[0].indexing_policy
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
    let replace_collection_response = collection
        .replace_collection("/id")
        .indexing_policy(new_indexing_policy)
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

    info!("Successfully replaced collection");
    debug!(
        "The replace_collection response: {:#?}",
        replace_collection_response
    );

    // delete collection!
    let delete_collection_response = collection.delete_collection().await?;

    info!("Successfully deleted collection");
    debug!(
        "The delete_collection response: {:#?}",
        delete_collection_response
    );

    // delete database
    database.delete_database().await?;
    info!("Successfully deleted database");

    Ok(())
}
