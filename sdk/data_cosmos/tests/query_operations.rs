use azure_data_cosmos::prelude::*;
use futures::StreamExt;
use serde::Deserialize;

mod setup_mock;

#[tokio::test]
async fn query_operations() -> azure_core::Result<()> {
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Family {
        pub id: String,
        pub last_name: String,
        pub parents: Vec<Person>,
        pub children: Vec<Person>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Person {
        pub first_name: String,
    }

    let client = setup_mock::initialize("query_operations")?;

    let client = client
        .database_client("database1")
        .collection_client("container1");

    let query_obj = Query::new("select * from Family".into());

    let query = client
        .query_documents(query_obj)
        .query_cross_partition(true)
        .max_item_count(1);

    // We'll look at the results as `Family` structs.
    let mut stream = query.into_stream::<Family>();
    let mut last_names = Vec::new();
    while let Some(respo) = stream.next().await {
        let respo = respo?;
        assert!(respo.results.len() <= 1);
        let mut documents = respo.documents();
        if let Some(item) = documents.next() {
            last_names.push(item.last_name.clone());
        }
    }
    last_names.sort();

    assert_eq!(last_names, vec!["Andersen", "Wakefield"]);

    Ok(())
}
