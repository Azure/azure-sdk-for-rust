use super::DatabaseClient;
use crate::authorization_policy::{generate_authorization, generate_resource_link, CosmosContext};
use crate::headers::*;
use crate::operations::*;
use crate::resources::permission::AuthorizationToken;
use crate::resources::ResourceType;
use crate::{ReadonlyString, TimeNonce};

use azure_core::pipeline::Pipeline;
use azure_core::prelude::Continuation;
use azure_core::HttpClient;
use azure_core::Request;
use azure_core::*;
use futures::stream::unfold;
use futures::Stream;
use http::request::Builder as RequestBuilder;
use http::{header, HeaderValue};

use std::fmt::Debug;
use std::sync::Arc;

/// The well-known account key used by Azure Cosmos DB Emulator.
/// https://docs.microsoft.com/azure/cosmos-db/local-emulator?tabs=ssl-netstd21#connect-with-emulator-apis
pub const EMULATOR_ACCOUNT_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

const AZURE_VERSION: &str = "2018-12-31";

/// A plain Cosmos client.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pipeline: Pipeline<CosmosContext>,
    auth_token: AuthorizationToken,
    cloud_location: CloudLocation,
}

/// Options for specifying how a Cosmos client will behave
#[derive(Debug, Clone)]
pub struct CosmosOptions {
    options: ClientOptions<CosmosContext>,
}

impl CosmosOptions {
    /// Create new options with a given transaction name
    pub fn new(
        #[cfg(feature = "mock_transport_framework")] transaction_name: impl Into<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "mock_transport_framework")]
            options: ClientOptions::new(transaction_name.into()),
            #[cfg(not(feature = "mock_transport_framework"))]
            options: ClientOptions::default(),
        }
    }
}

#[cfg(not(feature = "mock_transport_framework"))]
impl Default for CosmosOptions {
    fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }
}

/// Create a Pipeline from CosmosOptions
fn new_pipeline_from_options(
    options: CosmosOptions,
    authorization_token: AuthorizationToken,
) -> Pipeline<CosmosContext> {
    let auth_policy: Arc<dyn azure_core::Policy<CosmosContext>> =
        Arc::new(crate::AuthorizationPolicy::new(authorization_token));

    let mut per_retry_policies = Vec::new();
    // take care of adding the AuthorizationPolicy as **last** retry policy.
    // Policies can change the url and/or the headers and the AuthorizationPolicy
    // must be able to inspect them or the resulting token will be invalid.
    per_retry_policies.push(auth_policy);

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        &options.options,
        Vec::new(),
        per_retry_policies,
    )
}

