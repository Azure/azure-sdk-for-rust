#![cfg(feature = "mock_transport_framework")]
use azure_core::Context;
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
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

    let client = setup::initialize("attachment")?;

    // create a temp database
    let _create_database_response = client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
        .await?;

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
            .await?
    };

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    let id = format!("unique_id{}", 100);

    let doc = MySampleStruct {
        id: Cow::Borrowed(&id),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
    };

    // let's add an entity.
    let session_token: ConsistencyLevel = collection_client
        .create_document(Context::new(), &doc, CreateDocumentOptions::new())
        .await?
        .into();

    let document_client = collection_client.into_document_client(id.clone(), &doc.id)?;

    // list attachments, there must be none.
    let options = ListAttachmentsOptions::new().consistency_level(session_token.clone());
    let ret = Box::pin(document_client.list_attachments(Context::new(), options))
        .next()
        .await
        .unwrap()?;
    assert_eq!(0, ret.attachments.len());

    // create reference attachment
    let attachment_client = document_client.clone().into_attachment_client("reference");
    let options = CreateReferenceAttachmentOptions::new().consistency_level(&ret);
    let resp = attachment_client
        .create_reference(
            Context::new(),
            "https://www.bing.com",
            "image/jpeg",
            options,
        )
        .await?;

    // replace reference attachment
    let options = ReplaceReferenceAttachmentOptions::new().consistency_level(&resp);
    let resp = attachment_client
        .replace_reference(
            Context::new(),
            "https://www.microsoft.com",
            "image/jpeg",
            options,
        )
        .await?;

    // create slug attachment
    let attachment_client = document_client.clone().into_attachment_client("slug");
    let options = CreateSlugAttachmentOptions::new()
        .consistency_level(&resp)
        .content_type("text/plain");
    let resp = attachment_client
        .create_slug(Context::new(), "something cool here", options)
        .await?;

    // list attachments, there must be two.
    let options = ListAttachmentsOptions::new().consistency_level(&resp);
    let ret = Box::pin(document_client.list_attachments(Context::new(), options))
        .next()
        .await
        .unwrap()?;
    assert_eq!(2, ret.attachments.len());

    // get reference attachment, it must have the updated media link
    let attachment_client = document_client.clone().into_attachment_client("reference");
    let options = GetAttachmentOptions::new().consistency_level(&ret);
    let reference_attachment = attachment_client.get(Context::new(), options).await?;
    assert_eq!(
        "https://www.microsoft.com",
        reference_attachment.attachment.media
    );

    // get slug attachment, it must have the text/plain content type
    let attachment_client = document_client.clone().into_attachment_client("slug");
    let options = GetAttachmentOptions::new().consistency_level(&reference_attachment);
    let slug_attachment = attachment_client.get(Context::new(), options).await?;
    assert_eq!("text/plain", slug_attachment.attachment.content_type);

    // delete slug attachment
    let options = DeleteAttachmentOptions::new().consistency_level(&slug_attachment);
    let resp_delete = attachment_client.delete(Context::new(), options).await?;

    // list attachments, there must be one.
    let options = ListAttachmentsOptions::new().consistency_level(&resp_delete);
    let ret = Box::pin(document_client.list_attachments(Context::new(), options))
        .next()
        .await
        .unwrap()?;
    assert_eq!(1, ret.attachments.len());

    // delete the database
    let options = DeleteDatabaseOptions::new();
    database_client
        .delete_database(Context::new(), options)
        .await?;

    Ok(())
}
