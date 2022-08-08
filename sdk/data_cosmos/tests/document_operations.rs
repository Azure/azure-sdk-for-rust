use serde::{Deserialize, Serialize};

mod setup_mock;

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
async fn document_operations() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-document";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-document";
    const DOCUMENT_NAME: &str = "test-document-name-create-and-delete-document";

    let client = setup_mock::initialize("document_operations").unwrap();

    client
        .create_database(DATABASE_NAME)
        .into_future()
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
        .into_future()
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
        .into_future()
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

    // now query all documents and see if we get the correct result
    let mut query_result = collection
        .query_documents("SELECT * FROM c")
        .query_cross_partition(true)
        .into_stream::<MyDocument>();

    // Ensure that the query stream can be sent to a task
    let query_result =
        tokio::spawn(async move { query_result.next().await.unwrap().unwrap().results })
            .await
            .unwrap();

    assert!(query_result.len() == 1);
    let (document, attributes) = &query_result[0];
    assert!(attributes.as_ref().unwrap().rid() == documents.documents[0].document_attributes.rid());
    assert_eq!(document, &document_data);

    // try to get the contents of the previously created document
    let document = collection
        .document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();

    let document_after_get = document
        .get_document::<MyDocument>()
        .into_future()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert_eq!(document.document.document, document_data);
    } else {
        panic!("document not found");
    }

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
        .into_future()
        .await
        .unwrap();

    // now get the replaced document
    let document = collection
        .document_client(DOCUMENT_NAME, &DOCUMENT_NAME)
        .unwrap();
    let document_after_get = document
        .get_document::<MyDocument>()
        .into_future()
        .await
        .unwrap();

    if let GetDocumentResponse::Found(document) = document_after_get {
        assert!(document.document.document.hello == 190);
    } else {
        panic!("document not found");
    }

    // delete document
    document.delete_document().into_future().await.unwrap();

    let mut documents = collection.list_documents().into_stream::<MyDocument>();

    // Ensure that the documents stream can be sent to a task
    let result = tokio::spawn(async move {
        let documents = documents.next().await.unwrap().unwrap().documents;
        documents.len() == 0
    })
    .await
    .unwrap();
    assert!(result, "Documents length was not 0");

    database.delete_database().into_future().await.unwrap();
}
