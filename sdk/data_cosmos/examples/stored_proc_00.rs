/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_data_cosmos::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let client = CosmosClient::new(
        account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let ret = client
        .database_client(database)
        .collection_client(collection)
        .stored_procedure_client("test_proc")
        .execute_stored_procedure()
        .parameters(["Robert"])
        .into_future::<serde_json::Value>()
        .await?;

    println!("Response object:\n{:#?}", ret);
    println!("Response as JSON:\n{}", ret.payload);

    Ok(())
}
