// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fake transport and client wrapper for Key Vault secrets tests.
// cspell:ignore skiptoken

use crate::{
    generated::{
        clients::SecretClient as GeneratedSecretClient,
        models::{
            BackupSecretResult, ContentType, DeletedSecret, ListDeletedSecretPropertiesResult,
            ListSecretPropertiesResult, RestoreSecretParameters, Secret,
            SecretClientBackupSecretOptions, SecretClientDeleteSecretOptions,
            SecretClientGetDeletedSecretOptions, SecretClientGetSecretOptions,
            SecretClientListDeletedSecretPropertiesOptions,
            SecretClientListSecretPropertiesOptions,
            SecretClientListSecretPropertiesVersionsOptions, SecretClientPurgeDeletedSecretOptions,
            SecretClientRecoverDeletedSecretOptions, SecretClientRestoreSecretOptions,
            SecretClientSetSecretOptions, SecretClientUpdateSecretPropertiesOptions,
            SetSecretParameters, UpdateSecretPropertiesParameters,
        },
    },
    SecretClientOptions,
};
use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    http::{
        fake::ApiName,
        pager::{Pager, PagerOptions, PagerState},
        policies::{Policy, PolicyResult},
        AsyncRawResponse, Body, ClientMethodOptions, Context, Method, NoFormat, Pipeline, Request,
        RequestContent, Response, Transport, Url,
    },
    json,
    time::{Duration, OffsetDateTime},
    Error, Result,
};
use futures::io::AsyncReadExt as _;
use percent_encoding::percent_decode_str;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt, sync::Arc};

const API_BACKUP_SECRET: &str = "KeyVault.backupSecret";
const API_DELETE_SECRET: &str = "KeyVault.deleteSecret";
const API_GET_DELETED_SECRET: &str = "KeyVault.getDeletedSecret";
const API_GET_SECRET: &str = "KeyVault.getSecret";
const API_LIST_DELETED_SECRET_PROPERTIES: &str = "KeyVault.getDeletedSecrets";
const API_LIST_SECRET_PROPERTIES: &str = "KeyVault.getSecrets";
const API_LIST_SECRET_PROPERTIES_VERSIONS: &str = "KeyVault.getSecretVersions";
const API_PURGE_DELETED_SECRET: &str = "KeyVault.purgeDeletedSecret";
const API_RECOVER_DELETED_SECRET: &str = "KeyVault.recoverDeletedSecret";
const API_RESTORE_SECRET: &str = "KeyVault.restoreSecret";
const API_SET_SECRET: &str = "KeyVault.setSecret";
const API_UPDATE_SECRET_PROPERTIES: &str = "KeyVault.updateSecret";

#[derive(Clone, Debug)]
struct ListDeletedSecretPropertiesContext {
    maxresults: Option<i32>,
}

#[derive(Clone, Debug)]
struct ListSecretPropertiesContext {
    maxresults: Option<i32>,
}

#[derive(Clone, Debug)]
struct ListSecretPropertiesVersionsContext {
    secret_name: String,
    maxresults: Option<i32>,
}

/// Fake Key Vault secrets server API used by [`SecretClient::new_fake`].
///
/// Implement only the operations your test exercises.
///
/// # Panics
///
/// Default method implementations call [`unimplemented!()`] and panic when invoked.
#[async_trait]
pub trait SecretServer: Send + Sync {
    /// Handles [`SecretClient::backup_secret`].
    async fn backup_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientBackupSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::backup_secret is not implemented")
    }

    /// Handles [`SecretClient::delete_secret`].
    async fn delete_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientDeleteSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::delete_secret is not implemented")
    }

    /// Handles [`SecretClient::get_deleted_secret`].
    async fn get_deleted_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientGetDeletedSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::get_deleted_secret is not implemented")
    }

    /// Handles [`SecretClient::get_secret`].
    async fn get_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientGetSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::get_secret is not implemented")
    }

    /// Handles [`SecretClient::list_deleted_secret_properties`].
    async fn list_deleted_secret_properties(
        &self,
        _state: PagerState,
        _options: Option<SecretClientListDeletedSecretPropertiesOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::list_deleted_secret_properties is not implemented")
    }

    /// Handles [`SecretClient::list_secret_properties`].
    async fn list_secret_properties(
        &self,
        _state: PagerState,
        _options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::list_secret_properties is not implemented")
    }

    /// Handles [`SecretClient::list_secret_properties_versions`].
    async fn list_secret_properties_versions(
        &self,
        _secret_name: &str,
        _state: PagerState,
        _options: Option<SecretClientListSecretPropertiesVersionsOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::list_secret_properties_versions is not implemented")
    }

    /// Handles [`SecretClient::purge_deleted_secret`].
    async fn purge_deleted_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientPurgeDeletedSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::purge_deleted_secret is not implemented")
    }

    /// Handles [`SecretClient::recover_deleted_secret`].
    async fn recover_deleted_secret(
        &self,
        _secret_name: &str,
        _options: Option<SecretClientRecoverDeletedSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::recover_deleted_secret is not implemented")
    }

    /// Handles [`SecretClient::restore_secret`].
    async fn restore_secret(
        &self,
        _parameters: RequestContent<RestoreSecretParameters>,
        _options: Option<SecretClientRestoreSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::restore_secret is not implemented")
    }

    /// Handles [`SecretClient::set_secret`].
    async fn set_secret(
        &self,
        _secret_name: &str,
        _parameters: RequestContent<SetSecretParameters>,
        _options: Option<SecretClientSetSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::set_secret is not implemented")
    }

    /// Handles [`SecretClient::update_secret_properties`].
    async fn update_secret_properties(
        &self,
        _secret_name: &str,
        _parameters: RequestContent<UpdateSecretPropertiesParameters>,
        _options: Option<SecretClientUpdateSecretPropertiesOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!("SecretServer::update_secret_properties is not implemented")
    }
}

