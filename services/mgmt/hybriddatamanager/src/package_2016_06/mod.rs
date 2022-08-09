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
    pub fn data_managers_client(&self) -> data_managers::Client {
        data_managers::Client(self.clone())
    }
    pub fn data_services_client(&self) -> data_services::Client {
        data_services::Client(self.clone())
    }
    pub fn data_store_types_client(&self) -> data_store_types::Client {
        data_store_types::Client(self.clone())
    }
    pub fn data_stores_client(&self) -> data_stores::Client {
        data_stores::Client(self.clone())
    }
    pub fn job_definitions_client(&self) -> job_definitions::Client {
        job_definitions::Client(self.clone())
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn public_keys_client(&self) -> public_keys::Client {
        public_keys::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets all the operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AvailableProviderOperations;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.HybridData/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AvailableProviderOperations = serde_json::from_slice(&rsp_body)?;
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
pub mod data_managers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the data manager resources available under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all the data manager resources available under the given resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Gets information about the specified data manager resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Creates a new data manager resource with the specified parameters. Existing resources cannot be updated with this API\r\nand should instead be updated with the Update data manager resource API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `data_manager`: Data manager resource details from request body."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
            data_manager: impl Into<models::DataManager>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                data_manager: data_manager.into(),
            }
        }
        #[doc = "Updates the properties of an existing data manager resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        #[doc = "* `data_manager_update_parameter`: Data manager resource details from request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
            data_manager_update_parameter: impl Into<models::DataManagerUpdateParameter>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                data_manager_update_parameter: data_manager_update_parameter.into(),
                if_match: None,
            }
        }
        #[doc = "Deletes a data manager resource in Microsoft Azure."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::DataManagerList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.HybridData/dataManagers",
                            this.client.endpoint(),
                            &this.subscription_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataManagerList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_resource_group {
        use super::models;
        type Response = models::DataManagerList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataManagerList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get {
        use super::models;
        type Response = models::DataManager;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataManager = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DataManager),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) data_manager: models::DataManager,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.data_manager)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataManager = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DataManager),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) data_manager_update_parameter: models::DataManagerUpdateParameter,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "Defines the If-Match condition. The patch will be performed only if the ETag of the data manager resource on the server matches this value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.data_manager_update_parameter)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataManager = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
