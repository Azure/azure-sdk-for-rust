// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// use crate::{
//     clients::GeneratedBlobClient,
//     models::{BlobServiceClientGetPropertiesOptions, StorageServiceProperties},
//     pipeline::StorageHeadersPolicy,
//     BlobClientOptions,
// };
// use azure_core::{
//     credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
// };
// use std::sync::Arc;

// pub struct BlobServiceClient {
//     endpoint: Url,
//     client: GeneratedBlobClient,
// }

// impl BlobServiceClient {
//     pub fn new(
//         endpoint: &str,
//         credential: Arc<dyn TokenCredential>,
//         options: Option<BlobClientOptions>,
//     ) -> Result<Self> {
//         let mut options = options.unwrap_or_default();

//         let storage_headers_policy = Arc::new(StorageHeadersPolicy);
//         options
//             .client_options
//             .per_call_policies
//             .push(storage_headers_policy);

//         let oauth_token_policy = BearerTokenCredentialPolicy::new(
//             credential.clone(),
//             ["https://storage.azure.com/.default"],
//         );
//         options
//             .client_options
//             .per_try_policies
//             .push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);

//         // Generated code needs to be adjusted -- service client trying to get navigation generated client
//         // but the generated client wants to know a container_name (which is not a concept for ServiceClient abstraction)
//         let client =
//             GeneratedBlobClient::new(endpoint, credential, container_name.clone(), Some(options))?;

//         Ok(Self {
//             endpoint: endpoint.parse()?,
//             client,
//         })
//     }

//     pub async fn get_service_properties(
//         &self,
//         options: Option<BlobServiceClientGetPropertiesOptions<'_>>,
//     ) -> Result<Response<StorageServiceProperties>> {
//         let response = self
//             .client
//             .get_blob_service_client()
//             .get_properties(options)
//             .await?;
//         Ok(response)
//     }
// }