/// Adapter policy that dispatches requests to a [`SecretServer`].
pub struct SecretServerTransport<T: SecretServer + ?Sized> {
    server: Arc<T>,
    endpoint_base_path: Vec<String>,
}

impl<T: SecretServer + ?Sized> SecretServerTransport<T> {
    /// Creates a new adapter from a fake server implementation.
    pub fn new(server: Arc<T>) -> Self {
        Self {
            server,
            endpoint_base_path: Vec::new(),
        }
    }

    fn with_endpoint_base_path(server: Arc<T>, endpoint_base_path: Vec<String>) -> Self {
        Self {
            server,
            endpoint_base_path,
        }
    }
}

impl<T: SecretServer + ?Sized> fmt::Debug for SecretServerTransport<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretServerTransport { .. }")
    }
}

#[async_trait]
impl<T: SecretServer + ?Sized> Policy for SecretServerTransport<T> {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        _next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let api_name = ctx
            .value::<ApiName>()
            .ok_or_else(|| fake_error("unknown", "missing ApiName in request context"))?;

        match api_name.as_ref() {
            API_BACKUP_SECRET => {
                expect_method(request, Method::Post, API_BACKUP_SECRET)?;
                let query = parse_query(request.url(), API_BACKUP_SECRET)?;
                expect_query(&query, &["api-version"], API_BACKUP_SECRET)?;
                let secret_name = parse_three_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "secrets",
                    "backup",
                    API_BACKUP_SECRET,
                )?;
                self.server
                    .backup_secret(
                        &secret_name,
                        Some(SecretClientBackupSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_DELETE_SECRET => {
                expect_method(request, Method::Delete, API_DELETE_SECRET)?;
                let query = parse_query(request.url(), API_DELETE_SECRET)?;
                expect_query(&query, &["api-version"], API_DELETE_SECRET)?;
                let secret_name = parse_two_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "secrets",
                    API_DELETE_SECRET,
                )?;
                self.server
                    .delete_secret(
                        &secret_name,
                        Some(SecretClientDeleteSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_GET_DELETED_SECRET => {
                expect_method(request, Method::Get, API_GET_DELETED_SECRET)?;
                let query = parse_query(request.url(), API_GET_DELETED_SECRET)?;
                expect_query(&query, &["api-version"], API_GET_DELETED_SECRET)?;
                let secret_name = parse_two_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "deletedsecrets",
                    API_GET_DELETED_SECRET,
                )?;
                self.server
                    .get_deleted_secret(
                        &secret_name,
                        Some(SecretClientGetDeletedSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_GET_SECRET => {
                expect_method(request, Method::Get, API_GET_SECRET)?;
                let query = parse_query(request.url(), API_GET_SECRET)?;
                expect_query(&query, &["api-version", "outContentType"], API_GET_SECRET)?;
                let (secret_name, secret_version) = parse_secret_with_optional_version(
                    request.url(),
                    &self.endpoint_base_path,
                    API_GET_SECRET,
                )?;
                let out_content_type = query
                    .get("outContentType")
                    .map(|value| parse_content_type(value.as_str()));
                self.server
                    .get_secret(
                        &secret_name,
                        Some(SecretClientGetSecretOptions {
                            method_options: method_options(ctx),
                            out_content_type,
                            secret_version,
                        }),
                    )
                    .await
            }
            API_LIST_DELETED_SECRET_PROPERTIES => {
                expect_method(request, Method::Get, API_LIST_DELETED_SECRET_PROPERTIES)?;
                let state = pager_state(ctx, API_LIST_DELETED_SECRET_PROPERTIES)?;
                let maxresults = match &state {
                    PagerState::Initial => {
                        let query = parse_query(request.url(), API_LIST_DELETED_SECRET_PROPERTIES)?;
                        expect_query(
                            &query,
                            &["api-version", "maxresults", "skiptoken"],
                            API_LIST_DELETED_SECRET_PROPERTIES,
                        )?;
                        expect_collection_path(
                            request.url(),
                            &self.endpoint_base_path,
                            "deletedsecrets",
                            API_LIST_DELETED_SECRET_PROPERTIES,
                        )?;
                        parse_maxresults(&query, API_LIST_DELETED_SECRET_PROPERTIES)?
                    }
                    PagerState::More(_) => ctx
                        .value::<ListDeletedSecretPropertiesContext>()
                        .map(|value| value.maxresults)
                        .unwrap_or_default(),
                };
                self.server
                    .list_deleted_secret_properties(
                        state.clone(),
                        Some(SecretClientListDeletedSecretPropertiesOptions {
                            maxresults,
                            method_options: pager_options(ctx, &state),
                        }),
                    )
                    .await
            }
            API_LIST_SECRET_PROPERTIES => {
                expect_method(request, Method::Get, API_LIST_SECRET_PROPERTIES)?;
                let state = pager_state(ctx, API_LIST_SECRET_PROPERTIES)?;
                let maxresults = match &state {
                    PagerState::Initial => {
                        let query = parse_query(request.url(), API_LIST_SECRET_PROPERTIES)?;
                        expect_query(
                            &query,
                            &["api-version", "maxresults", "skiptoken"],
                            API_LIST_SECRET_PROPERTIES,
                        )?;
                        expect_collection_path(
                            request.url(),
                            &self.endpoint_base_path,
                            "secrets",
                            API_LIST_SECRET_PROPERTIES,
                        )?;
                        parse_maxresults(&query, API_LIST_SECRET_PROPERTIES)?
                    }
                    PagerState::More(_) => ctx
                        .value::<ListSecretPropertiesContext>()
                        .map(|value| value.maxresults)
                        .unwrap_or_default(),
                };
                self.server
                    .list_secret_properties(
                        state.clone(),
                        Some(SecretClientListSecretPropertiesOptions {
                            maxresults,
                            method_options: pager_options(ctx, &state),
                        }),
                    )
                    .await
            }
            API_LIST_SECRET_PROPERTIES_VERSIONS => {
                expect_method(request, Method::Get, API_LIST_SECRET_PROPERTIES_VERSIONS)?;
                let state = pager_state(ctx, API_LIST_SECRET_PROPERTIES_VERSIONS)?;
                let (secret_name, maxresults) = match &state {
                    PagerState::Initial => {
                        let query =
                            parse_query(request.url(), API_LIST_SECRET_PROPERTIES_VERSIONS)?;
                        expect_query(
                            &query,
                            &["api-version", "maxresults", "skiptoken"],
                            API_LIST_SECRET_PROPERTIES_VERSIONS,
                        )?;
                        let secret_name = parse_three_segment_path(
                            request.url(),
                            &self.endpoint_base_path,
                            "secrets",
                            "versions",
                            API_LIST_SECRET_PROPERTIES_VERSIONS,
                        )?;
                        let maxresults =
                            parse_maxresults(&query, API_LIST_SECRET_PROPERTIES_VERSIONS)?;
                        (secret_name, maxresults)
                    }
                    PagerState::More(_) => {
                        let context = ctx
                            .value::<ListSecretPropertiesVersionsContext>()
                            .ok_or_else(|| {
                                fake_error(
                                    API_LIST_SECRET_PROPERTIES_VERSIONS,
                                    "missing list secret versions context in request context",
                                )
                            })?;
                        (context.secret_name.clone(), context.maxresults)
                    }
                };
                self.server
                    .list_secret_properties_versions(
                        &secret_name,
                        state.clone(),
                        Some(SecretClientListSecretPropertiesVersionsOptions {
                            maxresults,
                            method_options: pager_options(ctx, &state),
                        }),
                    )
                    .await
            }
            API_PURGE_DELETED_SECRET => {
                expect_method(request, Method::Delete, API_PURGE_DELETED_SECRET)?;
                let query = parse_query(request.url(), API_PURGE_DELETED_SECRET)?;
                expect_query(&query, &["api-version"], API_PURGE_DELETED_SECRET)?;
                let secret_name = parse_two_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "deletedsecrets",
                    API_PURGE_DELETED_SECRET,
                )?;
                self.server
                    .purge_deleted_secret(
                        &secret_name,
                        Some(SecretClientPurgeDeletedSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_RECOVER_DELETED_SECRET => {
                expect_method(request, Method::Post, API_RECOVER_DELETED_SECRET)?;
                let query = parse_query(request.url(), API_RECOVER_DELETED_SECRET)?;
                expect_query(&query, &["api-version"], API_RECOVER_DELETED_SECRET)?;
                let secret_name = parse_three_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "deletedsecrets",
                    "recover",
                    API_RECOVER_DELETED_SECRET,
                )?;
                self.server
                    .recover_deleted_secret(
                        &secret_name,
                        Some(SecretClientRecoverDeletedSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_RESTORE_SECRET => {
                expect_method(request, Method::Post, API_RESTORE_SECRET)?;
                let query = parse_query(request.url(), API_RESTORE_SECRET)?;
                expect_query(&query, &["api-version"], API_RESTORE_SECRET)?;
                expect_two_segment_literal_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "secrets",
                    "restore",
                    API_RESTORE_SECRET,
                )?;
                let parameters =
                    parse_request_content::<RestoreSecretParameters>(request, API_RESTORE_SECRET)
                        .await?;
                self.server
                    .restore_secret(
                        parameters,
                        Some(SecretClientRestoreSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_SET_SECRET => {
                expect_method(request, Method::Put, API_SET_SECRET)?;
                let query = parse_query(request.url(), API_SET_SECRET)?;
                expect_query(&query, &["api-version"], API_SET_SECRET)?;
                let secret_name = parse_two_segment_path(
                    request.url(),
                    &self.endpoint_base_path,
                    "secrets",
                    API_SET_SECRET,
                )?;
                let parameters =
                    parse_request_content::<SetSecretParameters>(request, API_SET_SECRET).await?;
                self.server
                    .set_secret(
                        &secret_name,
                        parameters,
                        Some(SecretClientSetSecretOptions {
                            method_options: method_options(ctx),
                        }),
                    )
                    .await
            }
            API_UPDATE_SECRET_PROPERTIES => {
                expect_method(request, Method::Patch, API_UPDATE_SECRET_PROPERTIES)?;
                let query = parse_query(request.url(), API_UPDATE_SECRET_PROPERTIES)?;
                expect_query(&query, &["api-version"], API_UPDATE_SECRET_PROPERTIES)?;
                let (secret_name, secret_version) = parse_secret_with_optional_version(
                    request.url(),
                    &self.endpoint_base_path,
                    API_UPDATE_SECRET_PROPERTIES,
                )?;
                let parameters = parse_request_content::<UpdateSecretPropertiesParameters>(
                    request,
                    API_UPDATE_SECRET_PROPERTIES,
                )
                .await?;
                self.server
                    .update_secret_properties(
                        &secret_name,
                        parameters,
                        Some(SecretClientUpdateSecretPropertiesOptions {
                            method_options: method_options(ctx),
                            secret_version,
                        }),
                    )
                    .await
            }
            other => Err(fake_error(
                "unknown",
                format!("unknown ApiName '{other}' in request context"),
            )),
        }
    }
}

/// A fake-aware `SecretClient` that forwards to the generated client.
pub struct SecretClient {
    inner: GeneratedSecretClient,
}

impl SecretClient {
    /// Creates a new `SecretClient`, using Entra ID authentication.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        options: Option<SecretClientOptions>,
    ) -> Result<Self> {
        Ok(Self {
            inner: GeneratedSecretClient::new(endpoint, credential, options)?,
        })
    }

    /// Creates a new `SecretClient` backed by a fake server.
    pub fn new_fake<T>(
        endpoint: &str,
        server: Arc<T>,
        options: Option<SecretClientOptions>,
    ) -> Result<Self>
    where
        T: SecretServer + ?Sized + 'static,
    {
        let options = options.unwrap_or_default();
        let endpoint_base_path = parse_endpoint_base_path(endpoint)?;
        let generated_client =
            GeneratedSecretClient::new(endpoint, Arc::new(FakeCredential), Some(options.clone()))?;

        let mut client_options = options.client_options;
        client_options.transport = Some(Transport::with_policy(Arc::new(
            SecretServerTransport::with_endpoint_base_path(server, endpoint_base_path),
        )));

        Ok(Self {
            inner: GeneratedSecretClient {
                api_version: options.api_version,
                pipeline: Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    client_options,
                    Vec::new(),
                    Vec::new(),
                    None,
                ),
                ..generated_client
            },
        })
    }

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        self.inner.endpoint()
    }

    /// Backs up the specified secret.
    pub async fn backup_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientBackupSecretOptions<'_>>,
    ) -> Result<Response<BackupSecretResult>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_BACKUP_SECRET);
        self.inner.backup_secret(secret_name, Some(options)).await
    }

    /// Deletes a secret from a specified key vault.
    pub async fn delete_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientDeleteSecretOptions<'_>>,
    ) -> Result<Response<DeletedSecret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_DELETE_SECRET);
        self.inner.delete_secret(secret_name, Some(options)).await
    }

    /// Gets the specified deleted secret.
    pub async fn get_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientGetDeletedSecretOptions<'_>>,
    ) -> Result<Response<DeletedSecret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_GET_DELETED_SECRET);
        self.inner
            .get_deleted_secret(secret_name, Some(options))
            .await
    }

    /// Gets a specified secret from a given key vault.
    pub async fn get_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientGetSecretOptions<'_>>,
    ) -> Result<Response<Secret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_GET_SECRET);
        self.inner.get_secret(secret_name, Some(options)).await
    }

    /// Lists deleted secrets for the specified vault.
    pub fn list_deleted_secret_properties(
        &self,
        options: Option<SecretClientListDeletedSecretPropertiesOptions<'_>>,
    ) -> Result<Pager<ListDeletedSecretPropertiesResult>> {
        let mut options = options.unwrap_or_default();
        options
            .method_options
            .context
            .insert(ListDeletedSecretPropertiesContext {
                maxresults: options.maxresults,
            });
        insert_api_name(
            &mut options.method_options.context,
            API_LIST_DELETED_SECRET_PROPERTIES,
        );
        self.inner.list_deleted_secret_properties(Some(options))
    }

    /// Lists secrets in a specified vault.
    pub fn list_secret_properties(
        &self,
        options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<Pager<ListSecretPropertiesResult>> {
        let mut options = options.unwrap_or_default();
        options
            .method_options
            .context
            .insert(ListSecretPropertiesContext {
                maxresults: options.maxresults,
            });
        insert_api_name(
            &mut options.method_options.context,
            API_LIST_SECRET_PROPERTIES,
        );
        self.inner.list_secret_properties(Some(options))
    }

    /// Lists all versions of a specified secret.
    pub fn list_secret_properties_versions(
        &self,
        secret_name: &str,
        options: Option<SecretClientListSecretPropertiesVersionsOptions<'_>>,
    ) -> Result<Pager<ListSecretPropertiesResult>> {
        let mut options = options.unwrap_or_default();
        options
            .method_options
            .context
            .insert(ListSecretPropertiesVersionsContext {
                secret_name: secret_name.to_owned(),
                maxresults: options.maxresults,
            });
        insert_api_name(
            &mut options.method_options.context,
            API_LIST_SECRET_PROPERTIES_VERSIONS,
        );
        self.inner
            .list_secret_properties_versions(secret_name, Some(options))
    }

    /// Permanently deletes the specified secret.
    pub async fn purge_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientPurgeDeletedSecretOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(
            &mut options.method_options.context,
            API_PURGE_DELETED_SECRET,
        );
        self.inner
            .purge_deleted_secret(secret_name, Some(options))
            .await
    }

    /// Recovers the deleted secret to the latest version.
    pub async fn recover_deleted_secret(
        &self,
        secret_name: &str,
        options: Option<SecretClientRecoverDeletedSecretOptions<'_>>,
    ) -> Result<Response<Secret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(
            &mut options.method_options.context,
            API_RECOVER_DELETED_SECRET,
        );
        self.inner
            .recover_deleted_secret(secret_name, Some(options))
            .await
    }

    /// Restores a backed up secret to a vault.
    pub async fn restore_secret(
        &self,
        parameters: RequestContent<RestoreSecretParameters>,
        options: Option<SecretClientRestoreSecretOptions<'_>>,
    ) -> Result<Response<Secret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_RESTORE_SECRET);
        self.inner.restore_secret(parameters, Some(options)).await
    }

    /// Sets a secret in a specified key vault.
    pub async fn set_secret(
        &self,
        secret_name: &str,
        parameters: RequestContent<SetSecretParameters>,
        options: Option<SecretClientSetSecretOptions<'_>>,
    ) -> Result<Response<Secret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(&mut options.method_options.context, API_SET_SECRET);
        self.inner
            .set_secret(secret_name, parameters, Some(options))
            .await
    }

    /// Updates the attributes associated with a specified secret.
    pub async fn update_secret_properties(
        &self,
        secret_name: &str,
        parameters: RequestContent<UpdateSecretPropertiesParameters>,
        options: Option<SecretClientUpdateSecretPropertiesOptions<'_>>,
    ) -> Result<Response<Secret>> {
        let mut options = options.unwrap_or_default();
        insert_api_name(
            &mut options.method_options.context,
            API_UPDATE_SECRET_PROPERTIES,
        );
        self.inner
            .update_secret_properties(secret_name, parameters, Some(options))
            .await
    }
}

