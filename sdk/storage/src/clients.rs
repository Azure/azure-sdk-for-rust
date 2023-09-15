use crate::authorization::AuthorizationPolicy;
use crate::shared_access_signature::account_sas::{
    AccountSasPermissions, AccountSasResource, AccountSasResourceType, AccountSharedAccessSignature,
};
use crate::StorageCredentials;
use azure_core::date;
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    Body, ClientOptions, Method, Pipeline, Request,
};
use std::sync::Arc;
use time::OffsetDateTime;
use url::Url;

/// The well-known account used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT: &str = "devstoreaccount1";

/// The well-known account key used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

const AZURE_VERSION: HeaderValue = HeaderValue::from_static("2020-10-02");

#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    Blob,
    Queue,
    // File,
    Table,
    DataLake,
}

impl ServiceType {
    pub fn subdomain(&self) -> &str {
        match self {
            ServiceType::Blob => "blob",
            ServiceType::Queue => "queue",
            ServiceType::Table => "table",
            ServiceType::DataLake => "dfs",
        }
    }
}

pub fn shared_access_signature(
    storage_credentials: &StorageCredentials,
    resource: AccountSasResource,
    resource_type: AccountSasResourceType,
    expiry: OffsetDateTime,
    permissions: AccountSasPermissions,
) -> Result<AccountSharedAccessSignature, Error> {
    match storage_credentials {
            StorageCredentials::Key(account, key) => {
                Ok(AccountSharedAccessSignature::new(account.clone(), key.clone(), resource, resource_type, expiry, permissions))
            }
            _ => Err(Error::message(ErrorKind::Credential, "failed shared access signature generation. SAS can be generated only from key and account clients")),
        }
}

pub fn finalize_request(
    url: Url,
    method: Method,
    headers: Headers,
    request_body: Option<Body>,
) -> Result<Request, Error> {
    let dt = OffsetDateTime::now_utc();
    let time = date::to_rfc1123(&dt);
    let mut request = Request::new(url, method);
    for (k, v) in headers {
        request.insert_header(k, v);
    }
    // let's add content length to avoid "chunking" errors.
    match request_body {
        Some(ref b) => request.insert_header(CONTENT_LENGTH, b.len().to_string()),
        None => request.insert_header(CONTENT_LENGTH, "0"),
    };
    request.insert_header(MS_DATE, time);
    request.insert_header(VERSION, AZURE_VERSION);
    if let Some(request_body) = request_body {
        request.set_body(request_body);
    } else {
        request.set_body(azure_core::EMPTY_BODY);
    };
    Ok(request)
}

/// Create a Pipeline from `ClientOptions`
pub fn new_pipeline_from_options(
    options: ClientOptions,
    credentials: StorageCredentials,
) -> Pipeline {
    let auth_policy: Arc<dyn azure_core::Policy> = Arc::new(AuthorizationPolicy::new(credentials));

    // The `AuthorizationPolicy` must be the **last** retry policy.
    // Policies can change the url and/or the headers, and the `AuthorizationPolicy`
    // must be able to inspect them or the resulting token will be invalid.
    let per_retry_policies = vec![
        Arc::new(options.timeout.clone()) as Arc<dyn azure_core::Policy>,
        auth_policy,
    ];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options,
        Vec::new(),
        per_retry_policies,
    )
}
