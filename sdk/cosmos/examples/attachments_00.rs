use azure_core::Context;
use azure_cosmos::prelude::*;
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
    let ret = document_client.list_attachments().execute().await?;
    println!("list attachments == {:#?}", ret);

    // reference attachment
    println!("creating");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let resp = attachment_client
        .create_reference()
        .consistency_level(ret)
        .execute(
            "https://cdn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
        )
        .await?;
    println!("create reference == {:#?}", resp);

    // we pass the consistency level to make
    // sure to find the just created attachment
    let session_token: ConsistencyLevel = resp.into();

    let resp = attachment_client
        .get()
        .consistency_level(session_token)
        .execute()
        .await?;

    println!("get attachment == {:#?}", resp);
    let session_token: ConsistencyLevel = resp.into();

    println!("replacing");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let resp = attachment_client
        .replace_reference()
        .consistency_level(session_token)
        .execute(
            "https://Adn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
        )
        .await?;
    println!("replace reference == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment_client
        .delete()
        .consistency_level(&resp)
        .execute()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    // slug attachment
    println!("creating slug attachment");
    let attachment_client = document_client.into_attachment_client("slug00".to_owned());
    let resp = attachment_client
        .create_slug()
        .consistency_level(&resp_delete)
        .content_type("text/plain")
        .execute("FFFFF")
        .await?;

    println!("create slug == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment_client
        .delete()
        .consistency_level(&resp)
        .execute()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    Ok(())
}
