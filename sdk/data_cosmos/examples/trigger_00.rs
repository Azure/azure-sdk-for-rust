use azure_data_cosmos::prelude::*;
use azure_data_cosmos::resources::trigger::{TriggerOperation, TriggerType};
use futures::stream::StreamExt;

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

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");
    let trigger_name = std::env::args()
        .nth(3)
        .expect("please specify trigger name as third command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database = client.database_client(database);
    let collection = database.collection_client(collection);
    let trigger = collection.clone().trigger_client(trigger_name);

    let ret = trigger
        .create_trigger("something", TriggerType::Post, TriggerOperation::All)
        .into_future()
        .await?;
    println!("Create response object:\n{:#?}", ret);

    let ret = trigger
        .replace_trigger(TRIGGER_BODY, TriggerType::Post, TriggerOperation::All)
        .consistency_level(ret)
        .into_future()
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let mut last_session_token: Option<ConsistencyLevel> = None;

    let mut stream = collection
        .list_triggers()
        .max_item_count(3)
        .consistency_level(&ret)
        .into_stream();
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        println!(
            "List loop received {} items. Object:\n{:#?}",
            ret.item_count, ret
        );
        last_session_token = Some(ConsistencyLevel::Session(ret.session_token));
    }

    let ret = trigger
        .delete_trigger()
        .consistency_level(last_session_token.unwrap())
        .into_future()
        .await?;
    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
