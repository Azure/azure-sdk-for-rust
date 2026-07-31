// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::azure_arc_credential::{is_arc_agent_present, AzureArcCredential};
use crate::{
    authentication_error, env::Env, AppServiceManagedIdentityCredential, ImdsId,
    VirtualMachineManagedIdentityCredential,
};
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use azure_core::http::ClientOptions;
use std::{any::type_name, fmt, sync::Arc};
use tracing::info;

/// Identifies a specific user-assigned identity for [`ManagedIdentityCredential`] to authenticate.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum UserAssignedId {
    /// The client ID of a user-assigned identity
    ClientId(String),
    /// The object or principal ID of a user-assigned identity
    ObjectId(String),
    /// The Azure resource ID of a user-assigned identity
    ResourceId(String),
}

/// Authenticates a managed identity from Azure App Service, Azure Virtual Machine, or Azure Arc.
pub struct ManagedIdentityCredential {
    credential: Arc<dyn TokenCredential>,
}

impl fmt::Debug for ManagedIdentityCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

/// Options for constructing a new [`ManagedIdentityCredential`].
#[derive(Clone, Default)]
pub struct ManagedIdentityCredentialOptions {
    /// Specifies a user-assigned identity the credential should authenticate.
    /// When `None`, the credential will authenticate a system-assigned identity, if any.
    pub user_assigned_id: Option<UserAssignedId>,

    /// The [`ClientOptions`] to use for the credential's pipeline.
    pub client_options: ClientOptions,

    #[cfg(test)]
    pub(crate) env: Env,
}

impl fmt::Debug for ManagedIdentityCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

impl ManagedIdentityCredential {
    /// Creates a new instance of `ManagedIdentityCredential`.
    ///
    /// # Arguments
    /// * `options`: Options for configuring the credential. If `None`, the credential uses its default options.
    ///
    pub fn new(options: Option<ManagedIdentityCredentialOptions>) -> azure_core::Result<Arc<Self>> {
        let options = options.unwrap_or_default();
        #[cfg(test)]
        let env = options.env;
        #[cfg(not(test))]
        let env = Env::default();
        let source = get_source(&env);
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
                    return Err(azure_core::Error::with_message_fn(
                        azure_core::error::ErrorKind::Credential,
                        || {
                            "User-assigned resource IDs aren't supported for App Service. Use a client or object ID instead.".to_string()
                        },
                    ));
                }
                AppServiceManagedIdentityCredential::new(id, options.client_options, env)?
            }
            ManagedIdentitySource::Imds => {
                VirtualMachineManagedIdentityCredential::new(id, options.client_options, env)?
            }
            ManagedIdentitySource::AzureArc => {
                if !matches!(&id, ImdsId::SystemAssigned) {
                    return Err(azure_core::Error::with_message_fn(
                        azure_core::error::ErrorKind::Credential,
                        || {
                            "User-assigned managed identities aren't supported for Azure Arc. Only a system-assigned managed identity is supported.".to_string()
                        },
                    ));
                }
                AzureArcCredential::new(id, options.client_options, env)?
            }
            _ => {
                return Err(azure_core::Error::with_message_fn(
                    azure_core::error::ErrorKind::Credential,
                    || format!("{} managed identity isn't supported", source.as_str()),
                ));
            }
        };

        info!(user_assigned_id = ?options.user_assigned_id, "ManagedIdentityCredential will use {} managed identity", source.as_str());

        Ok(Arc::new(Self { credential }))
    }
}

