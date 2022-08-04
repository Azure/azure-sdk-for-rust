use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let configuration_id = std::env::args()
        .nth(1)
        .expect("Please pass the configuration id as the first parameter");

    let service_client = ServiceClient::new_connection_string(iot_hub_connection_string, 3600)?;

    println!("Creating a new configuration with id: {}", configuration_id);

    let configuration = service_client
        .create_configuration(&configuration_id, 10, "tags.environment='test'")
        .device_content(serde_json::json!({
            "properties.desired.settings": {
                "test": "test",
                "otherKey": "otherValue"
            }
        }))
        .label("some", "label")
        .metric(
            "metric1",
            "SELECT deviceId FROM devices WHERE properties.reported.lastDesiredStatus.code = 200",
        )
        .into_future()
        .await?;

    println!(
        "Successfully created a new configuration with id: {}",
        configuration.id,
    );

    println!("Getting configuration: {}", configuration_id);
    let configuration = service_client
        .get_configuration(configuration_id)
        .into_future()
        .await?;

    println!(
        "Successfully retrieved the new configuration '{:?}'",
        configuration
    );

    println!(
        "Updating the newly created configuration with id: {}",
        configuration.id
    );

    let configuration = service_client
        .update_configuration(
            &configuration.id,
            20,
            "tags.environment='debug'",
            &configuration.etag,
        )
        .device_content(serde_json::json!({
            "properties.desired.settings": {
                "test": "test2",
                "otherKey": "otherValue3"
            }
        }))
        .labels(configuration.labels)
        .metrics(configuration.metrics.queries)
        .into_future()
        .await?;

    let multiple_configurations = service_client.get_configurations().into_future().await?;
    println!(
        "Successfully retrieved all configurations '{:?}'",
        multiple_configurations
    );

    println!(
        "Succesfully updated the newly created configuration with id: {}",
        configuration.id
    );

    println!(
        "Deleting newly created configuration with id: {}",
        configuration.id
    );

    service_client
        .delete_configuration(&configuration.id, configuration.etag)
        .into_future()
        .await?;

    println!(
        "Successfully deleted configuration with id: {}",
        configuration.id,
    );
    Ok(())
}
