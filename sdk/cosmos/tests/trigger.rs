#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;

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
async fn trigger() -> Result<(), CosmosError> {
    const DATABASE_NAME: &str = "test-cosmos-db-trigger";
    const COLLECTION_NAME: &str = "test-udf";
    const TRIGGER_NAME: &str = "test";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .execute(DATABASE_NAME)
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a temp collection
    let _create_collection_response = {
        let indexes = collection::IncludedPathIndex {
            kind: collection::KeyKind::Hash,
            data_type: collection::DataType::String,
            precision: Some(3),
        };

        let ip = collection::IncludedPath {
            path: "/*".to_owned(),
            indexes: Some(vec![indexes]),
        };

        let ip = collection::IndexingPolicy {
            automatic: true,
            indexing_mode: collection::IndexingMode::Consistent,
            included_paths: vec![ip],
            excluded_paths: vec![],
        };

        database_client
            .create_collection("/id")
            .offer(Offer::Throughput(400))
            .indexing_policy(ip)
            .execute(COLLECTION_NAME)
            .await
            .unwrap()
    };

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);
    let trigger_client = collection_client.clone().into_trigger_client(TRIGGER_NAME);

    let ret = trigger_client
        .create_trigger()
        .execute(
            "something",
            trigger::TriggerType::Post,
            trigger::TriggerOperation::All,
        )
        .await?;

    let ret = trigger_client
        .replace_trigger()
        .consistency_level(ret)
        .execute(
            TRIGGER_BODY,
            trigger::TriggerType::Post,
            trigger::TriggerOperation::All,
        )
        .await?;

    let mut last_session_token: Option<ConsistencyLevel> = None;

    let stream = collection_client
        .list_triggers()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        last_session_token = Some(ConsistencyLevel::Session(ret.session_token));
    }

    let _ret = trigger_client
        .delete_trigger()
        .consistency_level(last_session_token.unwrap())
        .execute()
        .await?;

    // delete the database
    database_client.delete_database().execute().await?;

    Ok(())
}
