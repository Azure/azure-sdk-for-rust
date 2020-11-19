/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_cosmos::prelude::*;
use azure_cosmos::stored_procedure::Parameters;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let function_body: &str = r#"
        function f(personToGreet) {
            var context = getContext();
            var response = context.getResponse();
            response.setBody("Hello, " + personToGreet);
        }
        "#;

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as second command line parameter");
    let stored_procedure_name = std::env::args()
        .nth(3)
        .expect("please specify the stored procedure name as third command line parameter");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = CosmosClient::new(account, authorization_token);
    let database_client = client.into_database_client(database_name);
    let collection_client = database_client.into_collection_client(collection_name);
    let stored_procedure_client = collection_client
        .clone()
        .into_stored_procedure_client(stored_procedure_name);

    let list_stored_procedures_response =
        collection_client.list_stored_procedures().execute().await?;
    println!(
        "list_stored_procedures_response == {:#?}",
        list_stored_procedures_response
    );

    let create_stored_procedure_response = stored_procedure_client
        .create_stored_procedure()
        .with_body(&function_body)
        .execute()
        .await?;
    println!(
        "create_stored_procedure_response == {:#?}",
        create_stored_procedure_response
    );

    let execute_stored_procedure_response = stored_procedure_client
        .execute_stored_procedure()
        .with_parameters(Parameters::new().push("Robert")?)
        .execute::<serde_json::Value>()
        .await?;

    println!(
        "execute_stored_procedure_response == {:#?}",
        execute_stored_procedure_response
    );
    println!(
        "Response as JSON:\n{}",
        execute_stored_procedure_response.payload.to_string()
    );

    let delete_stored_procedure_response = stored_procedure_client
        .delete_stored_procedure()
        .execute()
        .await?;
    println!(
        "delete_stored_procedure_response == {:#?}",
        delete_stored_procedure_response
    );

    Ok(())
}
