/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use clap::Parser;
use futures::StreamExt;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The name of the stored procedure
    stored_procedure_name: String,
}

const FUNCTION_BODY: &str = r#"
    function f(personToGreet) {
        var context = getContext();
        var response = context.getResponse();
        response.setBody("Hello, " + personToGreet);
    }
"#;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args.auth.into_client()?;

    let collection = client
        .database_client(args.database_name)
        .collection_client(args.collection_name);
    let stored_procedure = collection.stored_procedure_client(args.stored_procedure_name);

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
        .create_stored_procedure(FUNCTION_BODY)
        .into_future()
        .await?;
    println!(
        "create_stored_procedure_response == {:#?}",
        create_stored_procedure_response
    );

    let execute_stored_procedure_response = stored_procedure
        .execute_stored_procedure::<serde_json::Value>()
        .parameters(["Robert"])
        .into_future()
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
