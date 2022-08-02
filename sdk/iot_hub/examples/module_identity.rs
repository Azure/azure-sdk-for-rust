use azure_iot_hub::service::resources::AuthenticationMechanism;
use azure_iot_hub::service::responses::ModuleIdentityResponse;
use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let module_id = std::env::args()
        .nth(2)
        .expect("Please pass the module id as the second parameter");

    let service_client = ServiceClient::new_connection_string(iot_hub_connection_string, 3600)?;
    let module = service_client
        .create_module_identity(
            &device_id,
            &module_id,
            "IoTEdge",
            AuthenticationMechanism::new_using_symmetric_key(
                "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
                "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk",
            ),
        )
        .into_future()
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
        .update_module_identity(
            &device_id,
            &module_id,
            "Docker",
            AuthenticationMechanism::new_using_symmetric_key(
                "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
                "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk",
            ),
            module.etag,
        )
        .into_future()
        .await?;

    println!(
        "Getting module identity of '{}:{}'",
        module.device_id, module.module_id
    );
    let module = service_client
        .get_module_identity(module.device_id, module.module_id)
        .into_future()
        .await?;
    let module: ModuleIdentityResponse = module.try_into()?;
    println!("Identity is: {:?}", module);

    println!(
        "Deleting module '{}:{}'",
        module.device_id, module.module_id
    );
    service_client
        .delete_module_identity(module.device_id, module.module_id, module.etag)
        .into_future()
        .await?;

    Ok(())
}
