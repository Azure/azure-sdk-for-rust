#![cfg(all(test, feature = "test_e2e"))]
use azure_core::Context;
use azure_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod setup;

// Now we create a sample struct. The Cow trick
// allows us to use the same struct for serializing
// (without having to own the items if not needed) and
// for deserializing (where owning is required).
// We do not need to define the "id" field here, it will be
// specified in the Document struct below.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct MySampleStruct<'a> {
    id: Cow<'a, str>,
    a_string: Cow<'a, str>,
    a_number: u64,
    a_timestamp: i64,
}

impl<'a> azure_cosmos::CosmosEntity<'a> for MySampleStruct<'a> {
    type Entity = &'a str;

    fn partition_key(&'a self) -> Self::Entity {
        self.id.as_ref()
    }
}

#[tokio::test]
async fn attachment() -> Result<(), azure_cosmos::Error> {
    const DATABASE_NAME: &str = "test-cosmos-db-attachment";
    const COLLECTION_NAME: &str = "test-collection-attachment";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
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

        let options = CreateCollectionOptions::new("/id")
            .offer(Offer::Throughput(400))
            .indexing_policy(ip);
        database_client
            .create_collection(Context::new(), COLLECTION_NAME, options)
            .await
            .unwrap()
    };

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    let id = format!("unique_id{}", 100);

    let doc = MySampleStruct {
        id: Cow::Borrowed(&id),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // let's add an entity.
    let session_token: ConsistencyLevel = collection_client
        .create_document(Context::new(), &doc, CreateDocumentOptions::new())
        .await?
        .into();

    let document_client = collection_client.into_document_client(id.clone(), &doc.id)?;

    // list attachments, there must be none.
    let ret = document_client
        .list_attachments()
        .consistency_level(session_token.clone())
        .execute()
        .await?;
    assert_eq!(0, ret.attachments.len());

    // create reference attachment
    let attachment_client = document_client.clone().into_attachment_client("reference");
    let resp = attachment_client
        .create_reference()
        .consistency_level(&ret)
        .execute("https://www.bing.com", "image/jpeg")
        .await?;

    // replace reference attachment
    let resp = attachment_client
        .replace_reference()
        .consistency_level(&resp)
        .execute("https://www.microsoft.com", "image/jpeg")
        .await?;

    // create slug attachment
    let attachment_client = document_client.clone().into_attachment_client("slug");
    let resp = attachment_client
        .create_slug()
        .consistency_level(&resp)
        .content_type("text/plain")
        .execute("something cool here")
        .await?;

    // list attachments, there must be two.
    let ret = document_client
        .list_attachments()
        .consistency_level(&resp)
        .execute()
        .await?;
    assert_eq!(2, ret.attachments.len());

    // get reference attachment, it must have the updated media link
    let reference_attachment = document_client
        .clone()
        .into_attachment_client("reference")
        .get()
        .consistency_level(&ret)
        .execute()
        .await?;
    assert_eq!(
        "https://www.microsoft.com",
        reference_attachment.attachment.media
    );

    // get slug attachment, it must have the text/plain content type
    println!("getting slug attachment");
    let slug_attachment = document_client
        .clone()
        .into_attachment_client("slug")
        .get()
        .consistency_level(&reference_attachment)
        .execute()
        .await
        .unwrap();
    assert_eq!("text/plain", slug_attachment.attachment.content_type);

    // delete slug attachment
    let resp_delete = attachment_client
        .delete()
        .consistency_level(&slug_attachment)
        .execute()
        .await?;

    // list attachments, there must be one.
    let ret = document_client
        .list_attachments()
        .consistency_level(&resp_delete)
        .execute()
        .await?;
    assert_eq!(1, ret.attachments.len());

    // delete the database
    database_client.delete_database().execute().await?;

    Ok(())
}
