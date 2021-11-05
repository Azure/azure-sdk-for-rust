use azure_databricks::client::*;
use azure_databricks::clusters::ClusterDriver;
use azure_databricks::prelude::*;

#[tokio::main]
async fn main() -> DatabricksResult<()> {
    // to connect to databricks api, we need at least, the host and personal access token

    // get host_name from environment variables (includes the https://)
    let host_name: String =
        std::env::var("DATABRICKS_INSTANCE").expect("Set env variable DATABRICKS_INSTANCE first!");

    // get personal access token to databricks workspace
    let token: String =
        std::env::var("DATABRICKS_TOKEN").expect("Set env variable DATABRICKS_TOKEN first!");

    let client = DatabricksClient::new(&token, &host_name)?;

    let cluster_driver = ClusterDriver::new(client.clone())?;

    let clusters_list = cluster_driver.get_list().await?;

    println!("{:#?}", clusters_list);
    Ok(())
}