pub mod data_services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets all the data services."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Gets the data service that match the data service name given."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service that is being queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            data_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
    }
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::DataServiceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataServiceList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::DataService;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataService = serde_json::from_slice(&rsp_body)?;
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
pub mod job_definitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets all the job definitions of the given data service name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The data service type of interest."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_service(
            &self,
            data_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_service::Builder {
            list_by_data_service::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
        #[doc = "This method gets job definition object by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The data service name of the job definition"]
        #[doc = "* `job_definition_name`: The job definition name that is being queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Creates or updates a job definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The data service type of the job definition."]
        #[doc = "* `job_definition_name`: The job definition name to be created or updated."]
        #[doc = "* `job_definition`: Job Definition object to be created or updated."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn create_or_update(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            job_definition: impl Into<models::JobDefinition>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                job_definition: job_definition.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method deletes the given job definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The data service type of the job definition."]
        #[doc = "* `job_definition_name`: The job definition name to be deleted."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn delete(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method runs a job instance of the given job definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The data service type of the job definition."]
        #[doc = "* `job_definition_name`: Name of the job definition."]
        #[doc = "* `run_parameters`: Run time parameters for the job definition."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn run(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            run_parameters: impl Into<models::RunParameters>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> run::Builder {
            run::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                run_parameters: run_parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method gets all the job definitions of the given data manager resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
    }
    pub mod list_by_data_service {
        use super::models;
        type Response = models::JobDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobDefinitionList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::JobDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobDefinition = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::JobDefinition),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) job_definition: models::JobDefinition,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.job_definition)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobDefinition = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
    pub mod run {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) run_parameters: models::RunParameters,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}/run" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.run_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::JobDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/jobDefinitions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobDefinitionList = serde_json::from_slice(&rsp_body)?;
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
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets all the jobs of a given job definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service of the job definition."]
        #[doc = "* `job_definition_name`: The name of the job definition for which jobs are needed."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_job_definition(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_job_definition::Builder {
            list_by_job_definition::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
        #[doc = "This method gets a data manager job given the jobId."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service of the job definition."]
        #[doc = "* `job_definition_name`: The name of the job definition of the job."]
        #[doc = "* `job_id`: The job id of the job queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            job_id: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                job_id: job_id.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                expand: None,
            }
        }
        #[doc = "Cancels the given job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service of the job definition."]
        #[doc = "* `job_definition_name`: The name of the job definition of the job."]
        #[doc = "* `job_id`: The job id of the job queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn cancel(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            job_id: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                job_id: job_id.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Resumes the given job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service of the job definition."]
        #[doc = "* `job_definition_name`: The name of the job definition of the job."]
        #[doc = "* `job_id`: The job id of the job queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn resume(
            &self,
            data_service_name: impl Into<String>,
            job_definition_name: impl Into<String>,
            job_id: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> resume::Builder {
            resume::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                job_definition_name: job_definition_name.into(),
                job_id: job_id.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method gets all the jobs of a data service type in a given resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_service_name`: The name of the data service of interest."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_service(
            &self,
            data_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_service::Builder {
            list_by_data_service::Builder {
                client: self.0.clone(),
                data_service_name: data_service_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
        #[doc = "This method gets all the jobs at the data manager resource level."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
    }
    pub mod list_by_job_definition {
        use super::models;
        type Response = models::JobList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}/jobs" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Job;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) job_id: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "$expand is supported on details parameter for job, which provides details on the job stages."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}/jobs/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name , & this . job_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Job = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) job_id: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}/jobs/{}/cancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name , & this . job_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
    pub mod resume {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) job_definition_name: String,
            pub(crate) job_id: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobDefinitions/{}/jobs/{}/resume" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . data_manager_name , & this . data_service_name , & this . job_definition_name , & this . job_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
    pub mod list_by_data_service {
        use super::models;
        type Response = models::JobList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_service_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataServices/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::JobList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobList = serde_json::from_slice(&rsp_body)?;
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
pub mod data_stores {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the data stores/repositories in the given resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
                filter: None,
            }
        }
        #[doc = "This method gets the data store/repository by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_store_name`: The data store/repository name queried."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            data_store_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                data_store_name: data_store_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Creates or updates the data store/repository in the data manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_store_name`: The data store/repository name to be created or updated."]
        #[doc = "* `data_store`: The data store/repository object to be created or updated."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn create_or_update(
            &self,
            data_store_name: impl Into<String>,
            data_store: impl Into<models::DataStore>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                data_store_name: data_store_name.into(),
                data_store: data_store.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method deletes the given data store/repository."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_store_name`: The data store/repository name to be deleted."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn delete(
            &self,
            data_store_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                data_store_name: data_store_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
    }
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::DataStoreList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStores",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataStoreList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::DataStore;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_store_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStores/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_store_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataStore = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::DataStore),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_store_name: String,
            pub(crate) data_store: models::DataStore,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStores/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_store_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.data_store)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataStore = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_store_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStores/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_store_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
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
pub mod data_store_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the data store/repository types that the resource supports."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "Gets the data store/repository type given its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `data_store_type_name`: The data store/repository type name for which details are needed."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            data_store_type_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                data_store_type_name: data_store_type_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
    }
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::DataStoreTypeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStoreTypes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataStoreTypeList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::DataStoreType;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) data_store_type_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/dataStoreTypes/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.data_store_type_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataStoreType = serde_json::from_slice(&rsp_body)?;
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
pub mod public_keys {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This method gets the list view of public keys, however it will only have one element."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn list_by_data_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> list_by_data_manager::Builder {
            list_by_data_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
        #[doc = "This method gets the public keys."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `public_key_name`: Name of the public key."]
        #[doc = "* `subscription_id`: The Subscription Id"]
        #[doc = "* `resource_group_name`: The Resource Group Name"]
        #[doc = "* `data_manager_name`: The name of the DataManager Resource within the specified resource group. DataManager names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
        pub fn get(
            &self,
            public_key_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            data_manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                public_key_name: public_key_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                data_manager_name: data_manager_name.into(),
            }
        }
    }
    pub mod list_by_data_manager {
        use super::models;
        type Response = models::PublicKeyList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/publicKeys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PublicKeyList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::PublicKey;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) public_key_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) data_manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.HybridData/dataManagers/{}/publicKeys/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.data_manager_name,
                            &this.public_key_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PublicKey = serde_json::from_slice(&rsp_body)?;
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
