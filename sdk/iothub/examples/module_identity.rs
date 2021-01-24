use std::sync::Arc;

use azure_core::HttpClient;
use iothub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let module_id = std::env::args()
        .nth(2)
        .expect("Please pass the module id as the second parameter");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let service_client =
        ServiceClient::from_connection_string(http_client, iothub_connection_string, 3600)?;
    let module = service_client
        .create_module_identity()
        .device_id(device_id)
        .module_id(module_id)
        .managed_by("IoTEdge")
        .authentication_using_sas(
            "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
            "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk=",
        )
        .execute()
        .await?;

    println!(
        "Successfully created a new module '{}:{}'",
        module.device_id, module.module_id
    );

    println!(
        "Setting status to disabled of module '{}:{}'",
        module.device_id, module.module_id
    );
    let module = service_client
        .update_module_identity(module.etag)
        .device_id(module.device_id)
        .module_id(module.module_id)
        .managed_by("Docker")
        .authentication_using_sas(
            "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
            "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk=",
        )
        .execute()
        .await?;

    println!(
        "Getting module identity of '{}:{}'",
        module.device_id, module.module_id
    );
    let module = service_client
        .get_module_identity(module.device_id, module.module_id)
        .execute()
        .await?;
    println!("Identity is: {:?}", module);

    println!(
        "Deleting module '{}:{}'",
        module.device_id, module.module_id
    );
    service_client
        .delete_module_identity(module.device_id, module.module_id, module.etag)
        .execute()
        .await?;

    Ok(())
}
