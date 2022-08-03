#![cfg(all(test, feature = "test_e2e"))]
use azure_data_cosmos::prelude::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

mod setup;

// Now we create a sample struct. The Cow trick
// allows us to use the same struct for serializing
// (without having to own the items if not needed) and
// for deserializing (where owning is required).
// We do not need to define the "id" field here, it will be
// specified in the Document struct below.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct MySampleStruct {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

#[tokio::test]
async fn attachment_operations() -> azure_core::Result<()> {
    const DATABASE_NAME: &str = "test-cosmos-db-attachment";
    const COLLECTION_NAME: &str = "test-collection-attachment";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database(DATABASE_NAME)
        .into_future()
        .await
        .unwrap();

    let database = client.database_client(DATABASE_NAME);

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

        database
            .create_collection(COLLECTION_NAME, "/id")
            .offer(Offer::Throughput(400))
            .indexing_policy(ip)
            .into_future()
            .await
            .unwrap()
    };

    let collection = database.collection_client(COLLECTION_NAME);

    let id = format!("unique_id{}", 100);

    let doc = MySampleStruct {
        id: id.clone(),
        a_string: "Something here".into(),
        a_number: 100,
        a_timestamp: OffsetDateTime::now_utc().unix_timestamp(),
    };

    // let's add an entity.
    let session_token: ConsistencyLevel = collection
        .create_document(doc.clone())
        .into_future()
        .await?
        .into();

    let document = collection.document_client(id.clone(), &doc.id)?;

    // list attachments, there must be none.
    let ret = document
        .list_attachments()
        .consistency_level(session_token.clone())
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(0, ret.attachments.len());

    // create reference attachment
    let attachment = document.attachment_client("reference");
    let resp = attachment
        .create_attachment("https://www.bing.com", "image/jpeg")
        .consistency_level(&ret)
        .into_future()
        .await?;

    // replace reference attachment
    let resp = attachment
        .replace_attachment("https://www.microsoft.com", "image/jpeg")
        .consistency_level(&resp)
        .into_future()
        .await?;

    // create slug attachment
    let attachment = document.attachment_client("slug");
    let resp = attachment
        .create_slug("something cool here".into())
        .consistency_level(&resp)
        .content_type("text/plain")
        .into_future()
        .await?;

    // list attachments, there must be two.
    let ret = document
        .list_attachments()
        .consistency_level(&resp)
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(2, ret.attachments.len());

    // get reference attachment, it must have the updated media link
    let reference_attachment = document
        .attachment_client("reference")
        .get()
        .consistency_level(&ret)
        .into_future()
        .await?;
    assert_eq!(
        "https://www.microsoft.com",
        reference_attachment.attachment.media
    );

    // get slug attachment, it must have the text/plain content type
    println!("getting slug attachment");
    let slug_attachment = document
        .attachment_client("slug")
        .get()
        .consistency_level(&reference_attachment)
        .into_future()
        .await
        .unwrap();
    assert_eq!("text/plain", slug_attachment.attachment.content_type);

    // delete slug attachment
    let resp_delete = attachment
        .delete()
        .consistency_level(&slug_attachment)
        .into_future()
        .await?;

    // list attachments, there must be one.
    let ret = document
        .list_attachments()
        .consistency_level(&resp_delete)
        .into_stream()
        .next()
        .await
        .unwrap()?;
    assert_eq!(1, ret.attachments.len());

    // delete the database
    database.delete_database().into_future().await?;

    Ok(())
}
