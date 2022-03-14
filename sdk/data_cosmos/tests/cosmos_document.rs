#![cfg(all(test, feature = "test_e2e"))]
use serde::{Deserialize, Serialize};

mod setup;

use azure_core::prelude::*;
use azure_data_cosmos::prelude::*;
use collection::*;
use futures::StreamExt;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct MyDocument {
    id: String,
    hello: u32,
}

impl azure_data_cosmos::CosmosEntity for MyDocument {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

#[tokio::test]
async fn create_and_delete_document() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-document";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-document";
    const DOCUMENT_NAME: &str = "test-document-name-create-and-delete-document";

    let client = setup::initialize().unwrap();

    client
        .create_database(DATABASE_NAME)

        .await
        .unwrap();

    let database = client.database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    database
        .create_collection(COLLECTION_NAME, "/id")
        .offer(Offer::Throughput(400))
        .indexing_policy(indexing_policy)

        .await
        .unwrap();

    let collection = database.collection_client(COLLECTION_NAME);

    // create a new document
    let document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection
        .create_document(document_data.clone())

        .await
        .unwrap();

    let documents = collection
        .list_documents()
        .into_stream::<MyDocument>()
        .next()
        .await
        .unwrap()
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // try to get the contents of the previously created document
    let document = collection
        .document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();

    let document_after_get = document
        .get_document()
        .into_future::<MyDocument>()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert_eq!(document.document.document, document_data);
    } else {
        panic!("document not found");
    }

    // delete document
    document.delete_document().await.unwrap();

    let documents = collection
        .list_documents()
        .into_stream::<MyDocument>()
        .next()
        .await
        .unwrap()
        .unwrap()
        .documents;
    assert!(documents.len() == 0);

    database.delete_database().await.unwrap();
}

#[tokio::test]
async fn query_documents() {
    const DATABASE_NAME: &str = "test-cosmos-db-query-documents";
    const COLLECTION_NAME: &str = "test-collection-query-documents";
    const DOCUMENT_NAME: &str = "test-document-name-query-documents";

    let client = setup::initialize().unwrap();

    client
        .create_database(DATABASE_NAME)

        .await
        .unwrap();
    let database = client.database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    database
        .create_collection(COLLECTION_NAME, "/id")
        .indexing_policy(indexing_policy)
        .offer(Offer::S2)

        .await
        .unwrap();

    let collection = database.collection_client(COLLECTION_NAME);

    // create a new document
    let document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection
        .create_document(document_data.clone())

        .await
        .unwrap();

    let documents = collection
        .list_documents()
        .into_stream::<MyDocument>()
        .next()
        .await
        .unwrap()
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // now query all documents and see if we get the correct result
    let query_result = collection
        .query_documents("SELECT * FROM c")
        .query_cross_partition(true)
        .into_stream::<MyDocument>()
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_documents()
        .unwrap()
        .results;

    assert!(query_result.len() == 1);
    assert!(query_result[0].document_attributes.rid() == documents[0].document_attributes.rid());
    assert_eq!(query_result[0].result, document_data);

    database.delete_database().await.unwrap();
}

#[tokio::test]
async fn replace_document() {
    const DATABASE_NAME: &str = "test-cosmos-db-replace-documents";
    const COLLECTION_NAME: &str = "test-collection-replace-documents";
    const DOCUMENT_NAME: &str = "test-document-name-replace-documents";

    let client = setup::initialize().unwrap();

    client
        .create_database(DATABASE_NAME)

        .await
        .unwrap();
    let database = client.database_client(DATABASE_NAME);

    // create a new collection
    let indexing_policy = IndexingPolicy {
        automatic: true,
        indexing_mode: IndexingMode::Consistent,
        included_paths: vec![],
        excluded_paths: vec![],
    };

    database
        .create_collection(COLLECTION_NAME, "/id")
        .indexing_policy(indexing_policy)
        .offer(Offer::S2)

        .await
        .unwrap();

    let collection = database.collection_client(COLLECTION_NAME);

    // create a new document
    let mut document_data = MyDocument {
        id: DOCUMENT_NAME.to_owned(),
        hello: 42,
    };
    collection
        .create_document(document_data.clone())

        .await
        .unwrap();

    let documents = collection
        .list_documents()
        .into_stream::<MyDocument>()
        .next()
        .await
        .unwrap()
        .unwrap();
    assert!(documents.documents.len() == 1);

    // replace document with optimistic concurrency and session token
    document_data.hello = 190;
    collection
        .document_client(document_data.id.clone(), &document_data.id)
        .unwrap()
        .replace_document(document_data)
        .consistency_level(ConsistencyLevel::from(&documents))
        .if_match_condition(IfMatchCondition::Match(
            documents.documents[0]
                .document_attributes
                .etag()
                .to_string(),
        ))

        .await
        .unwrap();

    // now get the replaced document
    let document = collection
        .document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();
    let document_after_get = document
        .get_document()
        .into_future::<MyDocument>()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert!(document.document.document.hello == 190);
    } else {
        panic!("document not found");
    }

    database.delete_database().await.unwrap();
}