fn insert_api_name(context: &mut Context<'_>, api_name: &'static str) {
    context.insert(ApiName(api_name.to_string()));
}

fn method_options(ctx: &Context<'_>) -> ClientMethodOptions<'static> {
    ClientMethodOptions {
        context: ctx.to_owned(),
    }
}

fn pager_options(ctx: &Context<'_>, state: &PagerState) -> PagerOptions<'static> {
    PagerOptions {
        context: ctx.to_owned(),
        continuation: match state {
            PagerState::Initial => None,
            PagerState::More(continuation) => Some(continuation.clone()),
        },
    }
}

fn pager_state(ctx: &Context<'_>, operation: &'static str) -> Result<PagerState> {
    ctx.value::<PagerState>()
        .cloned()
        .ok_or_else(|| fake_error(operation, "missing PagerState in request context"))
}

async fn parse_request_content<T: DeserializeOwned>(
    request: &Request,
    operation: &'static str,
) -> Result<RequestContent<T>> {
    let body = body_bytes(request, operation).await?;
    json::from_json::<_, T>(&body).map_err(|err| {
        Error::with_message(
            ErrorKind::DataConversion,
            format!("{operation}: failed to deserialize request body: {err}"),
        )
    })?;
    Ok(RequestContent::from(body.to_vec()))
}

async fn body_bytes(request: &Request, operation: &'static str) -> Result<azure_core::Bytes> {
    match request.body() {
        Body::Bytes(bytes) => Ok(bytes.clone()),
        Body::SeekableStream(stream) => {
            let mut stream = stream.clone();
            stream.reset().await.map_err(|err| {
                Error::with_error(
                    ErrorKind::Io,
                    err,
                    format!("{operation}: failed to reset stream body"),
                )
            })?;
            let mut bytes = Vec::new();
            stream.read_to_end(&mut bytes).await.map_err(|err| {
                Error::with_error(
                    ErrorKind::Io,
                    err,
                    format!("{operation}: failed to read stream body"),
                )
            })?;
            Ok(bytes.into())
        }
    }
}

fn parse_query(url: &Url, operation: &'static str) -> Result<HashMap<String, String>> {
    let mut query = HashMap::new();
    for (name, value) in url.query_pairs() {
        let name = name.to_string();
        if query.insert(name.clone(), value.to_string()).is_some() {
            return Err(fake_error(
                operation,
                format!("duplicate query parameter '{name}'"),
            ));
        }
    }
    Ok(query)
}

fn expect_query(
    query: &HashMap<String, String>,
    allowed: &[&str],
    operation: &'static str,
) -> Result<()> {
    for name in query.keys() {
        if !allowed.contains(&name.as_str()) {
            return Err(fake_error(
                operation,
                format!("unexpected query parameter '{name}'"),
            ));
        }
    }
    match query.get("api-version") {
        Some(version) if !version.is_empty() => Ok(()),
        _ => Err(fake_error(
            operation,
            "missing required query parameter 'api-version'",
        )),
    }
}

fn parse_maxresults(
    query: &HashMap<String, String>,
    operation: &'static str,
) -> Result<Option<i32>> {
    query
        .get("maxresults")
        .map(|value| {
            value.parse::<i32>().map_err(|err| {
                fake_error(
                    operation,
                    format!("invalid maxresults value '{value}': {err}"),
                )
            })
        })
        .transpose()
}

fn parse_content_type(value: &str) -> ContentType {
    match value.parse::<ContentType>() {
        Ok(content_type) => content_type,
        Err(infallible) => match infallible {},
    }
}

fn expect_method(request: &Request, expected: Method, operation: &'static str) -> Result<()> {
    if request.method() != expected {
        return Err(fake_error(
            operation,
            format!(
                "expected HTTP method {:?}, found {:?}",
                expected,
                request.method()
            ),
        ));
    }
    Ok(())
}

fn parse_segments(url: &Url) -> Vec<&str> {
    let path = url.path().trim_start_matches('/');
    if path.is_empty() {
        Vec::new()
    } else {
        path.split('/').collect()
    }
}

fn parse_relative_segments<'a>(
    url: &'a Url,
    endpoint_base_path: &[String],
    operation: &'static str,
) -> Result<Vec<&'a str>> {
    let segments = parse_segments(url);
    if endpoint_base_path.is_empty() {
        return Ok(segments);
    }

    if segments.len() < endpoint_base_path.len() {
        return Err(fake_error(
            operation,
            format!("unexpected path '{}' for configured endpoint", url.path()),
        ));
    }

    for (actual, expected) in segments.iter().zip(endpoint_base_path) {
        if *actual != expected {
            return Err(fake_error(
                operation,
                format!("unexpected path '{}' for configured endpoint", url.path()),
            ));
        }
    }

    Ok(segments[endpoint_base_path.len()..].to_vec())
}