impl CosmosClient {
    /// Create a new `CosmosClient` which connects to the account's instance in the public Azure cloud.
    pub fn new(account: String, auth_token: AuthorizationToken, options: CosmosOptions) -> Self {
        let cloud_location = CloudLocation::Public(account);
        // TODO: The AuthorizationToken will only be stored in the pipeline via its policy.
        // Right now the AuthorizationToken is a field of the Client.
        // This will be corrected once every Cosmos function has been be migrated to the pipeline.
        // Once that happens, we will remove the clone below.
        let pipeline = new_pipeline_from_options(options, auth_token.clone());
        Self {
            pipeline,
            auth_token,
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in the Chinese Azure cloud.
    pub fn new_china(
        account: String,
        auth_token: AuthorizationToken,
        options: CosmosOptions,
    ) -> Self {
        let cloud_location = CloudLocation::China(account);
        let pipeline = new_pipeline_from_options(options, auth_token.clone());
        Self {
            pipeline,
            auth_token,
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in custom Azure cloud.
    pub fn new_custom(
        account: String,
        auth_token: AuthorizationToken,
        uri: String,
        options: CosmosOptions,
    ) -> Self {
        let cloud_location = CloudLocation::Custom { account, uri };
        let pipeline = new_pipeline_from_options(options, auth_token.clone());
        Self {
            pipeline,
            auth_token,
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in Azure emulator
    pub fn new_emulator(address: &str, port: u16, options: CosmosOptions) -> Self {
        let auth_token = AuthorizationToken::primary_from_base64(EMULATOR_ACCOUNT_KEY).unwrap();
        let uri = format!("https://{}:{}", address, port);
        let cloud_location = CloudLocation::Custom {
            account: String::from("Custom"),
            uri,
        };
        let pipeline = new_pipeline_from_options(options, auth_token.clone());
        Self {
            pipeline,
            auth_token,
            cloud_location,
        }
    }

    /// Set the auth token used
    pub fn auth_token(&mut self, auth_token: AuthorizationToken) {
        // TODO: To remove once everything uses the AutorizationPolicy
        self.auth_token = auth_token.clone();

        // we replace the AuthorizationPolicy. This is
        // the last-1 policy by construction.
        let auth_policy: Arc<dyn azure_core::Policy<CosmosContext>> =
            Arc::new(crate::AuthorizationPolicy::new(auth_token));

        self.pipeline
            .replace_policy(auth_policy, self.pipeline.policies().len() - 2);
    }

    /// Create a database
    pub async fn create_database<S: AsRef<str>>(
        &self,
        ctx: Context,
        database_name: S,
        options: CreateDatabaseOptions,
    ) -> Result<CreateDatabaseResponse, crate::Error> {
        let mut request = self.prepare_request_pipeline("dbs", http::Method::POST);

        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request, database_name.as_ref())?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        Ok(CreateDatabaseResponse::try_from(response).await?)
    }

    /// List all databases
    pub fn list_databases(
        &self,
        ctx: Context,
        options: ListDatabasesOptions,
    ) -> impl Stream<Item = Result<ListDatabasesResponse, crate::Error>> + '_ {
        macro_rules! r#try {
            ($expr:expr $(,)?) => {
                match $expr {
                    Result::Ok(val) => val,
                    Result::Err(err) => {
                        return Some((Err(err.into()), State::Done));
                    }
                }
            };
        }

        #[derive(Debug, Clone, PartialEq)]
        enum State {
            Init,
            Continuation(String),
            Done,
        }

        unfold(State::Init, move |state: State| {
            let this = self.clone();
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => {
                        let mut request = this.prepare_request_pipeline("dbs", http::Method::GET);
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Databases.into());

                        r#try!(options.decorate_request(&mut request).await);
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        let response = r#try!(response.validate(http::StatusCode::OK).await);

                        ListDatabasesResponse::try_from(response).await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        let mut request = this.prepare_request_pipeline("dbs", http::Method::GET);
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Databases.into());

                        r#try!(options.decorate_request(&mut request).await);
                        r#try!(continuation.add_as_header2(&mut request));
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        let response = r#try!(response.validate(http::StatusCode::OK).await);
                        ListDatabasesResponse::try_from(response).await
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(|ct| State::Continuation(ct))
                    .unwrap_or(State::Done);

                Some((Ok(response), next_state))
            }
        })
    }

    /// Convert into a [`DatabaseClient`]
    pub fn into_database_client<S: Into<ReadonlyString>>(self, database_name: S) -> DatabaseClient {
        DatabaseClient::new(self, database_name)
    }

    /// Prepares an `http::RequestBuilder`.
    ///
    /// TODO: Remove once all operations have been moved to pipeline architecture. This is used by
    /// legacy operations that have not moved to the use of the pipeline architecture. Once
    /// that is complete, this will be superceded by `prepare_request_pipeline`.
    pub(crate) fn prepare_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
        resource_type: ResourceType,
    ) -> RequestBuilder {
        let time = TimeNonce::default();

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            trace!(
                "resource_link generated by prepare_request == {}",
                resource_link
            );
            generate_authorization(
                &self.auth_token,
                &http_method,
                &resource_type,
                resource_link,
                time,
            )
        };
        self.prepare_request_with_signature(uri_path, http_method, time, &auth)
    }

    /// Prepares' an `azure_core::Request`. This function will
    /// add the cloud location to the URI suffix and generate
    /// a Request with the specified HTTP Method.
    /// It will also set the body to an empty Bytes instance.
    /// *Note*: This call does not handle authorization as
    /// it will be done by the `AuthorizationPolicy`.
    ///
    /// Note: Eventually this method will replace `prepare_request` fully.
    pub(crate) fn prepare_request_pipeline(
        &self,
        uri_path: &str,
        http_method: http::Method,
    ) -> Request {
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        RequestBuilder::new()
            .method(http_method)
            .uri(uri)
            .body(bytes::Bytes::new())
            .unwrap()
            .into()
    }

    fn prepare_request_with_signature(
        &self,
        uri_path: &str,
        http_method: http::Method,
        time_nonce: TimeNonce,
        signature: &str,
    ) -> RequestBuilder {
        trace!("prepare_request::auth == {:?}", signature);
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        debug!(
            "cosmos::client::prepare_request_with_resource_signature::uri == {:?}",
            uri
        );

        RequestBuilder::new()
            .method(http_method)
            .uri(uri)
            .header(HEADER_DATE, time_nonce.to_string())
            .header(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION))
            .header(header::AUTHORIZATION, signature)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline<CosmosContext> {
        &self.pipeline
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.pipeline.http_client()
    }
}

/// The cloud with which you want to interact.
///
/// All variants require the cosmos account name. `Custom` also requires a valid
/// base URL (e.g. https://custom.documents.azure.com)
#[derive(Debug, Clone)]
enum CloudLocation {
    /// Azure public cloud
    Public(String),
    /// Azure China cloud
    China(String),
    // TODO: Other govt clouds?
    /// A custom base URL
    Custom { account: String, uri: String },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudLocation::Public(account) => format!("https://{}.documents.azure.com", account),
            CloudLocation::China(account) => format!("https://{}.documents.azure.cn", account),
            CloudLocation::Custom { uri, .. } => uri.clone(),
        }
    }
}
