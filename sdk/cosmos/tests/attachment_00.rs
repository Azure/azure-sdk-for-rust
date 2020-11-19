#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

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

#[tokio::test]
async fn attachment() -> Result<(), Box<dyn Error>> {
    const DATABASE_NAME: &str = "test-cosmos-db-attachment";
    const COLLECTION_NAME: &str = "test-collection-attachment";

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

    let id = format!("unique_id{}", 100);

    let doc = Document::new(MySampleStruct {
        id: Cow::Borrowed(&id),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    });

    // let's add an entity.
    let session_token: ConsistencyLevel = collection_client
        .create_document()
        .with_partition_keys(PartitionKeys::new().push(&doc.document.id)?)
        .execute_with_document(&doc)
        .await?
        .into();

    let mut partition_keys = PartitionKeys::new();
    partition_keys.push(doc.document.id)?;
    let document_client = collection_client.into_document_client(&id, partition_keys);

    // list attachments, there must be none.
    let ret = document_client
        .list_attachments()
        .with_consistency_level(session_token.clone())
        .execute()
        .await?;
    assert_eq!(0, ret.attachments.len());

    // create reference attachment
    let attachment_client = document_client.with_attachment_client("reference");
    let resp = attachment_client
        .create_reference()
        .with_consistency_level((&ret).into())
        .with_content_type("image/jpeg")
        .with_media("https://www.bing.com")
        .execute()
        .await?;

    // replace reference attachment
    let attachment_client = document_client.with_attachment_client("reference");
    let resp = attachment_client
        .replace_reference()
        .with_consistency_level((&resp).into())
        .with_content_type("image/jpeg")
        .with_media("https://www.microsoft.com")
        .execute()
        .await?;

    // create slug attachment
    let attachment_client = document_client.with_attachment_client("slug");
    let resp = attachment_client
        .create_slug()
        .with_consistency_level((&resp).into())
        .with_content_type("text/plain")
        .with_body(b"something cool here")
        .execute()
        .await?;

    // list attachments, there must be two.
    let ret = document_client
        .list_attachments()
        .with_consistency_level((&resp).into())
        .execute()
        .await?;
    assert_eq!(2, ret.attachments.len());

    // get reference attachment, it must have the updated media link
    let reference_attachment = document_client
        .with_attachment_client("reference")
        .get()
        .with_consistency_level((&ret).into())
        .execute()
        .await?;
    assert_eq!(
        "https://www.microsoft.com",
        reference_attachment.attachment.media
    );

    // get slug attachment, it must have the text/plain content type
    println!("getting slug attachment");
    let slug_attachment = document_client
        .with_attachment_client("slug")
        .get()
        .with_consistency_level((&reference_attachment).into())
        .execute()
        .await
        .unwrap();
    assert_eq!("text/plain", slug_attachment.attachment.content_type);

    // delete slug attachment
    let resp_delete = attachment_client
        .delete()
        .with_consistency_level((&slug_attachment).into())
        .execute()
        .await?;

    // list attachments, there must be one.
    let ret = document_client
        .list_attachments()
        .with_consistency_level((&resp_delete).into())
        .execute()
        .await?;
    assert_eq!(1, ret.attachments.len());

    // delete the database
    database_client.delete_database().execute().await?;

    Ok(())
}
