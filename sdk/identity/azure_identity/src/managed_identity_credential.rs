// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    env::Env, AppServiceManagedIdentityCredential, ImdsId, TokenCredentialOptions,
    VirtualMachineManagedIdentityCredential,
};
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use std::sync::Arc;
use tracing::info;

/// Identifies a specific user-assigned identity for [`ManagedIdentityCredential`] to authenticate.
#[derive(Debug, Clone)]
pub enum UserAssignedId {
    /// The client ID of a user-assigned identity
    ClientId(String),
    /// The object or principal ID of a user-assigned identity
    ObjectId(String),
    /// The Azure resource ID of a user-assigned identity
    ResourceId(String),
}

/// Authenticates a managed identity from Azure App Service or an Azure Virtual Machine.
#[derive(Debug)]
pub struct ManagedIdentityCredential {
    credential: Arc<dyn TokenCredential>,
}

/// Options for constructing a new [`ManagedIdentityCredential`].
#[derive(Clone, Debug, Default)]
pub struct ManagedIdentityCredentialOptions {
    /// The [`TokenCredentialOptions`] to use for the credential.
    pub credential_options: TokenCredentialOptions,

    /// Specifies a user-assigned identity the credential should authenticate.
    /// When `None`, the credential will authenticate a system-assigned identity, if any.
    pub user_assigned_id: Option<UserAssignedId>,
}

impl ManagedIdentityCredential {
    pub fn new(options: Option<ManagedIdentityCredentialOptions>) -> azure_core::Result<Arc<Self>> {
        let options = options.unwrap_or_default();
        let env = options.credential_options.env();
        let source = get_source(env);
        let id = options
            .user_assigned_id
            .clone()
            .map(Into::into)
            .unwrap_or(ImdsId::SystemAssigned);

        let credential: Arc<dyn TokenCredential> = match source {
            ManagedIdentitySource::AppService => {
                // App Service does accept resource IDs, however this crate's current implementation sends
                // them in the wrong query parameter: https://github.com/Azure/azure-sdk-for-rust/issues/2407
                if let ImdsId::MsiResId(_) = id {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Credential,
                        || {
                            "User-assigned resource IDs aren't supported for App Service. Use a client or object ID instead.".to_string()
                        },
                    ));
                }
                AppServiceManagedIdentityCredential::new(id, options.credential_options)?
            }
            ManagedIdentitySource::Imds => {
                VirtualMachineManagedIdentityCredential::new(id, options.credential_options)?
            }
            _ => {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Credential,
                    || format!("{} managed identity isn't supported", source.as_str()),
                ));
            }
        };

        info!(user_assigned_id = ?options.user_assigned_id, "ManagedIdentityCredential will use {} managed identity", source.as_str());

        Ok(Arc::new(Self { credential }))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ManagedIdentityCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        if scopes.len() != 1 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Credential,
                || "ManagedIdentityCredential requires exactly one scope".to_string(),
            ));
        }
        self.credential.get_token(scopes, options).await
    }
}

#[derive(Debug, Copy, Clone)]
enum ManagedIdentitySource {
    AzureArc,
    AzureML,
    AppService,
    CloudShell,
    Imds,
    ServiceFabric,
}

impl ManagedIdentitySource {
    pub fn as_str(&self) -> &'static str {
        match self {
            ManagedIdentitySource::AzureArc => "Azure Arc",
            ManagedIdentitySource::AzureML => "Azure ML",
            ManagedIdentitySource::AppService => "App Service",
            ManagedIdentitySource::CloudShell => "CloudShell",
            ManagedIdentitySource::Imds => "IMDS",
            ManagedIdentitySource::ServiceFabric => "Service Fabric",
        }
    }
}

const IDENTITY_ENDPOINT: &str = "IDENTITY_ENDPOINT";
const IDENTITY_HEADER: &str = "IDENTITY_HEADER";
const IDENTITY_SERVER_THUMBPRINT: &str = "IDENTITY_SERVER_THUMBPRINT";
const IMDS_ENDPOINT: &str = "IMDS_ENDPOINT";
const MSI_ENDPOINT: &str = "MSI_ENDPOINT";
const MSI_SECRET: &str = "MSI_SECRET";

