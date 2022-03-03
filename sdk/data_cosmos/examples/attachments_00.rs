use azure_data_cosmos::prelude::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Now we create a sample struct.
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
        self.id.clone().into()
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
        id,
        a_string: "Something here".into(),
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // let's add an entity.
    match client.create_document(doc.clone()).into_future().await {
        Ok(_) => {
            println!("document created");
        }
        Err(err) => {
            println!("already exists? ==> {:?}", err);
        }
    };

    let document_client = client.into_document_client(doc.id.clone(), &doc.id)?;

    // list attachments
    let ret = document_client
        .list_attachments()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("list attachments == {:#?}", ret);

    // reference attachment
    println!("creating");
    let attachment_client = document_client.clone().into_attachment_client("myref06");
    let resp = attachment_client
        .create_reference(
            "https://cdn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
        )
        .consistency_level(ret)
        .into_future()
        .await?;
    println!("create reference == {:#?}", resp);

    // we pass the consistency level to make
    // sure to find the just created attachment
    let session_token: ConsistencyLevel = resp.into();

    let resp = attachment_client
        .get()
        .consistency_level(session_token)
        .into_future()
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
        .into_future()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    // slug attachment
    println!("creating slug attachment");
    let attachment_client = document_client.into_attachment_client("slug00".to_owned());
    let resp = attachment_client
        .create_slug("FFFFF".into())
        .consistency_level(&resp_delete)
        .content_type("text/plain")
        .into_future()
        .await?;

    println!("create slug == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment_client
        .delete()
        .consistency_level(&resp)
        .into_future()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    Ok(())
}
