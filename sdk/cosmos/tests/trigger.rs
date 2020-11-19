#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::prelude::*;
use azure_cosmos::trigger::*;
use futures::stream::StreamExt;
use std::error::Error;

mod setup;

const TRIGGER_BODY: &str = r#"
function updateMetadata() {
    var context = getContext();
    var collection = context.getCollection();
    var response = context.getResponse();
    var createdDocument = response.getBody();

    // query for metadata document
    var filterQuery = 'SELECT * FROM root r WHERE r.id = \"_metadata\"';
    var accept = collection.queryDocuments(collection.getSelfLink(), filterQuery,
        updateMetadataCallback);
    if (!accept) throw\ "Unable to update metadata, abort\";

    function updateMetadataCallback(err, documents, responseOptions) {
        if (err) throw new Error(\"Error\" + err.message);
            if (documents.length != 1) throw 'Unable to find metadata document';
            var metadataDocument = documents[0];

            // update metadata
            metadataDocument.createdDocuments += 1; metadataDocument.createdNames += \" \" + createdDocument.id;
            var accept = collection.replaceDocument(metadataDocument._self,
                metadataDocument,
                function(err, docReplaced) {
                    if (err) throw\ "Unable to update metadata, abort\";
                });
            if (!accept) throw\ "Unable to update metadata, abort\";
            return;
        }
}"#;

#[tokio::test]
async fn trigger() -> Result<(), Box<dyn Error>> {
    const DATABASE_NAME: &str = "test-cosmos-db-trigger";
    const COLLECTION_NAME: &str = "test-udf";
    const TRIGGER_NAME: &str = "test";

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
    let trigger_client = collection_client.into_trigger_client(TRIGGER_NAME);

    let ret = trigger_client
        .create_trigger()
        .with_trigger_type(TriggerType::Post)
        .with_trigger_operation(TriggerOperation::All)
        .with_body(&"something")
        .execute()
        .await?;

    let ret = trigger_client
        .replace_trigger()
        .with_consistency_level(ret.into())
        .with_trigger_type(TriggerType::Post)
        .with_trigger_operation(TriggerOperation::All)
        .with_body(&TRIGGER_BODY)
        .execute()
        .await?;

    let mut last_session_token: Option<ConsistencyLevel> = None;

    let stream = collection_client
        .list_triggers()
        .with_max_item_count(3)
        .with_consistency_level((&ret).into());
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        last_session_token = Some(ret.session_token.into());
    }

    let _ret = trigger_client
        .delete_trigger()
        .with_consistency_level(last_session_token.unwrap())
        .execute()
        .await?;

    // delete the database
    database_client.delete_database().execute().await?;

    Ok(())
}
