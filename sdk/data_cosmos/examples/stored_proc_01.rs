/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_data_cosmos::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let function_body: &str = r#"
        function f(personToGreet) {
            var context = getContext();
            var response = context.getResponse();
            response.setBody("Hello, " + personToGreet);
        }
        "#;

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as second command line parameter");
    let stored_procedure_name = std::env::args()
        .nth(3)
        .expect("please specify the stored procedure name as third command line parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database = client.database_client(database_name);
    let collection = database.collection_client(collection_name);
    let stored_procedure = collection.stored_procedure_client(stored_procedure_name);

    let list_stored_procedures_response = collection
        .list_stored_procedures()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!(
        "list_stored_procedures_response == {:#?}",
        list_stored_procedures_response
    );

    let create_stored_procedure_response = stored_procedure
        .create_stored_procedure(function_body)
        .into_future()
        .await?;
    println!(
        "create_stored_procedure_response == {:#?}",
        create_stored_procedure_response
    );

    let execute_stored_procedure_response = stored_procedure
        .execute_stored_procedure()
        .parameters(["Robert"])
        .into_future::<serde_json::Value>()
        .await?;

    println!(
        "execute_stored_procedure_response == {:#?}",
        execute_stored_procedure_response
    );
    println!(
        "Response as JSON:\n{}",
        execute_stored_procedure_response.payload
    );

    let delete_stored_procedure_response = stored_procedure
        .delete_stored_procedure()
        .into_future()
        .await?;
    println!(
        "delete_stored_procedure_response == {:#?}",
        delete_stored_procedure_response
    );

    Ok(())
}
