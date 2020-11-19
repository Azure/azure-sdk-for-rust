use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

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

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let client = client.into_database_client(database_name);
    let client = client.into_collection_client(collection_name);

    let id = format!("unique_id{}", 100);

    let doc = Document::new(MySampleStruct {
        id: Cow::Borrowed(&id),
        a_string: Cow::Borrowed("Something here"),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    });

    // let's add an entity.
    match client
        .create_document()
        .with_partition_keys(PartitionKeys::new().push(&doc.document.id)?)
        .execute_with_document(&doc)
        .await
    {
        Ok(_) => {
            println!("document created");
        }
        Err(err) => {
            println!("already exists? ==> {:?}", err);
        }
    };

    let mut partition_keys = PartitionKeys::new();
    partition_keys.push(&doc.document.id)?;
    let document_client = client.into_document_client(id, partition_keys);

    // list attachments
    let ret = document_client.list_attachments().execute().await?;
    println!("list attachments == {:#?}", ret);

    // reference attachment
    println!("creating");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let resp = attachment_client
        .create_reference()
        .with_consistency_level((&ret).into())
        .with_content_type("image/jpeg")
        .with_media(
            "https://cdn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
        )
        .execute()
        .await?;
    println!("create reference == {:#?}", resp);

    // we pass the consistency level to make
    // sure to find the just created attachment
    let session_token: ConsistencyLevel = resp.into();

    let resp = attachment_client
        .get()
        .with_consistency_level(session_token)
        .execute()
        .await?;

    println!("get attachment == {:#?}", resp);
    let session_token: ConsistencyLevel = resp.into();

    println!("replacing");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let resp = attachment_client
        .replace_reference()
        .with_consistency_level(session_token)
        .with_content_type("image/jpeg")
        .with_media(
            "https://Adn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
        )
        .execute()
        .await?;
    println!("replace reference == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment_client
        .delete()
        .with_consistency_level((&resp).into())
        .execute()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    // slug attachment
    println!("creating slug attachment");
    let attachment_client = document_client.into_attachment_client("slug00".to_owned());
    let resp = attachment_client
        .create_slug()
        .with_consistency_level((&resp_delete).into())
        .with_content_type("text/plain")
        .with_body(b"FFFFF")
        .execute()
        .await?;

    println!("create slug == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment_client
        .delete()
        .with_consistency_level((&resp).into())
        .execute()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    Ok(())
}
