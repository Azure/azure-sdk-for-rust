// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_core::http::Transport;
use azure_data_cosmos::CosmosClientOptions;

#[repr(C)]
pub struct ClientOptions {
    /// If true, disables certificate validation. Use only for testing.
    danger_allow_invalid_certificates: bool,
}

/// Converts a C ClientOptions to a Rust CosmosClientOptions.
impl TryFrom<&ClientOptions> for CosmosClientOptions {
    type Error = azure_core::Error;

    fn try_from(value: &ClientOptions) -> Result<Self, Self::Error> {
        if value.danger_allow_invalid_certificates {
            #[cfg(feature = "reqwest")]
            let client = reqwest::ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .map_err(|e| {
                    azure_core::Error::new(
                        azure_core::error::ErrorKind::Other,
                        format!("failed to build reqwest client: {}", e),
                    )
                })?;

            #[cfg(not(feature = "reqwest"))]
            panic!("at least one HTTP transport feature must be enabled");

            let transport = Transport::new(Arc::new(client));
            Ok(Self {
                client_options: azure_core::http::ClientOptions {
                    transport: Some(transport),
                    ..Default::default()
                },
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }
}

#[repr(C)]
pub struct QueryOptions {
    // Placeholder for future query options
}

#[repr(C)]
pub struct CreateDatabaseOptions {
    // Placeholder for future create database options
}

#[repr(C)]
pub struct ReadDatabaseOptions {
    // Placeholder for future read database options
}

#[repr(C)]
pub struct DeleteDatabaseOptions {
    // Placeholder for future delete database options
}

#[repr(C)]
pub struct CreateContainerOptions {
    // Placeholder for future create container options
}

#[repr(C)]
pub struct ReadContainerOptions {
    // Placeholder for future read container options
}

#[repr(C)]
pub struct DeleteContainerOptions {
    // Placeholder for future delete container options
}

#[repr(C)]
pub struct ItemOptions {
    // Placeholder for future item options
}
