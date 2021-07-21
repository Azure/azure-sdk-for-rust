#![cfg(all(test, feature = "test_e2e"))]
use azure_core::Context;
use azure_cosmos::prelude::CreateDocumentOptions;
use serde::{Deserialize, Serialize};

mod setup;

use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use azure_cosmos::responses::GetDocumentResponse;
use collection::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MyDocument {
    id: String,
    hello: u32,
}

impl<'a> azure_cosmos::CosmosEntity<'a> for MyDocument {
    type Entity = &'a str;

    fn partition_key(&'a self) -> Self::Entity {
        self.id.as_ref()
    }
}

#[tokio::test]
async fn create_and_delete_document() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-document";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-document";
    const DOCUMENT_NAME: &str = "test-document-name-create-and-delete-document";

    let client = setup::initialize().unwrap();

    client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let options = CreateCollectionOptions::new("/id")
        .offer(Offer::Throughput(400))
        .indexing_policy(indexing_policy);
    database_client
        .create_collection(Context::new(), COLLECTION_NAME, options)
        .await
        .unwrap();

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    // create a new document
    let document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection_client
        .create_document(Context::new(), &document_data, CreateDocumentOptions::new())
        .await
        .unwrap();

    let documents = collection_client
        .list_documents()
        .execute::<MyDocument>()
        .await
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // try to get the contents of the previously created document
    let document_client = collection_client
        .clone()
        .into_document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();

    let document_after_get = document_client
        .get_document()
        .execute::<MyDocument>()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert_eq!(document.document.document, document_data);
    } else {
        panic!("document not found");
    }

    // delete document
    document_client.delete_document().execute().await.unwrap();

    let documents = collection_client
        .list_documents()
        .execute::<MyDocument>()
        .await
        .unwrap()
        .documents;
    assert!(documents.len() == 0);

    database_client.delete_database().execute().await.unwrap();
}

#[tokio::test]
async fn query_documents() {
    const DATABASE_NAME: &str = "test-cosmos-db-query-documents";
    const COLLECTION_NAME: &str = "test-collection-query-documents";
    const DOCUMENT_NAME: &str = "test-document-name-query-documents";

    let client = setup::initialize().unwrap();

    client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
        .await
        .unwrap();
    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let options = CreateCollectionOptions::new("/id")
        .indexing_policy(indexing_policy)
        .offer(Offer::S2);
    database_client
        .create_collection(Context::new(), COLLECTION_NAME, options)
        .await
        .unwrap();

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    // create a new document
    let document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection_client
        .create_document(Context::new(), &document_data, CreateDocumentOptions::new())
        .await
        .unwrap();

    let documents = collection_client
        .list_documents()
        .execute::<MyDocument>()
        .await
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // now query all documents and see if we get the correct result
    let query_result = collection_client
        .query_documents()
        .query_cross_partition(true)
        .execute::<MyDocument, _>("SELECT * FROM c")
        .await
        .unwrap()
        .into_documents()
        .unwrap()
        .results;

    assert!(query_result.len() == 1);
    assert!(query_result[0].document_attributes.rid() == documents[0].document_attributes.rid());
    assert_eq!(query_result[0].result, document_data);

    database_client.delete_database().execute().await.unwrap();
}

#[tokio::test]
async fn replace_document() {
    const DATABASE_NAME: &str = "test-cosmos-db-replace-documents";
    const COLLECTION_NAME: &str = "test-collection-replace-documents";
    const DOCUMENT_NAME: &str = "test-document-name-replace-documents";

    let client = setup::initialize().unwrap();

    client
        .create_database(
            azure_core::Context::new(),
            DATABASE_NAME,
            CreateDatabaseOptions::new(),
        )
        .await
        .unwrap();
    let database_client = client.into_database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    let options = CreateCollectionOptions::new("/id")
        .indexing_policy(indexing_policy)
        .offer(Offer::S2);
    database_client
        .create_collection(Context::new(), COLLECTION_NAME, options)
        .await
        .unwrap();

    let collection_client = database_client
        .clone()
        .into_collection_client(COLLECTION_NAME);

    // create a new document
    let mut document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection_client
        .create_document(Context::new(), &document_data, CreateDocumentOptions::new())
        .await
        .unwrap();

    let documents = collection_client
        .list_documents()
        .execute::<MyDocument>()
        .await
        .unwrap();
    assert!(documents.documents.len() == 1);

    // replace document with optimistic concurrency and session token
    document_data.hello = 190;
    collection_client
        .clone()
        .into_document_client(document_data.id.clone(), &document_data.id)
        .unwrap()
        .replace_document()
        .consistency_level(ConsistencyLevel::from(&documents))
        .if_match_condition(IfMatchCondition::Match(
            &documents.documents[0].document_attributes.etag(),
        ))
        .execute(&document_data)
        .await
        .unwrap();

    // now get the replaced document
    let document_client = collection_client
        .into_document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();
    let document_after_get = document_client
        .get_document()
        .execute::<MyDocument>()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert!(document.document.document.hello == 190);
    } else {
        panic!("document not found");
    }

    database_client.delete_database().execute().await.unwrap();
}
