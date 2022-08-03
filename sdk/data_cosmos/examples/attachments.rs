use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

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
        self.id.clone()
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Name of the database.
    database_name: String,
    /// Name of the collection in the database
    collection_name: String,
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
}

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token)
        .database_client(args.database_name)
        .collection_client(args.collection_name);

    let doc = MySampleStruct {
        id: format!("unique_id{}", 100),
        a_string: "Something here".into(),
        a_number: 100,
        a_timestamp: OffsetDateTime::now_utc().unix_timestamp(),
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

    let document = client.document_client(doc.id.clone(), &doc.id)?;

    // list attachments
    let ret = document
        .list_attachments()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("list attachments == {:#?}", ret);

    // reference attachment
    println!("creating");
    let attachment = document.attachment_client("myref06");
    let resp = attachment
        .create_attachment(
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

    let resp = attachment
        .get()
        .consistency_level(session_token)
        .into_future()
        .await?;

    println!("get attachment == {:#?}", resp);
    let session_token: ConsistencyLevel = resp.into();

    println!("replacing");
    let attachment = document.attachment_client("myref06");
    let resp = attachment
        .replace_attachment(
            "https://Adn.pixabay.com/photo/2020/01/11/09/30/abstract-background-4756987__340.jpg",
            "image/jpeg",
        )
        .consistency_level(session_token)
        .into_future()
        .await?;
    println!("replace reference == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment
        .delete()
        .consistency_level(&resp)
        .into_future()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    // slug attachment
    println!("creating slug attachment");
    let attachment = document.attachment_client("slug00".to_owned());
    let resp = attachment
        .create_slug("FFFFF".into())
        .consistency_level(&resp_delete)
        .content_type("text/plain")
        .into_future()
        .await?;

    println!("create slug == {:#?}", resp);

    println!("deleting");
    let resp_delete = attachment
        .delete()
        .consistency_level(&resp)
        .into_future()
        .await?;
    println!("delete attachment == {:#?}", resp_delete);

    Ok(())
}
