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
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
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
    pub fn app_component_client(&self) -> app_component::Client {
        app_component::Client(self.clone())
    }
    pub fn server_metrics_client(&self) -> server_metrics::Client {
        server_metrics::Client(self.clone())
    }
    pub fn test_client(&self) -> test::Client {
        test::Client(self.clone())
    }
    pub fn test_run_client(&self) -> test_run::Client {
        test_run::Client(self.clone())
    }
}
pub mod app_component {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get App Component details by App Component name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name of the App Component, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn get_app_component_by_name(&self, name: impl Into<String>) -> get_app_component_by_name::Builder {
            get_app_component_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Associate an App Component (Azure resource) to a test or test run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name of the App Component, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `body`: App Component model."]
        pub fn create_or_update_app_components(
            &self,
            name: impl Into<String>,
            body: impl Into<models::AppComponentsMap>,
        ) -> create_or_update_app_components::Builder {
            create_or_update_app_components::Builder {
                client: self.0.clone(),
                name: name.into(),
                body: body.into(),
            }
        }
        #[doc = "Delete an App Component."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name of the App Component, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn delete_app_component(&self, name: impl Into<String>) -> delete_app_component::Builder {
            delete_app_component::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get App Components for a test or a test run by its name."]
        pub fn get_app_component(&self) -> get_app_component::Builder {
            get_app_component::Builder {
                client: self.0.clone(),
                test_run_id: None,
                test_id: None,
            }
        }
    }
    pub mod get_app_component_by_name {
        use super::models;
        type Response = models::AppComponentsMap;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/appcomponents/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AppComponentsMap = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_app_components {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AppComponentsMap),
            Created201(models::AppComponentsMap),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) body: models::AppComponentsMap,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/appcomponents/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        req.insert_header("content-type", "application/merge-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AppComponentsMap = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AppComponentsMap = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod delete_app_component {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/appcomponents/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
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
    pub mod get_app_component {
        use super::models;
        type Response = models::AppComponentsMap;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: Option<String>,
            pub(crate) test_id: Option<String>,
        }
        impl Builder {
            #[doc = "[Required, if testId is not provided] Test run Id."]
            pub fn test_run_id(mut self, test_run_id: impl Into<String>) -> Self {
                self.test_run_id = Some(test_run_id.into());
                self
            }
            #[doc = "Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
            pub fn test_id(mut self, test_id: impl Into<String>) -> Self {
                self.test_id = Some(test_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/appcomponents", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(test_run_id) = &this.test_run_id {
                            req.url_mut().query_pairs_mut().append_pair("testRunId", test_run_id);
                        }
                        if let Some(test_id) = &this.test_id {
                            req.url_mut().query_pairs_mut().append_pair("testId", test_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AppComponentsMap = serde_json::from_slice(&rsp_body)?;
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
pub mod server_metrics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get server metrics configuration by its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name for server metrics, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn get_server_metrics_by_name(&self, name: impl Into<String>) -> get_server_metrics_by_name::Builder {
            get_server_metrics_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Configure server metrics for a test or test run"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name for server metrics, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `body`: Server metrics configuration model"]
        pub fn create_or_update_server_metrics_config(
            &self,
            name: impl Into<String>,
            body: impl Into<models::ServerMetricsModel>,
        ) -> create_or_update_server_metrics_config::Builder {
            create_or_update_server_metrics_config::Builder {
                client: self.0.clone(),
                name: name.into(),
                body: body.into(),
            }
        }
        #[doc = "Delete server metrics configuration by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Unique name for server metrics, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn delete_server_metrics(&self, name: impl Into<String>) -> delete_server_metrics::Builder {
            delete_server_metrics::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get server metrics configuration for a test or test run by its name."]
        pub fn get_server_metrics(&self) -> get_server_metrics::Builder {
            get_server_metrics::Builder {
                client: self.0.clone(),
                test_run_id: None,
                test_id: None,
            }
        }
        #[doc = "Get all default server metrics configuration for supported resource types."]
        pub fn get_server_default_metrics(&self) -> get_server_default_metrics::Builder {
            get_server_default_metrics::Builder { client: self.0.clone() }
        }
        #[doc = "Get all supported resource types for App Components(Azure resource types)."]
        pub fn list_supported_resource_type(&self) -> list_supported_resource_type::Builder {
            list_supported_resource_type::Builder { client: self.0.clone() }
        }
    }
    pub mod get_server_metrics_by_name {
        use super::models;
        type Response = models::ServerMetricsModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/serverMetricsConfig/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServerMetricsModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_server_metrics_config {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ServerMetricsModel),
            Created201(models::ServerMetricsModel),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) body: models::ServerMetricsModel,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/serverMetricsConfig/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        req.insert_header("content-type", "application/merge-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServerMetricsModel = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServerMetricsModel = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod delete_server_metrics {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/serverMetricsConfig/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
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
    pub mod get_server_metrics {
        use super::models;
        type Response = models::ServerMetricsModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: Option<String>,
            pub(crate) test_id: Option<String>,
        }
        impl Builder {
            #[doc = "[Required, if testId is not provided] Test run Id."]
            pub fn test_run_id(mut self, test_run_id: impl Into<String>) -> Self {
                self.test_run_id = Some(test_run_id.into());
                self
            }
            #[doc = "Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
            pub fn test_id(mut self, test_id: impl Into<String>) -> Self {
                self.test_id = Some(test_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/serverMetricsConfig", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(test_run_id) = &this.test_run_id {
                            req.url_mut().query_pairs_mut().append_pair("testRunId", test_run_id);
                        }
                        if let Some(test_id) = &this.test_id {
                            req.url_mut().query_pairs_mut().append_pair("testId", test_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServerMetricsModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_server_default_metrics {
        use super::models;
        type Response = models::DefaultServerMetricsConfigListModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/serverMetricsConfig/default", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DefaultServerMetricsConfigListModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_supported_resource_type {
        use super::models;
        type Response = models::SupportedResourceType;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/serverMetricsConfig/supportedResourceTypes", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SupportedResourceType = serde_json::from_slice(&rsp_body)?;
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
pub mod test {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get load test details by test name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn get_load_test(&self, test_id: impl Into<String>) -> get_load_test::Builder {
            get_load_test::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
            }
        }
        #[doc = "Create a new test or Update an existing test."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `body`: Load test model"]
        pub fn create_or_update_test(
            &self,
            test_id: impl Into<String>,
            body: impl Into<models::TestModel>,
        ) -> create_or_update_test::Builder {
            create_or_update_test::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
                body: body.into(),
            }
        }
        #[doc = "Delete a test by its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn delete_load_test(&self, test_id: impl Into<String>) -> delete_load_test::Builder {
            delete_load_test::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
            }
        }
        #[doc = "Get all load tests by the fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}."]
        pub fn list_load_test_search(&self) -> list_load_test_search::Builder {
            list_load_test_search::Builder {
                client: self.0.clone(),
                order_by: None,
                search: None,
                last_updated_start_time: None,
                last_updated_end_time: None,
                continuation_token: None,
                max_page_size: None,
            }
        }
        #[doc = "Get test file by the file name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `file_id`: Unique identifier for test file, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn get_test_file(&self, test_id: impl Into<String>, file_id: impl Into<String>) -> get_test_file::Builder {
            get_test_file::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
                file_id: file_id.into(),
            }
        }
        #[doc = "Upload input file for a given test name. File size can't be more than 50 MB. Existing file with same name for the given test will be overwritten. File should be provided in the request body as multipart/form-data."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `file_id`: Unique identifier for test file, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `file`: The file to be uploaded."]
        pub fn upload_test_file(
            &self,
            test_id: impl Into<String>,
            file_id: impl Into<String>,
            file: impl Into<bytes::Bytes>,
        ) -> upload_test_file::Builder {
            upload_test_file::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
                file_id: file_id.into(),
                file: file.into(),
                file_type: None,
            }
        }
        #[doc = "Delete file by the file name for a test."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `file_id`: Unique identifier for test file, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn delete_test_file(&self, test_id: impl Into<String>, file_id: impl Into<String>) -> delete_test_file::Builder {
            delete_test_file::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
                file_id: file_id.into(),
            }
        }
        #[doc = "Get all test files."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_id`: Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn get_all_test_files(&self, test_id: impl Into<String>) -> get_all_test_files::Builder {
            get_all_test_files::Builder {
                client: self.0.clone(),
                test_id: test_id.into(),
                continuation_token: None,
            }
        }
    }
    pub mod get_load_test {
        use super::models;
        type Response = models::TestModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/loadtests/{}", this.client.endpoint(), &this.test_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_test {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::TestModel),
            Created201(models::TestModel),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
            pub(crate) body: models::TestModel,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/loadtests/{}", this.client.endpoint(), &this.test_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        req.insert_header("content-type", "application/merge-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestModel = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestModel = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod delete_load_test {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/loadtests/{}", this.client.endpoint(), &this.test_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
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
    pub mod list_load_test_search {
        use super::models;
        type Response = models::TestModelResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) order_by: Option<String>,
            pub(crate) search: Option<String>,
            pub(crate) last_updated_start_time: Option<time::OffsetDateTime>,
            pub(crate) last_updated_end_time: Option<time::OffsetDateTime>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) max_page_size: Option<i32>,
        }
        impl Builder {
            #[doc = "Sort on one of the field - lastModifiedDateTime, displayName, createdBy in (field asc/desc) format. eg: displayName asc."]
            pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
                self.order_by = Some(order_by.into());
                self
            }
            #[doc = "Filter search based on searchable fields - testId, createdBy."]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "Start DateTime(ISO 8601 literal format) of the last updated time range to filter tests."]
            pub fn last_updated_start_time(mut self, last_updated_start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.last_updated_start_time = Some(last_updated_start_time.into());
                self
            }
            #[doc = "End DateTime(ISO 8601 literal format) of the last updated time range to filter tests."]
            pub fn last_updated_end_time(mut self, last_updated_end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.last_updated_end_time = Some(last_updated_end_time.into());
                self
            }
            #[doc = "Continuation token to get the next page of response."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Number of results in response."]
            pub fn max_page_size(mut self, max_page_size: i32) -> Self {
                self.max_page_size = Some(max_page_size);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/loadtests/sortAndFilter", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(order_by) = &this.order_by {
                            req.url_mut().query_pairs_mut().append_pair("orderBy", order_by);
                        }
                        if let Some(search) = &this.search {
                            req.url_mut().query_pairs_mut().append_pair("search", search);
                        }
                        if let Some(last_updated_start_time) = &this.last_updated_start_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("lastUpdatedStartTime", &last_updated_start_time.to_string());
                        }
                        if let Some(last_updated_end_time) = &this.last_updated_end_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("lastUpdatedEndTime", &last_updated_end_time.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(max_page_size) = &this.max_page_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxPageSize", &max_page_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestModelResourceList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_test_file {
        use super::models;
        type Response = models::FileUrl;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
            pub(crate) file_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/loadtests/{}/files/{}",
                            this.client.endpoint(),
                            &this.test_id,
                            &this.file_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileUrl = serde_json::from_slice(&rsp_body)?;
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
    pub mod upload_test_file {
        use super::models;
        type Response = models::FileUrl;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
            pub(crate) file_id: String,
            pub(crate) file: bytes::Bytes,
            pub(crate) file_type: Option<i32>,
        }
        impl Builder {
            #[doc = "Integer representation of the file type (0 = JMX_FILE, 1 = USER_PROPERTIES, 2 = ADDITIONAL_ARTIFACTS)."]
            pub fn file_type(mut self, file_type: i32) -> Self {
                self.file_type = Some(file_type);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/loadtests/{}/files/{}",
                            this.client.endpoint(),
                            &this.test_id,
                            &this.file_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(file_type) = &this.file_type {
                            req.url_mut().query_pairs_mut().append_pair("fileType", &file_type.to_string());
                        }
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileUrl = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_test_file {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
            pub(crate) file_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/loadtests/{}/files/{}",
                            this.client.endpoint(),
                            &this.test_id,
                            &this.file_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
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
    pub mod get_all_test_files {
        use super::models;
        type Response = models::FileUrlList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_id: String,
            pub(crate) continuation_token: Option<String>,
        }
        impl Builder {
            #[doc = "Continuation token to get the next page of response."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/loadtests/{}/files", this.client.endpoint(), &this.test_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileUrlList = serde_json::from_slice(&rsp_body)?;
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
pub mod test_run {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get test run details by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of load test run, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn get_test_run(&self, test_run_id: impl Into<String>) -> get_test_run::Builder {
            get_test_run::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
            }
        }
        #[doc = "Create and start a new test run with the given name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of the load test run, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `body`: Load test run model"]
        pub fn create_and_update_test(
            &self,
            test_run_id: impl Into<String>,
            body: impl Into<models::TestRunModel>,
        ) -> create_and_update_test::Builder {
            create_and_update_test::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
                body: body.into(),
                old_test_run_id: None,
            }
        }
        #[doc = "Delete a test run by its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of the load test run, must be a valid URL character ^[a-z0-9_-]*$."]
        pub fn delete_test_run(&self, test_run_id: impl Into<String>) -> delete_test_run::Builder {
            delete_test_run::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
            }
        }
        #[doc = "Get test run file by file name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of load test run, must be a valid URL character ^[a-z0-9_-]*$"]
        #[doc = "* `file_id`: Unique identifier for test run file, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn get_test_run_file(&self, test_run_id: impl Into<String>, file_id: impl Into<String>) -> get_test_run_file::Builder {
            get_test_run_file::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
                file_id: file_id.into(),
            }
        }
        #[doc = "Get all test runs with given filters"]
        pub fn list_test_runs_search(&self) -> list_test_runs_search::Builder {
            list_test_runs_search::Builder {
                client: self.0.clone(),
                order_by: None,
                continuation_token: None,
                search: None,
                execution_from: None,
                execution_to: None,
                status: None,
                max_page_size: None,
                test_id: None,
            }
        }
        #[doc = "Stop test run by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of the load test run, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn stop_test_run(&self, test_run_id: impl Into<String>) -> stop_test_run::Builder {
            stop_test_run::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
            }
        }
        #[doc = "Get all client metrics for a load test run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name of the load test run, must be a valid URL character ^[a-z0-9_-]*$."]
        #[doc = "* `body`: Client metrics request model"]
        pub fn get_test_run_client_metrics(
            &self,
            test_run_id: impl Into<String>,
            body: impl Into<models::ClientMetricsRequestModel>,
        ) -> get_test_run_client_metrics::Builder {
            get_test_run_client_metrics::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
                body: body.into(),
            }
        }
        #[doc = "Get all filters that are supported for client metrics for a given load test run"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `test_run_id`: Unique name for load test run, must be a valid URL character ^[a-z0-9_-]*$"]
        pub fn get_test_run_client_metrics_filters(&self, test_run_id: impl Into<String>) -> get_test_run_client_metrics_filters::Builder {
            get_test_run_client_metrics_filters::Builder {
                client: self.0.clone(),
                test_run_id: test_run_id.into(),
            }
        }
    }
    pub mod get_test_run {
        use super::models;
        type Response = models::TestRunModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/testruns/{}", this.client.endpoint(), &this.test_run_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_and_update_test {
        use super::models;
        type Response = models::TestRunModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
            pub(crate) body: models::TestRunModel,
            pub(crate) old_test_run_id: Option<String>,
        }
        impl Builder {
            #[doc = "Existing test run Id that should be rerun."]
            pub fn old_test_run_id(mut self, old_test_run_id: impl Into<String>) -> Self {
                self.old_test_run_id = Some(old_test_run_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/testruns/{}", this.client.endpoint(), &this.test_run_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(old_test_run_id) = &this.old_test_run_id {
                            req.url_mut().query_pairs_mut().append_pair("oldTestRunId", old_test_run_id);
                        }
                        req.insert_header("content-type", "application/merge-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_test_run {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/testruns/{}", this.client.endpoint(), &this.test_run_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
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
    pub mod get_test_run_file {
        use super::models;
        type Response = models::FileUrl;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
            pub(crate) file_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/testruns/{}/files/{}",
                            this.client.endpoint(),
                            &this.test_run_id,
                            &this.file_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileUrl = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_test_runs_search {
        use super::models;
        type Response = models::TestRunModelResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) order_by: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) search: Option<String>,
            pub(crate) execution_from: Option<time::OffsetDateTime>,
            pub(crate) execution_to: Option<time::OffsetDateTime>,
            pub(crate) status: Option<String>,
            pub(crate) max_page_size: Option<i32>,
            pub(crate) test_id: Option<String>,
        }
        impl Builder {
            #[doc = "Sort on one of the field - status, displayName, executedDateTime in (field asc/desc) format. eg: displayName asc."]
            pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
                self.order_by = Some(order_by.into());
                self
            }
            #[doc = "Continuation token to get the next page of response."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Filter search based on searchable fields - description, executedUser."]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "The end DateTime(ISO 8601 literal format) of test-run execution time filter range."]
            pub fn execution_from(mut self, execution_from: impl Into<time::OffsetDateTime>) -> Self {
                self.execution_from = Some(execution_from.into());
                self
            }
            #[doc = "The start DateTime(ISO 8601 literal format) of test-run execution time filter range."]
            pub fn execution_to(mut self, execution_to: impl Into<time::OffsetDateTime>) -> Self {
                self.execution_to = Some(execution_to.into());
                self
            }
            #[doc = "Comma separated list of test run status, value can be -  \"ACCEPTED\", \"NOTSTARTED\",\"PROVISIONING\",\"PROVISIONED\",\"CONFIGURING\",\n\"CONFIGURED\",\"EXECUTING\",\"EXECUTED\",\"DEPROVISIONING\",\"DEPROVISIONED\",\"DONE\",\"CANCELLED\",\"FAILED\"."]
            pub fn status(mut self, status: impl Into<String>) -> Self {
                self.status = Some(status.into());
                self
            }
            #[doc = "Number of results in response."]
            pub fn max_page_size(mut self, max_page_size: i32) -> Self {
                self.max_page_size = Some(max_page_size);
                self
            }
            #[doc = "Unique name for load test, must be a valid URL character ^[a-z0-9_-]*$."]
            pub fn test_id(mut self, test_id: impl Into<String>) -> Self {
                self.test_id = Some(test_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/testruns/sortAndFilter", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        if let Some(order_by) = &this.order_by {
                            req.url_mut().query_pairs_mut().append_pair("orderBy", order_by);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(search) = &this.search {
                            req.url_mut().query_pairs_mut().append_pair("search", search);
                        }
                        if let Some(execution_from) = &this.execution_from {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("executionFrom", &execution_from.to_string());
                        }
                        if let Some(execution_to) = &this.execution_to {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("executionTo", &execution_to.to_string());
                        }
                        if let Some(status) = &this.status {
                            req.url_mut().query_pairs_mut().append_pair("status", status);
                        }
                        if let Some(max_page_size) = &this.max_page_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("maxPageSize", &max_page_size.to_string());
                        }
                        if let Some(test_id) = &this.test_id {
                            req.url_mut().query_pairs_mut().append_pair("testId", test_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunModelResourceList = serde_json::from_slice(&rsp_body)?;
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
    pub mod stop_test_run {
        use super::models;
        type Response = models::TestRunModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/testruns/{}:stop", this.client.endpoint(), &this.test_run_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TestRunModel = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_test_run_client_metrics {
        use super::models;
        type Response = models::ClientMetricsResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
            pub(crate) body: models::ClientMetricsRequestModel,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/testruns/{}/clientMetrics", this.client.endpoint(), &this.test_run_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ClientMetricsResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_test_run_client_metrics_filters {
        use super::models;
        type Response = models::ClientMetricsFilters;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) test_run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/testruns/{}/clientMetricsFilters",
                            this.client.endpoint(),
                            &this.test_run_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ClientMetricsFilters = serde_json::from_slice(&rsp_body)?;
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
