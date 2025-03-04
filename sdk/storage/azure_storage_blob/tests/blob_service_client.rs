// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// use azure_core_test::recorded;
// use azure_storage_blob::{
//     clients::ServiceClient, models::BlobServiceClientGetPropertiesOptions, BlobClientOptions,
// };
// use azure_storage_blob_test::recorded_test_setup;
// use std::error::Error;

// #[cfg(test)]
// mod tests {
//     use azure_core_test::TestContext;

//     use super::*;

// Need new copy of generated code for this to deserialize properly.
// #[recorded::test]
// async fn test_get_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // Recording Setup
//     let recording = ctx.recording();
//     let (options, endpoint) =
//         recorded_test_setup(recording, BlobClientOptions::default()).await;

//     // Act
//     let service_client = ServiceClient::new(&endpoint, recording.credential(), Some(options))?;
//     let response = service_client
//         .get_service_properties(Some(BlobServiceClientGetPropertiesOptions::default()))
//         .await?;

//     // Assert
//     let storage_service_properties = response.into_body().await?;
//     assert!(storage_service_properties.default_service_version.is_some());
//     Ok(())
// }
// }
