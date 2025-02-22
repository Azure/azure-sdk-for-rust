// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use azure_core::ClientMethodOptions;
use typespec_client_core::fmt::SafeDebug;

/// Options to be passed to [`SecretClient::backup_secret()`](crate::SecretClient::backup_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientBackupSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::delete_secret()`](crate::SecretClient::delete_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientDeleteSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::get_deleted_secret()`](crate::SecretClient::get_deleted_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetDeletedSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::get_deleted_secrets()`](crate::SecretClient::get_deleted_secrets())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetDeletedSecretsOptions<'a> {
    /// Maximum number of results to return in a page. If not specified the service will return up to 25 results.
    pub maxresults: Option<i32>,

    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetDeletedSecretsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetDeletedSecretsOptions<'static> {
        SecretClientGetDeletedSecretsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

/// Options to be passed to [`SecretClient::get_secret()`](crate::SecretClient::get_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::get_secret_versions()`](crate::SecretClient::get_secret_versions())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretVersionsOptions<'a> {
    /// Maximum number of results to return in a page. If not specified the service will return up to 25 results.
    pub maxresults: Option<i32>,

    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetSecretVersionsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetSecretVersionsOptions<'static> {
        SecretClientGetSecretVersionsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

/// Options to be passed to [`SecretClient::get_secrets()`](crate::SecretClient::get_secrets())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientGetSecretsOptions<'a> {
    /// Maximum number of results to return in a page. If not specified the service will return up to 25 results.
    pub maxresults: Option<i32>,

    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

impl SecretClientGetSecretsOptions<'_> {
    pub fn into_owned(self) -> SecretClientGetSecretsOptions<'static> {
        SecretClientGetSecretsOptions {
            maxresults: self.maxresults,
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}

/// Options to be passed to [`SecretClient::purge_deleted_secret()`](crate::SecretClient::purge_deleted_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientPurgeDeletedSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::recover_deleted_secret()`](crate::SecretClient::recover_deleted_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientRecoverDeletedSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::restore_secret()`](crate::SecretClient::restore_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientRestoreSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::set_secret()`](crate::SecretClient::set_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientSetSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`SecretClient::update_secret()`](crate::SecretClient::update_secret())
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientUpdateSecretOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,
}
