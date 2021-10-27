use azure_core::Context;
use azure_cosmos::prelude::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;

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

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let id = format!("unique_id{}", 100);

    let doc = MySampleStruct {
        id: Cow::Borrowed(&id),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // let's add an entity.
    match client
        .create_document(Context::new(), &doc, CreateDocumentOptions::new())
        .await
    {
        Ok(_) => {
            println!("document created");
        }
        Err(err) => {
            println!("already exists? ==> {:?}", err);
        }
    };

    let document_client = client.into_document_client(doc.id.clone(), &doc.id)?;

    // list attachments
    let options = ListAttachmentsOptions::new();
    let ret = Box::pin(document_client.list_attachments(Context::new(), options))
        .next()
        .await
        .unwrap()?;
    println!("list attachments == {:#?}", ret);

    // reference attachment
    println!("creating");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let options = CreateReferenceAttachmentOptions::new().consistency_level(ret);
    let resp = attachment_client
        .create_reference(
            Context::new(),
            "https://cdn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
            options,
        )
        .await?;
    println!("create reference == {:#?}", resp);

    // we pass the consistency level to make
    // sure to find the just created attachment
    let session_token: ConsistencyLevel = resp.into();

    let resp = attachment_client
        .get(
            Context::new(),
            GetAttachmentOptions::new().consistency_level(session_token),
        )
        .await?;

    println!("get attachment == {:#?}", resp);
    let session_token: ConsistencyLevel = resp.into();

    println!("replacing");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let options = ReplaceReferenceAttachmentOptions::new().consistency_level(session_token);
    let resp = attachment_client
        .replace_reference(
            Context::new(),
            "https://Adn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
            options,
        )
        .await?;
    println!("replace reference == {:#?}", resp);

    println!("deleting");
    let options = DeleteAttachmentOptions::new().consistency_level(&resp);
    let resp_delete = attachment_client.delete(Context::new(), options).await?;
    println!("delete attachment == {:#?}", resp_delete);

    // slug attachment
    println!("creating slug attachment");
    let attachment_client = document_client.into_attachment_client("slug00".to_owned());
    let options = CreateSlugAttachmentOptions::new()
        .consistency_level(&resp_delete)
        .content_type("text/plain");
    let resp = attachment_client
        .create_slug(Context::new(), "FFFFF", options)
        .await?;

    println!("create slug == {:#?}", resp);

    // slug replacement
    println!("replacing slug attachment");
    let options = ReplaceSlugAttachmentOptions::new()
        .consistency_level(&resp_delete)
        .content_type("text/plain");
    let resp = attachment_client
        .replace_slug(Context::new(), "12345", options)
        .await?;

    println!("deleting");
    let options = DeleteAttachmentOptions::new().consistency_level(&resp);
    let resp_delete = attachment_client.delete(Context::new(), options).await?;
    println!("delete attachment == {:#?}", resp_delete);

    Ok(())
}
