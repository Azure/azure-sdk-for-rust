// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::CosmosClientOptions;

#[repr(C)]
pub struct ClientOptions {
    /// If true, disables certificate validation. Use only for testing.
    #[cfg(not(target_family = "wasm"))]
    danger_allow_invalid_certificates: bool,
}

/// Converts a C ClientOptions to a Rust CosmosClientOptions.
impl TryFrom<&ClientOptions> for CosmosClientOptions {
    type Error = azure_core::Error;

    #[cfg_attr(
        target_family = "wasm",
        allow(unused_variables, reason = "used in other targets")
    )]
    fn try_from(value: &ClientOptions) -> Result<Self, Self::Error> {
        #[cfg(not(target_family = "wasm"))]
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

            let transport = azure_core::http::Transport::new(std::sync::Arc::new(client));
            let mut options = Self::default();
            options.client_options.transport = Some(transport);
            Ok(options)
        } else {
            Ok(Default::default())
        }

        #[cfg(target_family = "wasm")]
        Ok(Default::default())
    }
}

// Empty structs are not permitted in standard C (though GNU/Clang allow it).
// So, several of these structs have placeholder fields for future expansion.

#[repr(C)]
pub struct QueryOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct CreateDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ReadDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct DeleteDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct CreateContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ReadContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct DeleteContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ItemOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}
