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
    pub fn azure_traffic_collectors_client(&self) -> azure_traffic_collectors::Client {
        azure_traffic_collectors::Client(self.clone())
    }
    pub fn azure_traffic_collectors_by_resource_group_client(&self) -> azure_traffic_collectors_by_resource_group::Client {
        azure_traffic_collectors_by_resource_group::Client(self.clone())
    }
    pub fn azure_traffic_collectors_by_subscription_client(&self) -> azure_traffic_collectors_by_subscription::Client {
        azure_traffic_collectors_by_subscription::Client(self.clone())
    }
    pub fn collector_policies_client(&self) -> collector_policies::Client {
        collector_policies::Client(self.clone())
    }
    pub fn network_function_client(&self) -> network_function::Client {
        network_function::Client(self.clone())
    }
}
pub mod network_function {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available NetworkFunction Rest API operations."]
        pub fn list_operations(&self) -> list_operations::Builder {
            list_operations::Builder { client: self.0.clone() }
        }
    }
    pub mod list_operations {
        use super::models;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.NetworkFunction/operations",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod azure_traffic_collectors_by_subscription {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return list of Azure Traffic Collectors in a subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AzureTrafficCollectorListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollectorListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod azure_traffic_collectors_by_resource_group {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return list of Azure Traffic Collectors in a Resource Group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list(&self, resource_group_name: impl Into<String>, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AzureTrafficCollectorListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollectorListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod azure_traffic_collectors {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the specified Azure Traffic Collector in a specified resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
            }
        }
        #[doc = "Creates or updates a Azure Traffic Collector resource"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        #[doc = "* `parameters`: The parameters to provide for the created Azure Traffic Collector."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
            parameters: impl Into<models::AzureTrafficCollector>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Updates the specified Azure Traffic Collector tags."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `parameters`: Parameters supplied to update Azure Traffic Collector tags."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        pub fn update_tags(
            &self,
            resource_group_name: impl Into<String>,
            parameters: impl Into<models::TagsObject>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
        ) -> update_tags::Builder {
            update_tags::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
            }
        }
        #[doc = "Deletes a specified Azure Traffic Collector resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::AzureTrafficCollector;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.azure_traffic_collector_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollector = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AzureTrafficCollector),
            Created201(models::AzureTrafficCollector),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
            pub(crate) parameters: models::AzureTrafficCollector,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.azure_traffic_collector_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollector = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollector = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_tags {
        use super::models;
        type Response = models::AzureTrafficCollector;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) parameters: models::TagsObject,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.azure_traffic_collector_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureTrafficCollector = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.azure_traffic_collector_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
pub mod collector_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return list of Collector policies in a Azure Traffic Collector"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        pub fn list(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
            }
        }
        #[doc = "Gets the collector policy in a specified Traffic Collector"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        #[doc = "* `collector_policy_name`: Collector Policy Name"]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
            collector_policy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
                collector_policy_name: collector_policy_name.into(),
            }
        }
        #[doc = "Creates or updates a Collector Policy resource"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        #[doc = "* `collector_policy_name`: Collector Policy Name"]
        #[doc = "* `parameters`: The parameters to provide for the created Collector Policy."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
            collector_policy_name: impl Into<String>,
            parameters: impl Into<models::CollectorPolicy>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
                collector_policy_name: collector_policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes a specified Collector Policy resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `azure_traffic_collector_name`: Azure Traffic Collector name"]
        #[doc = "* `collector_policy_name`: Collector Policy Name"]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            azure_traffic_collector_name: impl Into<String>,
            collector_policy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                azure_traffic_collector_name: azure_traffic_collector_name.into(),
                collector_policy_name: collector_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CollectorPolicyListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}/collectorPolicies" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . azure_traffic_collector_name)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectorPolicyListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::CollectorPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
            pub(crate) collector_policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}/collectorPolicies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . azure_traffic_collector_name , & this . collector_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectorPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::CollectorPolicy),
            Created201(models::CollectorPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
            pub(crate) collector_policy_name: String,
            pub(crate) parameters: models::CollectorPolicy,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}/collectorPolicies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . azure_traffic_collector_name , & this . collector_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectorPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectorPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) azure_traffic_collector_name: String,
            pub(crate) collector_policy_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetworkFunction/azureTrafficCollectors/{}/collectorPolicies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . azure_traffic_collector_name , & this . collector_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
