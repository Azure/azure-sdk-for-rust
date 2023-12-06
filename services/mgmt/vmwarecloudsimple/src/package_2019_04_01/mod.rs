#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: azure_core::Url,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<azure_core::Url>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
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
    pub fn endpoint(mut self, endpoint: impl Into<azure_core::Url>) -> Self {
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
    pub fn build(self) -> azure_core::Result<Client> {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = if let Some(scopes) = self.scopes {
            scopes
        } else {
            vec![endpoint.join(azure_core::auth::DEFAULT_SCOPE_SUFFIX)?.to_string()]
        };
        Ok(Client::new(endpoint, self.credential, scopes, self.options))
    }
}
impl Client {
    pub(crate) async fn bearer_token(&self) -> azure_core::Result<azure_core::auth::Secret> {
        let credential = self.token_credential();
        let response = credential.get_token(&self.scopes()).await?;
        Ok(response.token)
    }
    pub(crate) fn endpoint(&self) -> &azure_core::Url {
        &self.endpoint
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<azure_core::Url>,
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
    pub fn customization_policies_client(&self) -> customization_policies::Client {
        customization_policies::Client(self.clone())
    }
    pub fn dedicated_cloud_nodes_client(&self) -> dedicated_cloud_nodes::Client {
        dedicated_cloud_nodes::Client(self.clone())
    }
    pub fn dedicated_cloud_services_client(&self) -> dedicated_cloud_services::Client {
        dedicated_cloud_services::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn private_clouds_client(&self) -> private_clouds::Client {
        private_clouds::Client(self.clone())
    }
    pub fn resource_pools_client(&self) -> resource_pools::Client {
        resource_pools::Client(self.clone())
    }
    pub fn skus_availability_client(&self) -> skus_availability::Client {
        skus_availability::Client(self.clone())
    }
    pub fn usages_client(&self) -> usages::Client {
        usages::Client(self.clone())
    }
    pub fn virtual_machine_templates_client(&self) -> virtual_machine_templates::Client {
        virtual_machine_templates::Client(self.clone())
    }
    pub fn virtual_machines_client(&self) -> virtual_machines::Client {
        virtual_machines::Client(self.clone())
    }
    pub fn virtual_networks_client(&self) -> virtual_networks::Client {
        virtual_networks::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list of available operations"]
        #[doc = "Return list of operations"]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
        #[doc = "Implements get of async operation"]
        #[doc = "Return an async operation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `operation_id`: operation id"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            referer: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                referer: referer.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AvailableOperationsListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AvailableOperationsListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::AvailableOperationsListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/providers/Microsoft.VMwareCloudSimple/operations",
                    self.client.endpoint(),
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationResource = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) referer: String,
            pub(crate) operation_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/operationResults/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.operation_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::OperationResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::OperationResource>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod dedicated_cloud_nodes {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list of dedicated cloud nodes within subscription method"]
        #[doc = "Returns list of dedicate cloud nodes within subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements list of dedicated cloud nodes within RG method"]
        #[doc = "Returns list of dedicate cloud nodes within resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements dedicated cloud node GET method"]
        #[doc = "Returns dedicated cloud node"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_node_name`: dedicated cloud node name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_node_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_node_name: dedicated_cloud_node_name.into(),
            }
        }
        #[doc = "Implements dedicated cloud node PUT method"]
        #[doc = "Returns dedicated cloud node by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `dedicated_cloud_node_name`: dedicated cloud node name"]
        #[doc = "* `dedicated_cloud_node_request`: Create Dedicated Cloud Node request"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            referer: impl Into<String>,
            dedicated_cloud_node_name: impl Into<String>,
            dedicated_cloud_node_request: impl Into<models::DedicatedCloudNode>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                referer: referer.into(),
                dedicated_cloud_node_name: dedicated_cloud_node_name.into(),
                dedicated_cloud_node_request: dedicated_cloud_node_request.into(),
            }
        }
        #[doc = "Implements dedicated cloud node PATCH method"]
        #[doc = "Patches dedicated node properties"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_node_name`: dedicated cloud node name"]
        #[doc = "* `dedicated_cloud_node_request`: Patch Dedicated Cloud Node request"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_node_name: impl Into<String>,
            dedicated_cloud_node_request: impl Into<models::PatchPayload>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_node_name: dedicated_cloud_node_name.into(),
                dedicated_cloud_node_request: dedicated_cloud_node_request.into(),
            }
        }
        #[doc = "Implements dedicated cloud node DELETE method"]
        #[doc = "Delete dedicated cloud node"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_node_name`: dedicated cloud node name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_node_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_node_name: dedicated_cloud_node_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudNodeListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudNodeListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::DedicatedCloudNodeListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes",
                    self.client.endpoint(),
                    &self.subscription_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudNodeListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudNodeListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::DedicatedCloudNodeListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudNode> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudNode = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_node_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_node_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudNode>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudNode>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudNode> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudNode = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("azure-asyncoperation"))
            }
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) referer: String,
            pub(crate) dedicated_cloud_node_name: String,
            pub(crate) dedicated_cloud_node_request: models::DedicatedCloudNode,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.dedicated_cloud_node_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_node_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudNode>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudNode>>;
            #[doc = "Returns a future that polls the long running operation and checks for the state via `properties.provisioningState` in the response body."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{body_content::get_provisioning_state, get_retry_after, LroStatus},
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    loop {
                        let this = self.clone();
                        let response = this.send().await?;
                        let retry_after = get_retry_after(response.as_raw_response().headers());
                        let status = response.as_raw_response().status();
                        let body = response.into_body().await?;
                        let provisioning_state = get_provisioning_state(status, &body)?;
                        log::trace!("current provisioning_state: {provisioning_state:?}");
                        match provisioning_state {
                            LroStatus::Succeeded => return Ok(body),
                            LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                            LroStatus::Canceled => {
                                return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                            }
                            _ => {
                                sleep(retry_after).await;
                            }
                        }
                    }
                })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudNode> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudNode = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_node_name: String,
            pub(crate) dedicated_cloud_node_request: models::PatchPayload,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.dedicated_cloud_node_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_node_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudNode>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudNode>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_node_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudNodes/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_node_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
}
pub mod dedicated_cloud_services {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list of dedicatedCloudService objects within subscription method"]
        #[doc = "Returns list of dedicated cloud services within a subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements list of dedicatedCloudService objects within RG method"]
        #[doc = "Returns list of dedicated cloud services within a resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements dedicatedCloudService GET method"]
        #[doc = "Returns Dedicate Cloud Service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_service_name`: dedicated cloud Service name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_service_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_service_name: dedicated_cloud_service_name.into(),
            }
        }
        #[doc = "Implements dedicated cloud service PUT method"]
        #[doc = "Create dedicate cloud service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_service_name`: dedicated cloud Service name"]
        #[doc = "* `dedicated_cloud_service_request`: Create Dedicated Cloud Service request"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_service_name: impl Into<String>,
            dedicated_cloud_service_request: impl Into<models::DedicatedCloudService>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_service_name: dedicated_cloud_service_name.into(),
                dedicated_cloud_service_request: dedicated_cloud_service_request.into(),
            }
        }
        #[doc = "Implements dedicatedCloudService PATCH method"]
        #[doc = "Patch dedicated cloud service's properties"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_service_name`: dedicated cloud service name"]
        #[doc = "* `dedicated_cloud_service_request`: Patch Dedicated Cloud Service request"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_service_name: impl Into<String>,
            dedicated_cloud_service_request: impl Into<models::PatchPayload>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_service_name: dedicated_cloud_service_name.into(),
                dedicated_cloud_service_request: dedicated_cloud_service_request.into(),
            }
        }
        #[doc = "Implements dedicatedCloudService DELETE method"]
        #[doc = "Delete dedicate cloud service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `dedicated_cloud_service_name`: dedicated cloud service name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dedicated_cloud_service_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dedicated_cloud_service_name: dedicated_cloud_service_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudServiceListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudServiceListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::DedicatedCloudServiceListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices",
                    self.client.endpoint(),
                    &self.subscription_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudServiceListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudServiceListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::DedicatedCloudServiceListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudService> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_service_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_service_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudService>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudService>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudService> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_service_name: String,
            pub(crate) dedicated_cloud_service_request: models::DedicatedCloudService,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.dedicated_cloud_service_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_service_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudService>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudService>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DedicatedCloudService> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DedicatedCloudService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_service_name: String,
            pub(crate) dedicated_cloud_service_request: models::PatchPayload,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.dedicated_cloud_service_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_service_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DedicatedCloudService>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DedicatedCloudService>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dedicated_cloud_service_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/dedicatedCloudServices/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.dedicated_cloud_service_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
}
pub mod skus_availability {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements SkuAvailability List method"]
        #[doc = "Returns list of available resources in region"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        pub fn list(&self, subscription_id: impl Into<String>, region_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                sku_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuAvailabilityListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuAvailabilityListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) sku_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "sku id, if no sku is passed availability for all skus will be returned"]
            pub fn sku_id(mut self, sku_id: impl Into<String>) -> Self {
                self.sku_id = Some(sku_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::SkuAvailabilityListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(sku_id) = &this.sku_id {
                                    req.url_mut().query_pairs_mut().append_pair("skuId", sku_id);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/availabilities",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
}
pub mod private_clouds {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements private cloud list GET method"]
        #[doc = "Returns list of private clouds in particular region"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        pub fn list(&self, subscription_id: impl Into<String>, region_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
            }
        }
        #[doc = "Implements private cloud GET method"]
        #[doc = "Returns private cloud by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            pc_name: impl Into<String>,
            region_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                pc_name: pc_name.into(),
                region_id: region_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloudList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PrivateCloudList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PrivateCloudList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloud> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PrivateCloud = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) pc_name: String,
            pub(crate) region_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::PrivateCloud>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::PrivateCloud>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod customization_policies {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements get of customization policies list"]
        #[doc = "Returns list of customization policies in region for private cloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                filter: None,
            }
        }
        #[doc = "Implements get of customization policy"]
        #[doc = "Returns customization policy by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `customization_policy_name`: customization policy name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
            customization_policy_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                customization_policy_name: customization_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomizationPoliciesListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomizationPoliciesListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation. only type is allowed here as a filter e.g. $filter=type eq 'xxxx'"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::CustomizationPoliciesListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/customizationPolicies",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomizationPolicy> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomizationPolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) customization_policy_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/customizationPolicies/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name,
                    &self.customization_policy_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CustomizationPolicy>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CustomizationPolicy>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod resource_pools {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements get of resource pools list"]
        #[doc = "Returns list of resource pools in region for private cloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
            }
        }
        #[doc = "Implements get of resource pool"]
        #[doc = "Returns resource pool templates by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `resource_pool_name`: resource pool id (vsphereId)"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
            resource_pool_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                resource_pool_name: resource_pool_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourcePoolsListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourcePoolsListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ResourcePoolsListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/resourcePools",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourcePool> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourcePool = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) resource_pool_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/resourcePools/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name,
                    &self.resource_pool_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ResourcePool>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ResourcePool>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod virtual_machine_templates {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list of available VM templates"]
        #[doc = "Returns list of virtual machine templates in region for private cloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `resource_pool_name`: Resource pool used to derive vSphere cluster which contains VM templates"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            pc_name: impl Into<String>,
            region_id: impl Into<String>,
            resource_pool_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                pc_name: pc_name.into(),
                region_id: region_id.into(),
                resource_pool_name: resource_pool_name.into(),
            }
        }
        #[doc = "Implements virtual machine template GET method"]
        #[doc = "Returns virtual machine templates by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `virtual_machine_template_name`: virtual machine template id (vsphereId)"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
            virtual_machine_template_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                virtual_machine_template_name: virtual_machine_template_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachineTemplateListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachineTemplateListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) pc_name: String,
            pub(crate) region_id: String,
            pub(crate) resource_pool_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::VirtualMachineTemplateListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let resource_pool_name = &this.resource_pool_name;
                                req.url_mut().query_pairs_mut().append_pair("resourcePoolName", resource_pool_name);
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/virtualMachineTemplates",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachineTemplate> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachineTemplate = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) virtual_machine_template_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/virtualMachineTemplates/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name,
                    &self.virtual_machine_template_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VirtualMachineTemplate>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VirtualMachineTemplate>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod virtual_networks {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list available virtual networks within a subscription method"]
        #[doc = "Return list of virtual networks in location for private cloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `resource_pool_name`: Resource pool used to derive vSphere cluster which contains virtual networks"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
            resource_pool_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                resource_pool_name: resource_pool_name.into(),
            }
        }
        #[doc = "Implements virtual network GET method"]
        #[doc = "Return virtual network by its name"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        #[doc = "* `pc_name`: The private cloud name"]
        #[doc = "* `virtual_network_name`: virtual network id (vsphereId)"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            region_id: impl Into<String>,
            pc_name: impl Into<String>,
            virtual_network_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                pc_name: pc_name.into(),
                virtual_network_name: virtual_network_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualNetworkListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualNetworkListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) resource_pool_name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::VirtualNetworkListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let resource_pool_name = &this.resource_pool_name;
                                req.url_mut().query_pairs_mut().append_pair("resourcePoolName", resource_pool_name);
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/virtualNetworks",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualNetwork> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualNetwork = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) pc_name: String,
            pub(crate) virtual_network_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/privateClouds/{}/virtualNetworks/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id,
                    &self.pc_name,
                    &self.virtual_network_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VirtualNetwork>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VirtualNetwork>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod usages {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements Usages List method"]
        #[doc = "Returns list of usage in region"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `region_id`: The region Id (westus, eastus)"]
        pub fn list(&self, subscription_id: impl Into<String>, region_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                region_id: region_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::UsageListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::UsageListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) region_id: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation. only name.value is allowed here as a filter e.g. $filter=name.value eq 'xxxx'"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::UsageListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/locations/{}/usages",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.region_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
}
pub mod virtual_machines {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Implements list virtual machine within subscription method"]
        #[doc = "Returns list virtual machine within subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements list virtual machine within RG method"]
        #[doc = "Returns list of virtual machine within resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                filter: None,
                top: None,
                skip_token: None,
            }
        }
        #[doc = "Implements virtual machine GET method"]
        #[doc = "Get virtual machine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Implements virtual machine PUT method"]
        #[doc = "Create Or Update Virtual Machine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        #[doc = "* `virtual_machine_request`: Create or Update Virtual Machine request"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            referer: impl Into<String>,
            virtual_machine_name: impl Into<String>,
            virtual_machine_request: impl Into<models::VirtualMachine>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                referer: referer.into(),
                virtual_machine_name: virtual_machine_name.into(),
                virtual_machine_request: virtual_machine_request.into(),
            }
        }
        #[doc = "Implements virtual machine PATCH method"]
        #[doc = "Patch virtual machine properties"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        #[doc = "* `virtual_machine_request`: Patch virtual machine request"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
            virtual_machine_request: impl Into<models::PatchPayload>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
                virtual_machine_request: virtual_machine_request.into(),
            }
        }
        #[doc = "Implements virtual machine DELETE method"]
        #[doc = "Delete virtual machine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            referer: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                referer: referer.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Implements a start method for a virtual machine"]
        #[doc = "Power on virtual machine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        pub fn start(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            referer: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> start::RequestBuilder {
            start::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                referer: referer.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Implements shutdown, poweroff, and suspend method for a virtual machine"]
        #[doc = "Power off virtual machine, options: shutdown, poweroff, and suspend"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group"]
        #[doc = "* `referer`: referer url"]
        #[doc = "* `virtual_machine_name`: virtual machine name"]
        pub fn stop(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            referer: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> stop::RequestBuilder {
            stop::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                referer: referer.into(),
                virtual_machine_name: virtual_machine_name.into(),
                m: None,
                mode: None,
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachineListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachineListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::VirtualMachineListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines",
                    self.client.endpoint(),
                    &self.subscription_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachineListResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachineListResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) skip_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The filter to apply on the list operation"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of record sets to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "to be used by nextLink implementation"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::VirtualMachineListResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachine> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachine = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VirtualMachine>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VirtualMachine>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachine> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachine = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("azure-asyncoperation"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) referer: String,
            pub(crate) virtual_machine_name: String,
            pub(crate) virtual_machine_request: models::VirtualMachine,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.virtual_machine_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VirtualMachine>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VirtualMachine>>;
            #[doc = "Returns a future that polls the long running operation and checks for the state via `properties.provisioningState` in the response body."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{body_content::get_provisioning_state, get_retry_after, LroStatus},
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    loop {
                        let this = self.clone();
                        let response = this.send().await?;
                        let retry_after = get_retry_after(response.as_raw_response().headers());
                        let status = response.as_raw_response().status();
                        let body = response.into_body().await?;
                        let provisioning_state = get_provisioning_state(status, &body)?;
                        log::trace!("current provisioning_state: {provisioning_state:?}");
                        match provisioning_state {
                            LroStatus::Succeeded => return Ok(body),
                            LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                            LroStatus::Canceled => {
                                return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                            }
                            _ => {
                                sleep(retry_after).await;
                            }
                        }
                    }
                })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachine> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::VirtualMachine = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) virtual_machine_name: String,
            pub(crate) virtual_machine_request: models::PatchPayload,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.virtual_machine_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::VirtualMachine>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::VirtualMachine>>;
            #[doc = "Returns a future that polls the long running operation and checks for the state via `properties.provisioningState` in the response body."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{body_content::get_provisioning_state, get_retry_after, LroStatus},
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    loop {
                        let this = self.clone();
                        let response = this.send().await?;
                        let retry_after = get_retry_after(response.as_raw_response().headers());
                        let status = response.as_raw_response().status();
                        let body = response.into_body().await?;
                        let provisioning_state = get_provisioning_state(status, &body)?;
                        log::trace!("current provisioning_state: {provisioning_state:?}");
                        match provisioning_state {
                            LroStatus::Succeeded => return Ok(body),
                            LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                            LroStatus::Canceled => {
                                return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                            }
                            _ => {
                                sleep(retry_after).await;
                            }
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("azure-asyncoperation"))
            }
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) referer: String,
            pub(crate) virtual_machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod start {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("azure-asyncoperation"))
            }
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) referer: String,
            pub(crate) virtual_machine_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}/start",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
    pub mod stop {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("azure-asyncoperation"))
            }
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) referer: String,
            pub(crate) virtual_machine_name: String,
            pub(crate) m: Option<models::VirtualMachineStopMode>,
            pub(crate) mode: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "body stop mode parameter (reboot, shutdown, etc...)"]
            pub fn m(mut self, m: impl Into<models::VirtualMachineStopMode>) -> Self {
                self.m = Some(m.into());
                self
            }
            #[doc = "query stop mode parameter (reboot, shutdown, etc...)"]
            pub fn mode(mut self, mode: impl Into<String>) -> Self {
                self.mode = Some(mode.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("referer", &this.referer);
                        let req_body = if let Some(m) = &this.m {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(m)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(mode) = &this.mode {
                            req.url_mut().query_pairs_mut().append_pair("mode", mode);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.VMwareCloudSimple/virtualMachines/{}/stop",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.virtual_machine_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2019-04-01");
                }
                Ok(url)
            }
        }
    }
}
