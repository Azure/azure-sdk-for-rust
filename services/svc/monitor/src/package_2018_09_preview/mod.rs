#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://monitoring.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn metrics_client(&self) -> metrics::Client {
        metrics::Client(self.clone())
    }
}
pub mod metrics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "**Post the metric values for a resource**."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `content_type`: Supports application/json and application/x-ndjson"]
        #[doc = "* `content_length`: Content length of the payload"]
        #[doc = "* `authorization`: Authorization token issue for issued for audience \"https:\\\\monitoring.azure.com\\\""]
        #[doc = "* `subscription_id`: The azure subscription id"]
        #[doc = "* `resource_group_name`: The ARM resource group name"]
        #[doc = "* `resource_provider`: The ARM resource provider name"]
        #[doc = "* `resource_type_name`: The ARM resource type name"]
        #[doc = "* `resource_name`: The ARM resource name"]
        #[doc = "* `body`: The Azure metrics document json payload"]
        pub fn create(
            &self,
            content_type: impl Into<String>,
            content_length: i32,
            authorization: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_provider: impl Into<String>,
            resource_type_name: impl Into<String>,
            resource_name: impl Into<String>,
            body: impl Into<models::AzureMetricsDocument>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                content_type: content_type.into(),
                content_length,
                authorization: authorization.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_provider: resource_provider.into(),
                resource_type_name: resource_type_name.into(),
                resource_name: resource_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::AzureMetricsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) content_type: String,
            pub(crate) content_length: i32,
            pub(crate) authorization: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider: String,
            pub(crate) resource_type_name: String,
            pub(crate) resource_name: String,
            pub(crate) body: models::AzureMetricsDocument,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/metrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_provider,
                            &this.resource_type_name,
                            &this.resource_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", &this.content_type);
                        req.insert_header("content-length", &this.content_length.to_string());
                        req.insert_header("authorization", &this.authorization);
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureMetricsResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