fn get_source(env: &Env) -> ManagedIdentitySource {
    use ManagedIdentitySource::*;
    if env.var(IDENTITY_ENDPOINT).is_ok() {
        if env.var(IDENTITY_HEADER).is_ok() {
            if env.var(IDENTITY_SERVER_THUMBPRINT).is_ok() {
                return ServiceFabric;
            }
            return AppService;
        } else if env.var(IMDS_ENDPOINT).is_ok() {
            return AzureArc;
        }
    } else if env.var(MSI_ENDPOINT).is_ok() {
        if env.var(MSI_SECRET).is_ok() {
            return AzureML;
        }
        return CloudShell;
    }
    Imds
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::Env;
    use crate::tests::{LIVE_TEST_RESOURCE, LIVE_TEST_SCOPES};
    use azure_core::http::headers::Headers;
    use azure_core::http::{Method, RawResponse, Request, StatusCode};
    use azure_core::Bytes;
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    const EXPIRES_ON: &str = "EXPIRES_ON";

    fn imds_available() -> bool {
        std::env::var("IDENTITY_IMDS_AVAILABLE").is_ok()
    }

    async fn run_supported_source_test(
        env: Env,
        options: Option<ManagedIdentityCredentialOptions>,
        expected_source: ManagedIdentitySource,
        model_request: Request,
        response_format: String,
    ) {
        let actual_source = get_source(&env);
        assert_eq!(
            std::mem::discriminant(&actual_source),
            std::mem::discriminant(&expected_source)
        );
        let token_requests = Arc::new(AtomicUsize::new(0));
        let token_requests_clone = token_requests.clone();
        let expires_on = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;
        let mock_client = MockHttpClient::new(move |actual| {
            {
                token_requests_clone.fetch_add(1, Ordering::SeqCst);
                let expected = model_request.clone();
                let response_format = response_format.clone();
                async move {
                    assert_eq!(expected.method(), actual.method());

                    let mut actual_params: Vec<_> =
                        actual.url().query_pairs().into_owned().collect();
                    actual_params.sort();
                    let mut expected_params: Vec<_> =
                        expected.url().query_pairs().into_owned().collect();
                    expected_params.sort();
                    assert_eq!(expected_params, actual_params);

                    let mut actual_url = actual.url().clone();
                    actual_url.set_query(None);
                    let mut expected_url = expected.url().clone();
                    expected_url.set_query(None);
                    assert_eq!(actual_url, expected_url);

                    // allow additional headers in the actual request so changing
                    // the underlying client in the future won't break tests
                    expected.headers().iter().for_each(|(k, v)| {
                        assert_eq!(actual.headers().get_str(k).unwrap(), v.as_str())
                    });

                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::default(),
                        Bytes::from(response_format.replacen(
                            EXPIRES_ON,
                            &expires_on.to_string(),
                            1,
                        )),
                    ))
                }
            }
            .boxed()
        });
        let mut options = options.unwrap_or_default();
        options.credential_options = TokenCredentialOptions {
            env,
            http_client: Arc::new(mock_client),
            ..Default::default()
        };
        let cred = ManagedIdentityCredential::new(Some(options)).expect("credential");
        for _ in 0..4 {
            let token = cred.get_token(LIVE_TEST_SCOPES, None).await.expect("token");
            assert_eq!(token.expires_on.unix_timestamp(), expires_on as i64);
            assert_eq!(token.token.secret(), "*");
            assert_eq!(token_requests.load(Ordering::SeqCst), 1);
        }
    }

    fn run_unsupported_source_test(env: Env, expected_source: ManagedIdentitySource) {
        let actual_source = get_source(&env);
        assert_eq!(
            std::mem::discriminant(&actual_source),
            std::mem::discriminant(&expected_source)
        );
        let result = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
            credential_options: TokenCredentialOptions {
                env,
                ..Default::default()
            },
            ..Default::default()
        }));
        assert!(
            matches!(result, Err(ref e) if *e.kind() == azure_core::error::ErrorKind::Credential),
            "Expected constructor error"
        );
    }

    async fn run_app_service_test(options: Option<ManagedIdentityCredentialOptions>) {
        let endpoint = "http://localhost/metadata/identity/oauth2/token";
        let x_id_header = "x-id-header";
        let mut model = Request::new(endpoint.parse().unwrap(), Method::Get);
        model.insert_header("x-identity-header", x_id_header);
        let mut params = Vec::from([
            ("api-version", "2019-08-01"),
            ("resource", LIVE_TEST_RESOURCE),
        ]);
        if let Some(options) = options.as_ref() {
            if let Some(ref id) = options.user_assigned_id {
                match id {
                    UserAssignedId::ClientId(client_id) => {
                        params.push(("client_id", client_id));
                    }
                    UserAssignedId::ObjectId(object_id) => {
                        params.push(("object_id", object_id));
                    }
                    UserAssignedId::ResourceId(resource_id) => {
                        params.push(("mi_res_id", resource_id));
                    }
                }
            }
        }
        model.url_mut().query_pairs_mut().extend_pairs(params);
        run_supported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, endpoint),
                    (IDENTITY_HEADER, x_id_header),
                ][..],
            ),
            options,
            ManagedIdentitySource::AppService,
            model,
            format!(
                r#"{{"access_token":"*","expires_on":"{}","resource":"{}","token_type":"Bearer"}}"#,
                EXPIRES_ON, LIVE_TEST_RESOURCE
            )
            .to_string(),
        )
        .await;
    }

    #[tokio::test]
    async fn app_service() {
        run_app_service_test(None).await;
    }

    #[tokio::test]
    async fn app_service_client_id() {
        run_app_service_test(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: Some(UserAssignedId::ClientId("expected client ID".to_string())),
            ..Default::default()
        }))
        .await;
    }

    #[tokio::test]
    async fn app_service_object_id() {
        run_app_service_test(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: Some(UserAssignedId::ObjectId("expected object ID".to_string())),
            ..Default::default()
        }))
        .await;
    }

    #[tokio::test]
    async fn app_service_resource_id() {
        let result = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
            credential_options: TokenCredentialOptions {
                env: Env::from(&[(IDENTITY_ENDPOINT, "..."), (IDENTITY_HEADER, "x-id-header")][..]),
                ..Default::default()
            },
            user_assigned_id: Some(UserAssignedId::ResourceId(
                "expected resource ID".to_string(),
            )),
        }));
        assert!(
            matches!(result, Err(ref e) if *e.kind() == azure_core::error::ErrorKind::Credential),
            "Expected constructor error"
        );
    }

    #[test]
    fn arc() {
        run_unsupported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, "http://localhost"),
                    (IMDS_ENDPOINT, "..."),
                ][..],
            ),
            ManagedIdentitySource::AzureArc,
        );
    }

    #[test]
    fn azure_ml() {
        run_unsupported_source_test(
            Env::from(&[(MSI_ENDPOINT, "..."), (MSI_SECRET, "...")][..]),
            ManagedIdentitySource::AzureML,
        );
    }

    #[test]
    fn cloudshell() {
        run_unsupported_source_test(
            Env::from(&[(MSI_ENDPOINT, "http://localhost")][..]),
            ManagedIdentitySource::CloudShell,
        );
    }

    async fn run_imds_test(options: Option<ManagedIdentityCredentialOptions>) {
        let mut model = Request::new(
            "http://169.254.169.254/metadata/identity/oauth2/token"
                .parse()
                .unwrap(),
            Method::Get,
        );
        model.insert_header("metadata", "true");

        let mut params = Vec::from([
            ("api-version", "2019-08-01"),
            ("resource", LIVE_TEST_RESOURCE),
        ]);
        if let Some(options) = options.as_ref() {
            if let Some(ref id) = options.user_assigned_id {
                match id {
                    UserAssignedId::ClientId(client_id) => {
                        params.push(("client_id", client_id));
                    }
                    UserAssignedId::ObjectId(object_id) => {
                        params.push(("object_id", object_id));
                    }
                    UserAssignedId::ResourceId(resource_id) => {
                        params.push(("msi_res_id", resource_id));
                    }
                }
            }
        }
        model.url_mut().query_pairs_mut().extend_pairs(params);

        run_supported_source_test(
            Env::from(&[][..]),
            options,
            ManagedIdentitySource::Imds,
            model,
            format!(r#"{{"token_type":"Bearer","expires_in":"85770","expires_on":"{}","ext_expires_in":86399,"access_token":"*","resource":"{}"}}"#, EXPIRES_ON, LIVE_TEST_RESOURCE).to_string(),
        ).await;
    }

    #[tokio::test]
    async fn imds() {
        run_imds_test(None).await;
    }

    #[tokio::test]
    async fn imds_client_id() {
        run_imds_test(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: Some(UserAssignedId::ClientId("expected client ID".to_string())),
            ..Default::default()
        }))
        .await;
    }

    #[tokio::test]
    async fn imds_object_id() {
        run_imds_test(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: Some(UserAssignedId::ObjectId("expected object ID".to_string())),
            ..Default::default()
        }))
        .await;
    }

    #[tokio::test]
    async fn imds_resource_id() {
        run_imds_test(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: Some(UserAssignedId::ResourceId(
                "expected resource ID".to_string(),
            )),
            ..Default::default()
        }))
        .await;
    }

    async fn run_live_imds_test(id: Option<UserAssignedId>) {
        if !imds_available() {
            return;
        }

        let credential = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: id,
            ..Default::default()
        }))
        .expect("credential");

        let token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("token");

        assert!(!token.token.secret().is_empty());
        assert!(token.expires_on.unix_timestamp() > 0);
        assert_eq!(time::UtcOffset::UTC, token.expires_on.offset());
    }

    #[tokio::test]
    async fn imds_live() {
        run_live_imds_test(None).await;
    }

    #[tokio::test]
    async fn imds_live_client_id() {
        let Some(id) = std::env::var("IDENTITY_VM_USER_ASSIGNED_MI_CLIENT_ID").ok() else {
            assert!(
                !imds_available(),
                "pipeline configuration error: live IMDS environment but no value for IDENTITY_VM_USER_ASSIGNED_MI_CLIENT_ID"
            );
            return;
        };
        run_live_imds_test(Some(UserAssignedId::ClientId(id))).await;
    }

    #[tokio::test]
    async fn imds_live_object_id() {
        let Some(id) = std::env::var("IDENTITY_VM_USER_ASSIGNED_MI_OBJECT_ID").ok() else {
            assert!(
                !imds_available(),
                "pipeline configuration error: live IMDS environment but no value for IDENTITY_VM_USER_ASSIGNED_MI_OBJECT_ID"
            );
            return;
        };
        run_live_imds_test(Some(UserAssignedId::ObjectId(id))).await;
    }

    #[tokio::test]
    async fn imds_live_resource_id() {
        let Some(id) = std::env::var("IDENTITY_VM_USER_ASSIGNED_MI_RESOURCE_ID").ok() else {
            assert!(
                !imds_available(),
                "pipeline configuration error: live IMDS environment but no value for IDENTITY_VM_USER_ASSIGNED_MI_RESOURCE_ID"
            );
            return;
        };
        run_live_imds_test(Some(UserAssignedId::ResourceId(id))).await;
    }

    #[tokio::test]
    async fn requires_one_scope() {
        let credential = ManagedIdentityCredential::new(None).expect("valid credential");
        for scopes in [&[][..], &["A", "B"][..]].iter() {
            credential
                .get_token(scopes, None)
                .await
                .expect_err("expected an error, got");
        }
    }

    #[test]
    fn service_fabric() {
        run_unsupported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, "http://localhost"),
                    (IDENTITY_HEADER, "..."),
                    (IDENTITY_SERVER_THUMBPRINT, "..."),
                ][..],
            ),
            ManagedIdentitySource::ServiceFabric,
        );
    }
}
