#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate serde_derive;

use azure_sdk_cosmos::collection::*;
use azure_sdk_cosmos::query::Query;

mod setup;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Document {
    id: String, // required field
    hello: u32,
}

#[test]
fn create_and_delete_document() {
    const DATABASE_NAME: &str = "test-cosmos-db-create-and-delete-document";
    const COLLECTION_NAME: &str = "test-collection-create-and-delete-document";
    const DOCUMENT_NAME: &str = "test-document-name-create-and-delete-document";

    let (client, mut core) = setup::initialize().unwrap();

    core.run(client.create_database(DATABASE_NAME)).unwrap();

    // create a new collection
    let collection_to_create = Collection::new(
        COLLECTION_NAME,
        IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![],
            excluded_paths: vec![],
        },
    );
    core.run(client.create_collection(DATABASE_NAME, 400, &collection_to_create))
        .unwrap();

    // create a new document
    let document_data = Document {
        id: DOCUMENT_NAME.to_string(),
        hello: 42,
    };
    core.run(client.create_document(DATABASE_NAME, COLLECTION_NAME, &document_data).execute())
        .unwrap();
    let documents = core
        .run(client.list_documents(DATABASE_NAME, COLLECTION_NAME).execute::<Document>())
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // try to get the contents of the previously created document
    let document_request = client
        .get_document(DATABASE_NAME, COLLECTION_NAME, DOCUMENT_NAME)
        .execute::<Document>();
    let document_after_get = core.run(document_request).unwrap().document.expect("No document found!");
    assert_eq!(document_after_get.entity, document_data);

    // delete document
    core.run(client.delete_document(DATABASE_NAME, COLLECTION_NAME, DOCUMENT_NAME).execute())
        .unwrap();
    let documents = core
        .run(client.list_documents(DATABASE_NAME, COLLECTION_NAME).execute::<Document>())
        .unwrap()
        .documents;
    assert!(documents.len() == 0);

    core.run(client.delete_database(DATABASE_NAME)).unwrap();
}

#[test]
fn query_documents() {
    const DATABASE_NAME: &str = "test-cosmos-db-query-documents";
    const COLLECTION_NAME: &str = "test-collection-query-documents";
    const DOCUMENT_NAME: &str = "test-document-name-query-documents";

    let (client, mut core) = setup::initialize().unwrap();

    core.run(client.create_database(DATABASE_NAME)).unwrap();

    // create a new collection
    let collection_to_create = Collection::new(
        COLLECTION_NAME,
        IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![],
            excluded_paths: vec![],
        },
    );
    core.run(client.create_collection(DATABASE_NAME, 400, &collection_to_create))
        .unwrap();

    // create a new document
    let document_data = Document {
        id: DOCUMENT_NAME.to_string(),
        hello: 42,
    };
    let document = core
        .run(client.create_document(DATABASE_NAME, COLLECTION_NAME, &document_data).execute())
        .unwrap();
    let documents = core
        .run(client.list_documents(DATABASE_NAME, COLLECTION_NAME).execute::<Document>())
        .unwrap()
        .documents;
    assert!(documents.len() == 1);

    // now query all documents and see if we get the correct result
    let query_request = client
        .query_documents(DATABASE_NAME, COLLECTION_NAME, Query::new("SELECT * FROM c"))
        .execute::<Document>();
    let query_result = core.run(query_request).unwrap().results;

    assert!(query_result.len() == 1);
    assert!(query_result[0].document_attributes.as_ref().unwrap().rid() == document.rid());
    assert_eq!(query_result[0].result, document_data);

    core.run(client.delete_database(DATABASE_NAME)).unwrap();
}

#[test]
#[ignore]
fn replace_document() {}
