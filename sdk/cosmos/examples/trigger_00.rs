use azure_cosmos::prelude::*;
use azure_cosmos::trigger::*;
use futures::stream::StreamExt;
use std::error::Error;

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
async fn main() -> Result<(), Box<dyn Error>> {
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
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let database_client = client.into_database_client(database);
    let collection_client = database_client.into_collection_client(collection);
    let trigger_client = collection_client.clone().into_trigger_client(trigger_name);

    let ret = trigger_client
        .create_trigger()
        .with_trigger_type(TriggerType::Post)
        .with_trigger_operation(TriggerOperation::All)
        .with_body(&"something")
        .execute()
        .await?;
    println!("Creeate response object:\n{:#?}", ret);

    let ret = trigger_client
        .replace_trigger()
        .with_consistency_level(ret.into())
        .with_trigger_type(TriggerType::Post)
        .with_trigger_operation(TriggerOperation::All)
        .with_body(&TRIGGER_BODY)
        .execute()
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let mut last_session_token: Option<ConsistencyLevel> = None;

    let stream = collection_client
        .list_triggers()
        .with_max_item_count(3)
        .with_consistency_level((&ret).into());
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        println!(
            "List loop received {} items. Object:\n{:#?}",
            ret.item_count, ret
        );
        last_session_token = Some(ret.session_token.into());
    }

    let ret = trigger_client
        .delete_trigger()
        .with_consistency_level(last_session_token.unwrap())
        .execute()
        .await?;
    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
