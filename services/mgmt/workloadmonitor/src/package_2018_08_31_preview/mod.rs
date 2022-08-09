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
    pub fn components_client(&self) -> components::Client {
        components::Client(self.clone())
    }
    pub fn components_summary_client(&self) -> components_summary::Client {
        components_summary::Client(self.clone())
    }
    pub fn monitor_instances_client(&self) -> monitor_instances::Client {
        monitor_instances::Client(self.clone())
    }
    pub fn monitor_instances_summary_client(&self) -> monitor_instances_summary::Client {
        monitor_instances_summary::Client(self.clone())
    }
    pub fn monitors_client(&self) -> monitors::Client {
        monitors::Client(self.clone())
    }
    pub fn notification_settings_client(&self) -> notification_settings::Client {
        notification_settings::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
pub mod monitors {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of a monitors of a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        pub fn list_by_resource(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
        ) -> list_by_resource::Builder {
            list_by_resource::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                filter: None,
                skiptoken: None,
            }
        }
        #[doc = "Get details of a single monitor."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `monitor_id`: Monitor Id."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            monitor_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                monitor_id: monitor_id.into(),
            }
        }
        #[doc = "Update a Monitor's configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `monitor_id`: Monitor Id."]
        #[doc = "* `body`: Body of the Monitor PATCH object."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            monitor_id: impl Into<String>,
            body: impl Into<models::Monitor>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                monitor_id: monitor_id.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_resource {
        use super::models;
        type Response = models::MonitorsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Filter to be applied on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/monitors",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::MonitorsCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Monitor;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) monitor_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/monitors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name,
                            &this.monitor_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Monitor = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Monitor;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) monitor_id: String,
            pub(crate) body: models::Monitor,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/monitors/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name,
                            &this.monitor_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Monitor = serde_json::from_slice(&rsp_body)?;
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
pub mod components {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of components for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        pub fn list_by_resource(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
        ) -> list_by_resource::Builder {
            list_by_resource::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                select: None,
                filter: None,
                apply: None,
                orderby: None,
                expand: None,
                top: None,
                skiptoken: None,
            }
        }
        #[doc = "Get details of a component."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `component_id`: Component Id."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            component_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                component_id: component_id.into(),
                select: None,
                expand: None,
            }
        }
    }
    pub mod list_by_resource {
        use super::models;
        type Response = models::ComponentsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) top: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Filter to be applied on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Apply aggregation."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Sort the result on one or more properties."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Limit the result to the specified number of rows."]
            pub fn top(mut self, top: impl Into<String>) -> Self {
                self.top = Some(top.into());
                self
            }
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/components",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", top);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::ComponentsCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Component;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) component_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/components/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name,
                            &this.component_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
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
                                let rsp_value: models::Component = serde_json::from_slice(&rsp_body)?;
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
pub mod monitor_instances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of monitor instances for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        pub fn list_by_resource(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
        ) -> list_by_resource::Builder {
            list_by_resource::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                select: None,
                filter: None,
                apply: None,
                orderby: None,
                expand: None,
                top: None,
                skiptoken: None,
            }
        }
        #[doc = "Get details of a monitorInstance."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `monitor_instance_id`: MonitorInstance Id."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            monitor_instance_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                monitor_instance_id: monitor_instance_id.into(),
                select: None,
                expand: None,
            }
        }
    }
    pub mod list_by_resource {
        use super::models;
        type Response = models::MonitorInstancesCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) top: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Filter to be applied on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Apply aggregation."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Sort the result on one or more properties."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Limit the result to the specified number of rows."]
            pub fn top(mut self, top: impl Into<String>) -> Self {
                self.top = Some(top.into());
                self
            }
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/monitorInstances",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_namespace,
                            &this.resource_type,
                            &this.resource_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", top);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::MonitorInstancesCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::MonitorInstance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) monitor_instance_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/monitorInstances/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_namespace , & this . resource_type , & this . resource_name , & this . monitor_instance_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
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
                                let rsp_value: models::MonitorInstance = serde_json::from_slice(&rsp_body)?;
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
pub mod notification_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of notification settings for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        pub fn list_by_resource(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
        ) -> list_by_resource::Builder {
            list_by_resource::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                skiptoken: None,
            }
        }
        #[doc = "Get a of notification setting for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `notification_setting_name`: Default string modeled as parameter for URL to work correctly."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            notification_setting_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                notification_setting_name: notification_setting_name.into(),
            }
        }
        #[doc = "Update notification settings for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `resource_namespace`: The Namespace of the resource."]
        #[doc = "* `resource_type`: The type of the resource."]
        #[doc = "* `resource_name`: Name of the resource."]
        #[doc = "* `notification_setting_name`: Default string modeled as parameter for URL to work correctly."]
        #[doc = "* `body`: Body of the NotificationSetting PUT object."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            resource_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            resource_name: impl Into<String>,
            notification_setting_name: impl Into<String>,
            body: impl Into<models::NotificationSetting>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                resource_namespace: resource_namespace.into(),
                resource_type: resource_type.into(),
                resource_name: resource_name.into(),
                notification_setting_name: notification_setting_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_resource {
        use super::models;
        type Response = models::NotificationSettingsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/notificationSettings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_namespace , & this . resource_type , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::NotificationSettingsCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::NotificationSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) notification_setting_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/notificationSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_namespace , & this . resource_type , & this . resource_name , & this . notification_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationSetting = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::NotificationSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) resource_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) resource_name: String,
            pub(crate) notification_setting_name: String,
            pub(crate) body: models::NotificationSetting,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.WorkloadMonitor/notificationSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_namespace , & this . resource_type , & this . resource_name , & this . notification_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationSetting = serde_json::from_slice(&rsp_body)?;
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
pub mod components_summary {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get subscription wide details of components."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                select: None,
                filter: None,
                apply: None,
                orderby: None,
                expand: None,
                top: None,
                skiptoken: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ComponentsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) top: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Filter to be applied on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Apply aggregation."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Sort the result on one or more properties."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Limit the result to the specified number of rows."]
            pub fn top(mut self, top: impl Into<String>) -> Self {
                self.top = Some(top.into());
                self
            }
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.WorkloadMonitor/componentsSummary",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", top);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::ComponentsCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod monitor_instances_summary {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get subscription wide health instances."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                select: None,
                filter: None,
                apply: None,
                orderby: None,
                expand: None,
                top: None,
                skiptoken: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::MonitorInstancesCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) top: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Properties to be returned in the response."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Filter to be applied on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Apply aggregation."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Sort the result on one or more properties."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Include properties inline in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Limit the result to the specified number of rows."]
            pub fn top(mut self, top: impl Into<String>) -> Self {
                self.top = Some(top.into());
                self
            }
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.WorkloadMonitor/monitorInstancesSummary",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", top);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::MonitorInstancesCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of all operations possible on the resource provider."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skiptoken: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "The page-continuation token to use with a paged version of this API."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.WorkloadMonitor/operations",
                            this.client.endpoint(),
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-08-31-preview");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
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