fn decode_path_segment(
    segment: &str,
    segment_name: &str,
    operation: &'static str,
) -> Result<String> {
    percent_decode_str(segment)
        .decode_utf8()
        .map(|decoded| decoded.into_owned())
        .map_err(|err| {
            fake_error(
                operation,
                format!("failed to decode {segment_name} path segment '{segment}': {err}"),
            )
        })
}

fn expect_collection_path(
    url: &Url,
    endpoint_base_path: &[String],
    collection: &str,
    operation: &'static str,
) -> Result<()> {
    let segments = parse_relative_segments(url, endpoint_base_path, operation)?;
    match segments.as_slice() {
        [segment] | [segment, ""] if *segment == collection => Ok(()),
        _ => Err(fake_error(
            operation,
            format!("unexpected path '{}'", url.path()),
        )),
    }
}

fn parse_two_segment_path(
    url: &Url,
    endpoint_base_path: &[String],
    collection: &str,
    operation: &'static str,
) -> Result<String> {
    let segments = parse_relative_segments(url, endpoint_base_path, operation)?;
    match segments.as_slice() {
        [actual_collection, name] if *actual_collection == collection && !name.is_empty() => {
            decode_path_segment(name, "name", operation)
        }
        _ => Err(fake_error(
            operation,
            format!("unexpected path '{}'", url.path()),
        )),
    }
}