#[async_trait::async_trait]
impl TokenCredential for ManagedIdentityCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        if scopes.len() != 1 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Credential,
                "ManagedIdentityCredential requires exactly one scope".to_string(),
            ));
        }
        self.credential
            .get_token(scopes, options)
            .await
            .map_err(|err| authentication_error(stringify!(ManagedIdentityCredential), err))
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

    if is_arc_agent_present(env) {
        return AzureArc;
    }

    Imds
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        env::Env,
        tests::{LIVE_TEST_RESOURCE, LIVE_TEST_SCOPES},
    };
    use azure_core::http::{
        headers::HeaderValue, AsyncRawResponse, Method, RawResponse, Request, StatusCode,
        Transport, Url,
    };
    use azure_core::time::OffsetDateTime;
    use azure_core::Bytes;
    use azure_core::{error::ErrorKind, http::headers::Headers};
    use azure_core_test::{http::MockHttpClient, recorded};
    use futures::FutureExt;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs::File, io::Write};
    use std::{
        fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    const EXPIRES_ON: &str = "EXPIRES_ON";

    async fn run_deployed_test(
        authority: &str,
        storage_name: &str,
        id: Option<UserAssignedId>,
    ) -> azure_core::Result<()> {
        let id_param = id.map_or("".to_string(), |id| match id {
            UserAssignedId::ClientId(id) => format!("client-id={id}&"),
            UserAssignedId::ObjectId(id) => format!("object-id={id}&"),
            UserAssignedId::ResourceId(id) => format!("resource-id={id}&"),
        });
        let url = format!(
            "http://{authority}/api?test=managed-identity&{id_param}storage-name={storage_name}"
        );
        let u = Url::parse(&url).expect("invalid URL");
        let client = azure_core::http::new_http_client(None);
        let req = Request::new(u, Method::Get);

        let res = client.execute_request(&req).await.expect("request failed");
        let status = res.status();
        let body = res.into_body().collect_string().await?;
        assert_eq!(StatusCode::Ok, status, "Test app responded with '{body}'");

        Ok(())
    }

    async fn run_error_response_test(source: ManagedIdentitySource) {
        let expected_status = StatusCode::ImATeapot;
        let headers = Headers::default();
        let content: &str = "is a teapot";
        let body = Bytes::copy_from_slice(content.as_bytes());
        let expected_response =
            RawResponse::from_bytes(expected_status, headers.clone(), body.clone());
        let mock_headers = headers.clone();
        let mock_body = body.clone();
        let mock_client = MockHttpClient::new(move |_| {
            let headers = mock_headers.clone();
            let body = mock_body.clone();
            async move { Ok(AsyncRawResponse::from_bytes(expected_status, headers, body)) }.boxed()
        });
        let test_env = match source {
            ManagedIdentitySource::Imds => Env::from(&[][..]),
            ManagedIdentitySource::AppService => Env::from(
                &[
                    (
                        IDENTITY_ENDPOINT,
                        "http://localhost/metadata/identity/oauth2/token",
                    ),
                    (IDENTITY_HEADER, "secret"),
                ][..],
            ),
            other => panic!("unsupported managed identity source {:?}", other),
        };
        let options = ManagedIdentityCredentialOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(Arc::new(mock_client))),
                ..Default::default()
            },
            env: test_env,
            ..Default::default()
        };
        let credential = ManagedIdentityCredential::new(Some(options)).expect("credential");
        let err = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("expected error");
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert_eq!(
            "ManagedIdentityCredential authentication failed. The request failed: is a teapot\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#managed-id",
            err.to_string(),
        );
        match err
            .downcast_ref::<azure_core::Error>()
            .expect("returned error should wrap an azure_core::Error")
            .kind()
        {
            ErrorKind::HttpResponse {
                error_code: None,
                raw_response: Some(response),
                status,
            } => {
                assert_eq!(response.as_ref(), &expected_response);
                assert_eq!(expected_status, *status);
            }
            err => panic!("unexpected {:?}", err),
        };
    }

    #[derive(Debug, Clone)]
    struct MockRequestResponse {
        request: Request,

        response_status: StatusCode,
        response_headers: Headers,
        response_format: String,
    }

    /// When using multiple entries in model_request_responses, it's important that the most specific request is first,
    /// because this function will use the first request-response pair that matches the request received.
    async fn run_supported_source_test(
        env: Env,
        options: Option<ManagedIdentityCredentialOptions>,
        expected_source: ManagedIdentitySource,
        model_request_responses: Vec<MockRequestResponse>,
        expected_token_err: Option<azure_core::Error>,
    ) {
        let actual_source = get_source(&env);
        assert_eq!(
            std::mem::discriminant(&actual_source),
            std::mem::discriminant(&expected_source)
        );
        let expected_token_request_count = model_request_responses.len();
        let token_requests = Arc::new(AtomicUsize::new(0));
        let token_requests_clone = token_requests.clone();
        let expires_on = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;
        let mock_client = MockHttpClient::new(move |actual_req: &Request| {
            {
                token_requests_clone.fetch_add(1, Ordering::SeqCst);
                let model_request_responses = model_request_responses.clone();
                async move {
                    for mock_request_response in model_request_responses {
                        let expected_request = mock_request_response.request;
                        if expected_request.method() != actual_req.method() {
                            continue;
                        }

                        let mut actual_params: Vec<_> =
                            actual_req.url().query_pairs().into_owned().collect();
                        actual_params.sort();
                        let mut expected_params: Vec<_> =
                            expected_request.url().query_pairs().into_owned().collect();
                        expected_params.sort();

                        if expected_params != actual_params {
                            continue;
                        }

                        let mut actual_url = actual_req.url().clone();
                        actual_url.set_query(None);
                        let mut expected_url = expected_request.url().clone();
                        expected_url.set_query(None);

                        if actual_url != expected_url {
                            continue;
                        }

                        // allow additional headers in the actual request so changing
                        // the underlying client in the future won't break tests
                        if !expected_request.headers().iter().all(
                            |(expected_header_name, expected_header_val)| {
                                let result = actual_req
                                    .headers()
                                    .get_str(expected_header_name)
                                    .map_or(false, |actual_header| {
                                        actual_header == expected_header_val.as_str()
                                    });
                                result
                            },
                        ) {
                            continue;
                        }

                        return Ok(AsyncRawResponse::from_bytes(
                            mock_request_response.response_status,
                            mock_request_response.response_headers,
                            Bytes::from(mock_request_response.response_format.replacen(
                                EXPIRES_ON,
                                &expires_on.to_string(),
                                1,
                            )),
                        ));
                    }
                    // if we got here, none of the model requests matched.
                    panic!("None of the model requests matched the actual request received");
                }
            }
            .boxed()
        });
        let mut options = options.unwrap_or_default();
        options.env = env;
        options.client_options = ClientOptions {
            transport: Some(Transport::new(Arc::new(mock_client))),
            ..Default::default()
        };
        let cred = ManagedIdentityCredential::new(Some(options)).expect("credential");
        for _ in 0..4 {
            let token_result = cred.get_token(LIVE_TEST_SCOPES, None).await;

            if let Some(expected_token_err) = &expected_token_err {
                let actual_token_err =
                    token_result.expect_err("Expected get_token to return an error");
                assert_eq!(actual_token_err.kind(), expected_token_err.kind());
                assert_eq!(actual_token_err.to_string(), expected_token_err.to_string());
            } else {
                let token = token_result.expect("Expected get_token to return a token");
                assert_eq!(token.expires_on.unix_timestamp(), expires_on as i64);
                assert_eq!(token.token.secret(), "*");
                assert_eq!(
                    token_requests.load(Ordering::SeqCst),
                    expected_token_request_count
                );
            }
        }
    }

    fn run_unsupported_source_test(env: Env, expected_source: ManagedIdentitySource) {
        let actual_source = get_source(&env);
        assert_eq!(
            std::mem::discriminant(&actual_source),
            std::mem::discriminant(&expected_source)
        );
        let result = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
            env,
            ..Default::default()
        }));
        assert!(
            matches!(result, Err(ref e) if *e.kind() == azure_core::error::ErrorKind::Credential),
            "Expected constructor error"
        );
    }

    #[recorded::test(live)]
    async fn aci_user_assigned_live() -> azure_core::Result<()> {
        if env::var("CI_HAS_DEPLOYED_RESOURCES").is_err() {
            println!("Skipped: ACI live tests require deployed resources");
            return Ok(());
        }
        let ip = env::var("IDENTITY_ACI_IP_USER_ASSIGNED").expect("IDENTITY_ACI_IP_USER_ASSIGNED");
        let storage_name = env::var("IDENTITY_STORAGE_NAME_USER_ASSIGNED")
            .expect("IDENTITY_STORAGE_NAME_USER_ASSIGNED");
        let client_id = env::var("IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID")
            .expect("IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID");
        run_deployed_test(
            &format!("{}:8080", ip),
            &storage_name,
            Some(UserAssignedId::ClientId(client_id)),
        )
        .await?;

        Ok(())
    }

    async fn run_app_service_test(options: Option<ManagedIdentityCredentialOptions>) {
        let endpoint = "http://localhost/metadata/identity/oauth2/token";
        let x_id_header = "x-id-header";
        let mut model_request = Request::new(endpoint.parse().unwrap(), Method::Get);
        model_request.insert_header("x-identity-header", x_id_header);
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
        model_request
            .url_mut()
            .query_pairs_mut()
            .extend_pairs(params);
        run_supported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, endpoint),
                    (IDENTITY_HEADER, x_id_header),
                ][..],
            ),
            options,
            ManagedIdentitySource::AppService,
            vec![
                MockRequestResponse{
                    request: model_request,
                    response_status: StatusCode::Ok,
                    response_headers: Headers::default(),
                    response_format: format!(
                    r#"{{"access_token":"*","expires_on":"{}","resource":"{}","token_type":"Bearer"}}"#,
                    EXPIRES_ON, LIVE_TEST_RESOURCE).to_string(),
                }
            ],
            None,
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
    async fn app_service_error_response() {
        run_error_response_test(ManagedIdentitySource::AppService).await
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
            env: Env::from(&[(IDENTITY_ENDPOINT, "..."), (IDENTITY_HEADER, "x-id-header")][..]),
            user_assigned_id: Some(UserAssignedId::ResourceId(
                "expected resource ID".to_string(),
            )),
            ..Default::default()
        }));
        assert!(
            matches!(result, Err(ref e) if *e.kind() == azure_core::error::ErrorKind::Credential),
            "Expected constructor error"
        );
    }

    #[tokio::test]
    async fn arc_challenge_response() {
        let mut model_naive_req = Request::new(
            "http://localhost:40342/metadata/identity/oauth2/token"
                .parse()
                .unwrap(),
            Method::Get,
        );
        model_naive_req.insert_header("metadata", "true");

        let params = Vec::from([
            ("api-version", "2021-02-01"),
            ("resource", LIVE_TEST_RESOURCE),
        ]);
        model_naive_req
            .url_mut()
            .query_pairs_mut()
            .extend_pairs(params);

        let key_id = rand::random::<u8>();
        let token_path = env::temp_dir().join(format!("arc-{key_id}.key"));
        let mut token_file = File::create_new(&token_path).unwrap();
        token_file.write_all("abc".as_bytes()).unwrap();
        drop(token_file);

        let mut model_challenge_response_request = model_naive_req.clone();
        model_challenge_response_request.insert_header("authorization", "Basic abc");

        let mut challenge_headers = Headers::default();
        let response_path = token_path.to_owned();
        let response_path_str = response_path.to_str().unwrap().to_string();
        let response_header = HeaderValue::from(format!("Realm={response_path_str}"));

        challenge_headers.insert("www-authenticate", response_header);

        run_supported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, "http://localhost:40342/metadata/identity/oauth2/token"),
                    (IMDS_ENDPOINT, "..."),
                ][..]),
            None,
            ManagedIdentitySource::AzureArc,
            vec![
                MockRequestResponse {
                    request: model_challenge_response_request,
                    response_status: StatusCode::Ok,
                    response_headers: Headers::default(),
                    response_format: format!(r#"{{"token_type":"Bearer","expires_in":"85770","expires_on":"{}","ext_expires_in":86399,"access_token":"*","resource":"{}"}}"#, EXPIRES_ON, LIVE_TEST_RESOURCE).to_string(),
                },
                MockRequestResponse {
                    request: model_naive_req,
                    response_status: StatusCode::Unauthorized,
                    response_headers: challenge_headers,
                    response_format: String::from(r#"{"error":"unauthorized_client","error_description":"Missing Basic Authorization header","error_codes":[401]}"#),
                },
            ],
            None,
        ).await;

        let _ = fs::remove_file(token_path); // try our best to clean up the temp file
    }

    #[tokio::test]
    async fn arc_challenge_response_too_large() {
        let mut model_naive_req = Request::new(
            "http://localhost:40342/metadata/identity/oauth2/token"
                .parse()
                .unwrap(),
            Method::Get,
        );
        model_naive_req.insert_header("metadata", "true");

        let params = Vec::from([
            ("api-version", "2021-02-01"),
            ("resource", LIVE_TEST_RESOURCE),
        ]);
        model_naive_req
            .url_mut()
            .query_pairs_mut()
            .extend_pairs(params);

        let key_id = rand::random::<u8>();
        let token_path = env::temp_dir().join(format!("arc-big{key_id}.key"));
        let mut token_file = File::create_new(&token_path).unwrap();
        let large_buf: [u8; 4097] = [0; 4097]; // 4096 is the max
        token_file.write_all(&large_buf).unwrap();
        drop(token_file);

        let mut model_challenge_response_request = model_naive_req.clone();
        model_challenge_response_request.insert_header("authorization", "Basic abc");

        let mut challenge_headers = Headers::default();
        let response_path = token_path.to_owned();
        let response_path_str = response_path.to_str().unwrap().to_string();
        let response_header = HeaderValue::from(format!("Realm={response_path_str}"));

        challenge_headers.insert("www-authenticate", response_header);

        run_supported_source_test(
            Env::from(
                &[
                    (IDENTITY_ENDPOINT, "http://localhost:40342/metadata/identity/oauth2/token"),
                    (IMDS_ENDPOINT, "..."),
                ][..]),
            None,
            ManagedIdentitySource::AzureArc,
            vec![
                MockRequestResponse {
                    request: model_challenge_response_request,
                    response_status: StatusCode::Ok,
                    response_headers: Headers::default(),
                    response_format: format!(r#"{{"token_type":"Bearer","expires_in":"85770","expires_on":"{}","ext_expires_in":86399,"access_token":"*","resource":"{}"}}"#, EXPIRES_ON, LIVE_TEST_RESOURCE).to_string(),
                },
                MockRequestResponse {
                    request: model_naive_req,
                    response_status: StatusCode::Unauthorized,
                    response_headers: challenge_headers,
                    response_format: String::from(r#"{"error":"unauthorized_client","error_description":"Missing Basic Authorization header","error_codes":[401]}"#),
                },
            ],
            Some(azure_core::Error::with_message(ErrorKind::Credential, "ManagedIdentityCredential authentication failed. Arc challenge token file was larger than expected\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#managed-id")),
        ).await;

        let _ = fs::remove_file(token_path); // try our best to clean up the temp file
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

    async fn run_imds_live_test(id: Option<UserAssignedId>) -> azure_core::Result<()> {
        if std::env::var("IDENTITY_IMDS_AVAILABLE").is_err() {
            println!("Skipped: IMDS isn't available");
            return Ok(());
        }

        let credential = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
            user_assigned_id: id,
            ..Default::default()
        }))
        .expect("valid credential");

        let token = credential.get_token(LIVE_TEST_SCOPES, None).await?;

        assert!(!token.token.secret().is_empty());
        assert_eq!(time::UtcOffset::UTC, token.expires_on.offset());
        assert!(token.expires_on.unix_timestamp() > OffsetDateTime::now_utc().unix_timestamp());

        Ok(())
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
            vec![
                MockRequestResponse {
                    request: model,
                    response_status: StatusCode::Ok,
                    response_headers: Headers::default(),
                    response_format: format!(r#"{{"token_type":"Bearer","expires_in":"85770","expires_on":"{}","ext_expires_in":86399,"access_token":"*","resource":"{}"}}"#, EXPIRES_ON, LIVE_TEST_RESOURCE).to_string(),
                },
            ],
            None,
        ).await;
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
    async fn imds_error_response() {
        run_error_response_test(ManagedIdentitySource::Imds).await
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

    #[tokio::test]
    async fn imds_system_assigned() {
        run_imds_test(None).await;
    }

    #[recorded::test(live)]
    async fn imds_system_assigned_live() -> azure_core::Result<()> {
        run_imds_live_test(None).await
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
