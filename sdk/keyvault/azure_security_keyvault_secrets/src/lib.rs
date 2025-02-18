// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

// BEGIN GENERATED CODE -- do not edit from here till END
mod generated;

pub mod clients {
    pub use crate::generated::clients::{SecretClient, SecretClientOptions};
}

pub mod models {
    pub use crate::generated::clients::method_options::{
        SecretClientBackupSecretOptions, SecretClientDeleteSecretOptions,
        SecretClientGetDeletedSecretOptions, SecretClientGetDeletedSecretsOptions,
        SecretClientGetSecretOptions, SecretClientGetSecretVersionsOptions,
        SecretClientGetSecretsOptions, SecretClientPurgeDeletedSecretOptions,
        SecretClientRecoverDeletedSecretOptions, SecretClientRestoreSecretOptions,
        SecretClientSetSecretOptions, SecretClientUpdateSecretOptions,
    };
    pub use crate::generated::enums::*;
    pub use crate::generated::models::*;
}

pub use crate::generated::clients::{SecretClient, SecretClientOptions};
// END GENERATED CODE

mod resource;
pub use resource::*;