fn parse_three_segment_path(
    url: &Url,
    endpoint_base_path: &[String],
    collection: &str,
    action: &str,
    operation: &'static str,
) -> Result<String> {
    let segments = parse_relative_segments(url, endpoint_base_path, operation)?;
    match segments.as_slice() {
        [actual_collection, name, actual_action]
            if *actual_collection == collection && *actual_action == action && !name.is_empty() =>
        {
            decode_path_segment(name, "name", operation)
        }
        _ => Err(fake_error(
            operation,
            format!("unexpected path '{}'", url.path()),
        )),
    }
}

fn expect_two_segment_literal_path(
    url: &Url,
    endpoint_base_path: &[String],
    segment0: &str,
    segment1: &str,
    operation: &'static str,
) -> Result<()> {
    let segments = parse_relative_segments(url, endpoint_base_path, operation)?;
    if matches!(segments.as_slice(), [actual0, actual1] if *actual0 == segment0 && *actual1 == segment1)
    {
        return Ok(());
    }
    Err(fake_error(
        operation,
        format!("unexpected path '{}'", url.path()),
    ))
}

fn parse_secret_with_optional_version(
    url: &Url,
    endpoint_base_path: &[String],
    operation: &'static str,
) -> Result<(String, Option<String>)> {
    let segments = parse_relative_segments(url, endpoint_base_path, operation)?;
    match segments.as_slice() {
        ["secrets", secret_name] if !secret_name.is_empty() => Ok((
            decode_path_segment(secret_name, "secret_name", operation)?,
            None,
        )),
        ["secrets", secret_name, ""] if !secret_name.is_empty() => Ok((
            decode_path_segment(secret_name, "secret_name", operation)?,
            None,
        )),
        ["secrets", secret_name, secret_version]
            if !secret_name.is_empty() && !secret_version.is_empty() =>
        {
            Ok((
                decode_path_segment(secret_name, "secret_name", operation)?,
                Some(decode_path_segment(
                    secret_version,
                    "secret_version",
                    operation,
                )?),
            ))
        }
        _ => Err(fake_error(
            operation,
            format!("unexpected path '{}'", url.path()),
        )),
    }
}

