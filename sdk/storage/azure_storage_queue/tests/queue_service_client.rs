// use azure_core::{http::ClientOptions, Result};
// use azure_core_test::{recorded, Recording, TestContext};
// use azure_storage_queue::clients::QueueServiceClient;
// use azure_storage_queue::AzureQueueStorageServiceOperationsClientGetPropertiesOptions;
// use std::error::Error;

// #[recorded::test]
// async fn test_create_queue(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     let recording = ctx.recording();
//     let queue_client = get_queue_service_client(recording, false).await?;
// }

// /// Returns an instance of a BlobContainerClient.
// ///
// /// # Arguments
// ///
// /// * `recording` - A reference to a Recording instance.
// /// * `create` - An optional flag to determine whether the container should also be created.
// pub async fn get_queue_service_client(
//     recording: &Recording,
//     create: bool,
// ) -> Result<QueueServiceClient> {
//     let queue_name = get_queue_name(recording);
//     let (options, endpoint) = recorded_test_setup(recording);
//     let queue_client_options = AzureQueueStorageServiceOperationsClientGetPropertiesOptions {
//         method_options: options.clone(),
//         ..Default::default()
//     };
//     let queue_client =
//         QueueServiceClient::new(&endpoint, recording.credential(), queue_client_options)?;
//     if create {
//         queue_client.create_queue(None).await?;
//     }
//     Ok(queue_client)
// }

// /// Takes in a Recording instance and returns a randomized queue name with prefix "queue" of length 16.
// ///
// /// # Arguments
// ///
// /// * `recording` - A reference to a Recording instance.
// pub fn get_queue_name(recording: &Recording) -> String {
//     recording
//         .random_string::<17>(Some("queue"))
//         .to_ascii_lowercase()
// }

// /// Takes in a Recording instance and returns an instrumented options bag and endpoint.
// ///
// /// # Arguments
// ///
// /// * `recording` - A reference to a Recording instance.
// fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
//     let mut client_options = ClientOptions::default();
//     recording.instrument(&mut client_options);
//     let endpoint = format!(
//         "https://{}.queue.core.windows.net/",
//         recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
//     );

//     (client_options, endpoint)
// }
