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
    pub fn model_settings_client(&self) -> model_settings::Client {
        model_settings::Client(self.clone())
    }
    pub fn query_client(&self) -> query::Client {
        query::Client(self.clone())
    }
    pub fn time_series_hierarchies_client(&self) -> time_series_hierarchies::Client {
        time_series_hierarchies::Client(self.clone())
    }
    pub fn time_series_instances_client(&self) -> time_series_instances::Client {
        time_series_instances::Client(self.clone())
    }
    pub fn time_series_types_client(&self) -> time_series_types::Client {
        time_series_types::Client(self.clone())
    }
}
pub mod query {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the time range and distribution of event count over the event timestamp ($ts). This API can be used to provide landing experience of navigating to the environment."]
        pub fn get_availability(&self) -> get_availability::Builder {
            get_availability::Builder {
                client: self.0.clone(),
                store_type: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Returns environment event schema for a given search span. Event schema is a set of property definitions. Event schema may not be contain all persisted properties when there are too many properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Parameters to get event schema."]
        pub fn get_event_schema(&self, parameters: impl Into<models::GetEventSchemaRequest>) -> get_event_schema::Builder {
            get_event_schema::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                store_type: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Executes Time Series Query in pages of results - Get Events, Get Series or Aggregate Series."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series query request body."]
        pub fn execute(&self, parameters: impl Into<models::QueryRequest>) -> execute::Builder {
            execute::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                store_type: None,
                x_ms_continuation: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
    }
    pub mod get_availability {
        use super::models;
        type Response = models::AvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) store_type: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "For the environments with warm store enabled, the query can be executed either on the 'WarmStore' or 'ColdStore'. This parameter in the query defines which store the query should be executed on. If not defined, the query will be executed on the cold store."]
            pub fn store_type(mut self, store_type: impl Into<String>) -> Self {
                self.store_type = Some(store_type.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/availability", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(store_type) = &this.store_type {
                            req.url_mut().query_pairs_mut().append_pair("storeType", store_type);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_event_schema {
        use super::models;
        type Response = models::EventSchema;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::GetEventSchemaRequest,
            pub(crate) store_type: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "For the environments with warm store enabled, the query can be executed either on the 'WarmStore' or 'ColdStore'. This parameter in the query defines which store the query should be executed on. If not defined, the query will be executed on the cold store."]
            pub fn store_type(mut self, store_type: impl Into<String>) -> Self {
                self.store_type = Some(store_type.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/eventSchema", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(store_type) = &this.store_type {
                            req.url_mut().query_pairs_mut().append_pair("storeType", store_type);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventSchema = serde_json::from_slice(&rsp_body)?;
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
    pub mod execute {
        use super::models;
        type Response = models::QueryResultPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::QueryRequest,
            pub(crate) store_type: Option<String>,
            pub(crate) x_ms_continuation: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "For the environments with warm store enabled, the query can be executed either on the 'WarmStore' or 'ColdStore'. This parameter in the query defines which store the query should be executed on. If not defined, the query will be executed on the cold store."]
            pub fn store_type(mut self, store_type: impl Into<String>) -> Self {
                self.store_type = Some(store_type.into());
                self
            }
            #[doc = "Continuation token from previous page of results to retrieve the next page of the results in calls that support pagination. To get the first page results, specify null continuation token as parameter value. Returned continuation token is null if all results have been returned, and there is no next page of results."]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/query", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(store_type) = &this.store_type {
                            req.url_mut().query_pairs_mut().append_pair("storeType", store_type);
                        }
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryResultPage = serde_json::from_slice(&rsp_body)?;
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
pub mod model_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the model settings which includes model display name, Time Series ID properties and default type ID. Every Gen2 environment has a model that is automatically created."]
        pub fn get(&self) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Updates time series model settings - either the model name or default type ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Model settings update request body."]
        pub fn update(&self, parameters: impl Into<models::UpdateModelSettingsRequest>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ModelSettingsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/modelSettings", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ModelSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = models::ModelSettingsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::UpdateModelSettingsRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/modelSettings", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ModelSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod time_series_instances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets time series instances in pages."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                x_ms_continuation: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Executes a batch get, create, update, delete operation on multiple time series instances."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series instances suggest request body."]
        pub fn execute_batch(&self, parameters: impl Into<models::InstancesBatchRequest>) -> execute_batch::Builder {
            execute_batch::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Suggests keywords based on time series instance attributes to be later used in Search Instances."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series instances suggest request body."]
        pub fn suggest(&self, parameters: impl Into<models::InstancesSuggestRequest>) -> suggest::Builder {
            suggest::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Partial list of hits on search for time series instances based on instance attributes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series instances search request body."]
        pub fn search(&self, parameters: impl Into<models::SearchInstancesRequest>) -> search::Builder {
            search::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_continuation: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GetInstancesPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_continuation: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Continuation token from previous page of results to retrieve the next page of the results in calls that support pagination. To get the first page results, specify null continuation token as parameter value. Returned continuation token is null if all results have been returned, and there is no next page of results."]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/instances", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GetInstancesPage = serde_json::from_slice(&rsp_body)?;
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
    pub mod execute_batch {
        use super::models;
        type Response = models::InstancesBatchResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::InstancesBatchRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/instances/$batch", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::InstancesBatchResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod suggest {
        use super::models;
        type Response = models::InstancesSuggestResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::InstancesSuggestRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/instances/suggest", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::InstancesSuggestResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod search {
        use super::models;
        type Response = models::SearchInstancesResponsePage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::SearchInstancesRequest,
            pub(crate) x_ms_continuation: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Continuation token from previous page of results to retrieve the next page of the results in calls that support pagination. To get the first page results, specify null continuation token as parameter value. Returned continuation token is null if all results have been returned, and there is no next page of results."]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/instances/search", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SearchInstancesResponsePage = serde_json::from_slice(&rsp_body)?;
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
pub mod time_series_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets time series types in pages."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                x_ms_continuation: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Executes a batch get, create, update, delete operation on multiple time series types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series types batch request body."]
        pub fn execute_batch(&self, parameters: impl Into<models::TypesBatchRequest>) -> execute_batch::Builder {
            execute_batch::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GetTypesPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_continuation: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Continuation token from previous page of results to retrieve the next page of the results in calls that support pagination. To get the first page results, specify null continuation token as parameter value. Returned continuation token is null if all results have been returned, and there is no next page of results."]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/types", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GetTypesPage = serde_json::from_slice(&rsp_body)?;
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
    pub mod execute_batch {
        use super::models;
        type Response = models::TypesBatchResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::TypesBatchRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/types/$batch", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TypesBatchResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod time_series_hierarchies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns time series hierarchies definitions in pages."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                x_ms_continuation: None,
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
        #[doc = "Executes a batch get, create, update, delete operation on multiple time series hierarchy definitions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: Time series hierarchies batch request body."]
        pub fn execute_batch(&self, parameters: impl Into<models::HierarchiesBatchRequest>) -> execute_batch::Builder {
            execute_batch::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                x_ms_client_request_id: None,
                x_ms_client_session_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::GetHierarchiesPage;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_continuation: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Continuation token from previous page of results to retrieve the next page of the results in calls that support pagination. To get the first page results, specify null continuation token as parameter value. Returned continuation token is null if all results have been returned, and there is no next page of results."]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/hierarchies", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GetHierarchiesPage = serde_json::from_slice(&rsp_body)?;
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
    pub mod execute_batch {
        use super::models;
        type Response = models::HierarchiesBatchResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::HierarchiesBatchRequest,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_client_session_id: Option<String>,
        }
        impl Builder {
            #[doc = "Optional client request ID. Service records this value. Allows the service to trace operation across services, and allows the customer to contact support regarding a particular request."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional client session ID. Service records this value. Allows the service to trace a group of related operations across services, and allows the customer to contact support regarding a particular group of requests."]
            pub fn x_ms_client_session_id(mut self, x_ms_client_session_id: impl Into<String>) -> Self {
                self.x_ms_client_session_id = Some(x_ms_client_session_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/timeseries/hierarchies/$batch", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_client_session_id) = &this.x_ms_client_session_id {
                            req.insert_header("x-ms-client-session-id", x_ms_client_session_id);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HierarchiesBatchResponse = serde_json::from_slice(&rsp_body)?;
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