fn parse_endpoint_base_path(endpoint: &str) -> Result<Vec<String>> {
    let endpoint = Url::parse(endpoint).map_err(|err| {
        Error::with_message(
            ErrorKind::DataConversion,
            format!("invalid endpoint '{endpoint}': {err}"),
        )
    })?;
    Ok(parse_segments(&endpoint)
        .into_iter()
        .filter(|segment| !segment.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn fake_error(operation: &'static str, message: impl Into<String>) -> Error {
    Error::with_message(ErrorKind::Other, format!("{operation}: {}", message.into()))
}

#[derive(Debug)]
struct FakeCredential;

#[async_trait]
impl TokenCredential for FakeCredential {
    async fn get_token(
        &self,
        _scopes: &[&str],
        _options: Option<TokenRequestOptions<'_>>,
    ) -> Result<AccessToken> {
        Ok(AccessToken::new(
            "fake_token",
            OffsetDateTime::now_utc() + Duration::hours(1),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{
        fake::json_response, headers::Headers, pager::PagerContinuation, StatusCode,
    };
    use futures::TryStreamExt;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Default)]
    struct GetSecretServer {
        contexts: Mutex<Vec<Context<'static>>>,
    }

    #[async_trait]
    impl SecretServer for GetSecretServer {
        async fn get_secret(
            &self,
            secret_name: &str,
            options: Option<SecretClientGetSecretOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            assert_eq!(secret_name, "my-secret");
            let options = options.expect("expected options");
            assert!(options.secret_version.is_none());
            assert!(options.out_content_type.is_none());
            self.contexts
                .lock()
                .expect("context lock")
                .push(options.method_options.context.to_owned());
            json_response(
                StatusCode::Ok,
                Headers::new(),
                &Secret {
                    value: Some("fake value".to_string()),
                    ..Default::default()
                },
            )
        }
    }

    #[tokio::test]
    async fn new_fake_routes_get_secret_and_preserves_context() -> Result<()> {
        let server = Arc::new(GetSecretServer::default());
        let client =
            SecretClient::new_fake("https://example.vault.azure.net", server.clone(), None)?;
        let options = SecretClientGetSecretOptions {
            method_options: ClientMethodOptions {
                context: Context::new().with_value(42u32),
            },
            ..Default::default()
        };
        let secret = client
            .get_secret("my-secret", Some(options))
            .await?
            .into_model()?;
        assert_eq!(secret.value.as_deref(), Some("fake value"));

        let contexts = server.contexts.lock().expect("context lock");
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].value::<u32>(), Some(&42u32));
        assert_eq!(
            contexts[0].value::<ApiName>().map(|name| name.as_ref()),
            Some(API_GET_SECRET)
        );
        Ok(())
    }

    #[tokio::test]
    async fn new_fake_routes_get_secret_with_endpoint_prefix() -> Result<()> {
        let server = Arc::new(GetSecretServer::default());
        let client = SecretClient::new_fake(
            "https://example.vault.azure.net/prefix",
            server.clone(),
            None,
        )?;
        let secret = client.get_secret("my-secret", None).await?.into_model()?;
        assert_eq!(secret.value.as_deref(), Some("fake value"));
        Ok(())
    }

    #[derive(Debug, Default)]
    struct EncodedSecretPathServer {
        calls: Mutex<Vec<(String, Option<String>)>>,
    }

    #[async_trait]
    impl SecretServer for EncodedSecretPathServer {
        async fn get_secret(
            &self,
            secret_name: &str,
            options: Option<SecretClientGetSecretOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            let options = options.expect("expected options");
            self.calls
                .lock()
                .expect("calls lock")
                .push((secret_name.to_owned(), options.secret_version.clone()));
            json_response(
                StatusCode::Ok,
                Headers::new(),
                &Secret {
                    value: Some("fake value".to_string()),
                    ..Default::default()
                },
            )
        }
    }

    #[tokio::test]
    async fn decodes_percent_encoded_secret_name_and_version() -> Result<()> {
        let server = Arc::new(EncodedSecretPathServer::default());
        let client =
            SecretClient::new_fake("https://example.vault.azure.net", server.clone(), None)?;

        let secret_name = "folder/secret name+plus";
        let secret_version = "ver/1 +";
        let options = SecretClientGetSecretOptions {
            secret_version: Some(secret_version.to_string()),
            ..Default::default()
        };
        let _ = client
            .get_secret(secret_name, Some(options))
            .await?
            .into_model()?;

        let calls = server.calls.lock().expect("calls lock");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, secret_name);
        assert_eq!(calls[0].1.as_deref(), Some(secret_version));
        Ok(())
    }

    #[derive(Debug, Default)]
    struct SetSecretServer;

    #[async_trait]
    impl SecretServer for SetSecretServer {
        async fn set_secret(
            &self,
            secret_name: &str,
            parameters: RequestContent<SetSecretParameters>,
            _options: Option<SecretClientSetSecretOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            assert_eq!(secret_name, "serialized");
            let body = match parameters.body() {
                Body::Bytes(bytes) => bytes.clone(),
                Body::SeekableStream(_) => panic!("expected buffered bytes"),
            };
            let model: SetSecretParameters = json::from_json(&body)?;
            assert_eq!(model.value.as_deref(), Some("body-value"));
            json_response(
                StatusCode::Ok,
                Headers::new(),
                &Secret {
                    value: model.value,
                    ..Default::default()
                },
            )
        }
    }

    #[tokio::test]
    async fn parses_set_secret_body_back_to_request_content() -> Result<()> {
        let client = SecretClient::new_fake(
            "https://example.vault.azure.net",
            Arc::new(SetSecretServer),
            None,
        )?;
        let secret = client
            .set_secret(
                "serialized",
                RequestContent::from_str(r#"{"value":"body-value"}"#),
                None,
            )
            .await?
            .into_model()?;
        assert_eq!(secret.value.as_deref(), Some("body-value"));
        Ok(())
    }

    #[tokio::test]
    async fn parses_set_secret_stream_body_back_to_request_content() -> Result<()> {
        let client = SecretClient::new_fake(
            "https://example.vault.azure.net",
            Arc::new(SetSecretServer),
            None,
        )?;
        let data = br#"{"value":"body-value"}"#.to_vec();
        let content = RequestContent::from_reader(
            futures::io::Cursor::new(data.clone()),
            Some(data.len() as u64),
        );
        let secret = client
            .set_secret("serialized", content, None)
            .await?
            .into_model()?;
        assert_eq!(secret.value.as_deref(), Some("body-value"));
        Ok(())
    }

    #[derive(Debug, Default)]
    struct ListSecretsServer {
        states: Mutex<Vec<PagerState>>,
    }

    #[async_trait]
    impl SecretServer for ListSecretsServer {
        async fn list_secret_properties(
            &self,
            state: PagerState,
            options: Option<SecretClientListSecretPropertiesOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            let options = options.expect("expected options");
            assert_eq!(
                options.method_options.context.value::<PagerState>(),
                Some(&state)
            );
            assert_eq!(
                options
                    .method_options
                    .context
                    .value::<ApiName>()
                    .map(|name| name.as_ref()),
                Some(API_LIST_SECRET_PROPERTIES)
            );
            self.states.lock().expect("state lock").push(state.clone());
            match state {
                PagerState::Initial => json_response(
                    StatusCode::Ok,
                    Headers::new(),
                    &json!({
                        "nextLink": "/opaque-next-page?cursor=page-2",
                        "value": [],
                    }),
                ),
                PagerState::More(PagerContinuation::Link(_)) => {
                    json_response(StatusCode::Ok, Headers::new(), &json!({ "value": [] }))
                }
                other => panic!("unexpected state: {other:?}"),
            }
        }
    }

    #[tokio::test]
    async fn propagates_pager_state_for_fake_server_callbacks() -> Result<()> {
        let server = Arc::new(ListSecretsServer::default());
        let client =
            SecretClient::new_fake("https://example.vault.azure.net", server.clone(), None)?;

        let mut pages = client.list_secret_properties(None)?.into_pages();
        let page = pages.try_next().await?.expect("expected first page");
        let first = page.into_model()?;
        assert!(first.next_link.is_some());
        let page = pages.try_next().await?.expect("expected second page");
        let second = page.into_model()?;
        assert!(second.next_link.is_none());
        assert!(pages.try_next().await?.is_none());

        let states = server.states.lock().expect("state lock");
        assert_eq!(states.len(), 2);
        assert_eq!(states[0], PagerState::Initial);
        assert!(matches!(
            states[1],
            PagerState::More(PagerContinuation::Link(_))
        ));
        Ok(())
    }

    #[derive(Debug, Default)]
    struct ListSecretVersionsServer {
        calls: Mutex<Vec<(String, Option<i32>, PagerState)>>,
    }

    #[async_trait]
    impl SecretServer for ListSecretVersionsServer {
        async fn list_secret_properties_versions(
            &self,
            secret_name: &str,
            state: PagerState,
            options: Option<SecretClientListSecretPropertiesVersionsOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            let options = options.expect("expected options");
            assert_eq!(options.maxresults, Some(7));
            assert_eq!(
                options
                    .method_options
                    .context
                    .value::<ApiName>()
                    .map(|name| name.as_ref()),
                Some(API_LIST_SECRET_PROPERTIES_VERSIONS)
            );
            self.calls.lock().expect("calls lock").push((
                secret_name.to_owned(),
                options.maxresults,
                state.clone(),
            ));

            match state {
                PagerState::Initial => json_response(
                    StatusCode::Ok,
                    Headers::new(),
                    &json!({
                        "nextLink": "/opaque-next-page?cursor=ver-2",
                        "value": [],
                    }),
                ),
                PagerState::More(PagerContinuation::Link(_)) => {
                    json_response(StatusCode::Ok, Headers::new(), &json!({ "value": [] }))
                }
                other => panic!("unexpected state: {other:?}"),
            }
        }
    }

    #[tokio::test]
    async fn preserves_versions_parameters_for_opaque_next_links() -> Result<()> {
        let server = Arc::new(ListSecretVersionsServer::default());
        let client =
            SecretClient::new_fake("https://example.vault.azure.net", server.clone(), None)?;

        let options = SecretClientListSecretPropertiesVersionsOptions {
            maxresults: Some(7),
            ..Default::default()
        };
        let mut pages = client
            .list_secret_properties_versions("my-secret", Some(options))?
            .into_pages();
        let page = pages.try_next().await?.expect("expected first page");
        let first = page.into_model()?;
        assert!(first.next_link.is_some());
        let page = pages.try_next().await?.expect("expected second page");
        let second = page.into_model()?;
        assert!(second.next_link.is_none());
        assert!(pages.try_next().await?.is_none());

        let calls = server.calls.lock().expect("calls lock");
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "my-secret");
        assert_eq!(calls[0].1, Some(7));
        assert_eq!(calls[0].2, PagerState::Initial);
        assert_eq!(calls[1].0, "my-secret");
        assert_eq!(calls[1].1, Some(7));
        assert!(matches!(
            calls[1].2,
            PagerState::More(PagerContinuation::Link(_))
        ));
        Ok(())
    }

    #[derive(Debug, Default)]
    struct ErrorServer;

    #[async_trait]
    impl SecretServer for ErrorServer {
        async fn get_secret(
            &self,
            _secret_name: &str,
            _options: Option<SecretClientGetSecretOptions<'_>>,
        ) -> Result<AsyncRawResponse> {
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::new(),
                azure_core::Bytes::from_static(
                    br#"{"error":{"code":"BadRequest","message":"nope"}}"#,
                ),
            ))
        }
    }

    #[tokio::test]
    async fn preserves_raw_error_responses_from_fake_server() -> Result<()> {
        let client = SecretClient::new_fake(
            "https://example.vault.azure.net",
            Arc::new(ErrorServer),
            None,
        )?;
        let err = client
            .get_secret("my-secret", None)
            .await
            .expect_err("expected HTTP error");
        assert_eq!(err.http_status(), Some(StatusCode::BadRequest));
        Ok(())
    }

    #[derive(Debug)]
    struct NoopServer;

    #[async_trait]
    impl SecretServer for NoopServer {}

    #[tokio::test]
    async fn errors_for_missing_and_unknown_api_name() -> Result<()> {
        let transport = SecretServerTransport::new(Arc::new(NoopServer));
        let mut request = Request::new(
            "https://example.vault.azure.net/secrets/my-secret?api-version=2026-03-01-preview"
                .parse()?,
            Method::Get,
        );

        let err = transport
            .send(&Context::new(), &mut request, &[])
            .await
            .expect_err("expected missing ApiName error");
        assert!(err.to_string().contains("missing ApiName"));

        let context = Context::new().with_value(ApiName("KeyVault.unknown".to_string()));
        let err = transport
            .send(&context, &mut request, &[])
            .await
            .expect_err("expected unknown ApiName error");
        assert!(err.to_string().contains("unknown ApiName"));
        Ok(())
    }

    #[tokio::test]
    async fn errors_for_method_or_query_mismatch() -> Result<()> {
        let transport = SecretServerTransport::new(Arc::new(NoopServer));

        let context = Context::new().with_value(ApiName(API_GET_SECRET.to_string()));
        let mut wrong_method = Request::new(
            "https://example.vault.azure.net/secrets/my-secret?api-version=2026-03-01-preview"
                .parse()?,
            Method::Post,
        );
        let err = transport
            .send(&context, &mut wrong_method, &[])
            .await
            .expect_err("expected method mismatch");
        assert!(err.to_string().contains("expected HTTP method"));

        let mut wrong_query = Request::new(
            "https://example.vault.azure.net/secrets/my-secret?api-version=2026-03-01-preview&oops=1"
                .parse()?,
            Method::Get,
        );
        let err = transport
            .send(&context, &mut wrong_query, &[])
            .await
            .expect_err("expected query mismatch");
        assert!(err.to_string().contains("unexpected query parameter"));
        Ok(())
    }
}
