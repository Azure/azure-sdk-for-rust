use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

mod setup_mock;

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
async fn trigger_operations() -> azure_core::Result<()> {
    const DATABASE_NAME: &str = "test-cosmos-db-trigger";
    const COLLECTION_NAME: &str = "test-udf";
    const TRIGGER_NAME: &str = "test";

    let client = setup_mock::initialize("trigger_operations")?;

    // create a temp database
    let _create_database_response = client.create_database(DATABASE_NAME).into_future().await?;

    let database = client.database_client(DATABASE_NAME);

    // create a temp collection
    let _ = database
        .create_collection(COLLECTION_NAME, "/id")
        .into_future()
        .await?;

    let collection = database.collection_client(COLLECTION_NAME);
    let trigger = collection.trigger_client(TRIGGER_NAME);

    let ret = trigger
        .create_trigger(
            "something",
            trigger::TriggerType::Post,
            trigger::TriggerOperation::All,
        )
        .into_future()
        .await?;

    let ret = trigger
        .replace_trigger(
            TRIGGER_BODY,
            trigger::TriggerType::Post,
            trigger::TriggerOperation::All,
        )
        .consistency_level(ret)
        .into_future()
        .await?;

    let mut last_session_token: Option<ConsistencyLevel> = None;

    let stream = collection
        .list_triggers()
        .max_item_count(3)
        .consistency_level(&ret);
    let mut stream = stream.into_stream();
    while let Some(ret) = stream.next().await {
        let ret = ret?;
        last_session_token = Some(ConsistencyLevel::Session(ret.session_token));
    }

    let _ = trigger
        .delete_trigger()
        .consistency_level(last_session_token.expect("no triggers were found in collection"))
        .into_future()
        .await?;

    // delete the database
    database.delete_database().into_future().await?;

    Ok(())
}
