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
    pub fn attestations_client(&self) -> attestations::Client {
        attestations::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn policy_events_client(&self) -> policy_events::Client {
        policy_events::Client(self.clone())
    }
    pub fn policy_metadata_client(&self) -> policy_metadata::Client {
        policy_metadata::Client(self.clone())
    }
    pub fn policy_restrictions_client(&self) -> policy_restrictions::Client {
        policy_restrictions::Client(self.clone())
    }
    pub fn policy_states_client(&self) -> policy_states::Client {
        policy_states::Client(self.clone())
    }
    pub fn policy_tracked_resources_client(&self) -> policy_tracked_resources::Client {
        policy_tracked_resources::Client(self.clone())
    }
    pub fn remediations_client(&self) -> remediations::Client {
        remediations::Client(self.clone())
    }
}
pub mod policy_tracked_resources {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries policy tracked resources under the management group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_name`: Management group name."]
        #[doc = "* `policy_tracked_resources_resource`: The name of the virtual resource under PolicyTrackedResources resource type; only \"default\" is allowed."]
        pub fn list_query_results_for_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_name: impl Into<String>,
            policy_tracked_resources_resource: impl Into<String>,
        ) -> list_query_results_for_management_group::Builder {
            list_query_results_for_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_name: management_group_name.into(),
                policy_tracked_resources_resource: policy_tracked_resources_resource.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Queries policy tracked resources under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_tracked_resources_resource`: The name of the virtual resource under PolicyTrackedResources resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn list_query_results_for_subscription(
            &self,
            policy_tracked_resources_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_query_results_for_subscription::Builder {
            list_query_results_for_subscription::Builder {
                client: self.0.clone(),
                policy_tracked_resources_resource: policy_tracked_resources_resource.into(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Queries policy tracked resources under the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `policy_tracked_resources_resource`: The name of the virtual resource under PolicyTrackedResources resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn list_query_results_for_resource_group(
            &self,
            resource_group_name: impl Into<String>,
            policy_tracked_resources_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_query_results_for_resource_group::Builder {
            list_query_results_for_resource_group::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                policy_tracked_resources_resource: policy_tracked_resources_resource.into(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Queries policy tracked resources under the resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `policy_tracked_resources_resource`: The name of the virtual resource under PolicyTrackedResources resource type; only \"default\" is allowed."]
        pub fn list_query_results_for_resource(
            &self,
            resource_id: impl Into<String>,
            policy_tracked_resources_resource: impl Into<String>,
        ) -> list_query_results_for_resource::Builder {
            list_query_results_for_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                policy_tracked_resources_resource: policy_tracked_resources_resource.into(),
                top: None,
                filter: None,
            }
        }
    }
    pub mod list_query_results_for_management_group {
        use super::models;
        type Response = models::PolicyTrackedResourcesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_name: String,
            pub(crate) policy_tracked_resources_resource: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/policyTrackedResources/{}/queryResults",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_name,
                            &this.policy_tracked_resources_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyTrackedResourcesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_subscription {
        use super::models;
        type Response = models::PolicyTrackedResourcesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_tracked_resources_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/policyTrackedResources/{}/queryResults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.policy_tracked_resources_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyTrackedResourcesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource_group {
        use super::models;
        type Response = models::PolicyTrackedResourcesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) policy_tracked_resources_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/policyTrackedResources/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . policy_tracked_resources_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyTrackedResourcesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource {
        use super::models;
        type Response = models::PolicyTrackedResourcesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) policy_tracked_resources_resource: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/policyTrackedResources/{}/queryResults",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.policy_tracked_resources_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-07-01-preview");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyTrackedResourcesQueryResults = serde_json::from_slice(&rsp_body)?;
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
pub mod remediations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all deployments for a remediation at management group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn list_deployments_at_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> list_deployments_at_management_group::Builder {
            list_deployments_at_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                remediation_name: remediation_name.into(),
                top: None,
            }
        }
        #[doc = "Cancels a remediation at management group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn cancel_at_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> cancel_at_management_group::Builder {
            cancel_at_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all remediations for the management group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        pub fn list_for_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
        ) -> list_for_management_group::Builder {
            list_for_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing remediation at management group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn get_at_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> get_at_management_group::Builder {
            get_at_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Creates or updates a remediation at management group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        #[doc = "* `parameters`: The remediation parameters."]
        pub fn create_or_update_at_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
            remediation_name: impl Into<String>,
            parameters: impl Into<models::Remediation>,
        ) -> create_or_update_at_management_group::Builder {
            create_or_update_at_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                remediation_name: remediation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing remediation at management group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_id`: Management group ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn delete_at_management_group(
            &self,
            management_groups_namespace: impl Into<String>,
            management_group_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> delete_at_management_group::Builder {
            delete_at_management_group::Builder {
                client: self.0.clone(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_id: management_group_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all deployments for a remediation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn list_deployments_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> list_deployments_at_subscription::Builder {
            list_deployments_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                remediation_name: remediation_name.into(),
                top: None,
            }
        }
        #[doc = "Cancels a remediation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn cancel_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> cancel_at_subscription::Builder {
            cancel_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all remediations for the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn list_for_subscription(&self, subscription_id: impl Into<String>) -> list_for_subscription::Builder {
            list_for_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing remediation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn get_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> get_at_subscription::Builder {
            get_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Creates or updates a remediation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        #[doc = "* `parameters`: The remediation parameters."]
        pub fn create_or_update_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            remediation_name: impl Into<String>,
            parameters: impl Into<models::Remediation>,
        ) -> create_or_update_at_subscription::Builder {
            create_or_update_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                remediation_name: remediation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing remediation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn delete_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> delete_at_subscription::Builder {
            delete_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all deployments for a remediation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn list_deployments_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> list_deployments_at_resource_group::Builder {
            list_deployments_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                remediation_name: remediation_name.into(),
                top: None,
            }
        }
        #[doc = "Cancels a remediation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn cancel_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> cancel_at_resource_group::Builder {
            cancel_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all remediations for the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        pub fn list_for_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_for_resource_group::Builder {
            list_for_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing remediation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn get_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> get_at_resource_group::Builder {
            get_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Creates or updates a remediation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        #[doc = "* `parameters`: The remediation parameters."]
        pub fn create_or_update_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            remediation_name: impl Into<String>,
            parameters: impl Into<models::Remediation>,
        ) -> create_or_update_at_resource_group::Builder {
            create_or_update_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                remediation_name: remediation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing remediation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn delete_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> delete_at_resource_group::Builder {
            delete_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all deployments for a remediation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn list_deployments_at_resource(
            &self,
            resource_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> list_deployments_at_resource::Builder {
            list_deployments_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                remediation_name: remediation_name.into(),
                top: None,
            }
        }
        #[doc = "Cancel a remediation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn cancel_at_resource(
            &self,
            resource_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> cancel_at_resource::Builder {
            cancel_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Gets all remediations for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        pub fn list_for_resource(&self, resource_id: impl Into<String>) -> list_for_resource::Builder {
            list_for_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing remediation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn get_at_resource(&self, resource_id: impl Into<String>, remediation_name: impl Into<String>) -> get_at_resource::Builder {
            get_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
        #[doc = "Creates or updates a remediation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        #[doc = "* `parameters`: The remediation parameters."]
        pub fn create_or_update_at_resource(
            &self,
            resource_id: impl Into<String>,
            remediation_name: impl Into<String>,
            parameters: impl Into<models::Remediation>,
        ) -> create_or_update_at_resource::Builder {
            create_or_update_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                remediation_name: remediation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing remediation at individual resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `remediation_name`: The name of the remediation."]
        pub fn delete_at_resource(
            &self,
            resource_id: impl Into<String>,
            remediation_name: impl Into<String>,
        ) -> delete_at_resource::Builder {
            delete_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                remediation_name: remediation_name.into(),
            }
        }
    }
    pub mod list_deployments_at_management_group {
        use super::models;
        type Response = models::RemediationDeploymentsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) remediation_name: String,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}/listDeployments",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id,
                            &this.remediation_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RemediationDeploymentsListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel_at_management_group {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}/cancel",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id,
                            &this.remediation_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_for_management_group {
        use super::models;
        type Response = models::RemediationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::RemediationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_management_group {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_management_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            Created201(models::Remediation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) remediation_name: String,
            pub(crate) parameters: models::Remediation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_management_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
    pub mod list_deployments_at_subscription {
        use super::models;
        type Response = models::RemediationDeploymentsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) remediation_name: String,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations/{}/listDeployments",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.remediation_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RemediationDeploymentsListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel_at_subscription {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.remediation_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_for_subscription {
        use super::models;
        type Response = models::RemediationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::RemediationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_subscription {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_subscription {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            Created201(models::Remediation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) remediation_name: String,
            pub(crate) parameters: models::Remediation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_subscription {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
    pub mod list_deployments_at_resource_group {
        use super::models;
        type Response = models::RemediationDeploymentsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) remediation_name: String,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}/listDeployments",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.remediation_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RemediationDeploymentsListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel_at_resource_group {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.remediation_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_for_resource_group {
        use super::models;
        type Response = models::RemediationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::RemediationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_resource_group {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_resource_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            Created201(models::Remediation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) remediation_name: String,
            pub(crate) parameters: models::Remediation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_resource_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
    pub mod list_deployments_at_resource {
        use super::models;
        type Response = models::RemediationDeploymentsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) remediation_name: String,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations/{}/listDeployments",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.remediation_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RemediationDeploymentsListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel_at_resource {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations/{}/cancel",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.remediation_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_for_resource {
        use super::models;
        type Response = models::RemediationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations",
                            this.client.endpoint(),
                            &this.resource_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::RemediationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_resource {
        use super::models;
        type Response = models::Remediation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_resource {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            Created201(models::Remediation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) remediation_name: String,
            pub(crate) parameters: models::Remediation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_resource {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Remediation),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) remediation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/remediations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.remediation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Remediation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
pub mod policy_events {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Subsequent post calls to the next link"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `next_link`: Next link for list operation."]
        pub fn next_link(&self, next_link: impl Into<String>) -> next_link::Builder {
            next_link::Builder {
                client: self.0.clone(),
                next_link: next_link.into(),
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the resources under the management group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_name`: Management group name."]
        pub fn list_query_results_for_management_group(
            &self,
            policy_events_resource: impl Into<String>,
            management_groups_namespace: impl Into<String>,
            management_group_name: impl Into<String>,
        ) -> list_query_results_for_management_group::Builder {
            list_query_results_for_management_group::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_name: management_group_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the resources under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn list_query_results_for_subscription(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_query_results_for_subscription::Builder {
            list_query_results_for_subscription::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the resources under the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        pub fn list_query_results_for_resource_group(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_query_results_for_resource_group::Builder {
            list_query_results_for_resource_group::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `resource_id`: Resource ID."]
        pub fn list_query_results_for_resource(
            &self,
            policy_events_resource: impl Into<String>,
            resource_id: impl Into<String>,
        ) -> list_query_results_for_resource::Builder {
            list_query_results_for_resource::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                resource_id: resource_id.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                expand: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the subscription level policy set definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_set_definition_name`: Policy set definition name."]
        pub fn list_query_results_for_policy_set_definition(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_set_definition_name: impl Into<String>,
        ) -> list_query_results_for_policy_set_definition::Builder {
            list_query_results_for_policy_set_definition::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_set_definition_name: policy_set_definition_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the subscription level policy definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_definition_name`: Policy definition name."]
        pub fn list_query_results_for_policy_definition(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_definition_name: impl Into<String>,
        ) -> list_query_results_for_policy_definition::Builder {
            list_query_results_for_policy_definition::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_definition_name: policy_definition_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the subscription level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn list_query_results_for_subscription_level_policy_assignment(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> list_query_results_for_subscription_level_policy_assignment::Builder {
            list_query_results_for_subscription_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Queries policy events for the resource group level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_events_resource`: The name of the virtual resource under PolicyEvents resource type; only \"default\" is allowed."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn list_query_results_for_resource_group_level_policy_assignment(
            &self,
            policy_events_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> list_query_results_for_resource_group_level_policy_assignment::Builder {
            list_query_results_for_resource_group_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_events_resource: policy_events_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
    }
    pub mod next_link {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) next_link: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}{}?Next paging op for policy events",
                            this.client.endpoint(),
                            &this.next_link
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(skiptoken) = &this.skiptoken {
                            req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_management_group {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_name,
                            &this.policy_events_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_subscription {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.policy_events_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource_group {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_events_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) resource_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "The $expand query parameter. For example, to expand components use $expand=components"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.policy_events_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_policy_set_definition {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_set_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policySetDefinitions/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_set_definition_name , & this . policy_events_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_policy_definition {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyDefinitions/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_definition_name , & this . policy_events_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_subscription_level_policy_assignment {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_events_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource_group_level_policy_assignment {
        use super::models;
        type Response = models::PolicyEventsQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_events_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourcegroups/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyEvents/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_events_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyEventsQueryResults = serde_json::from_slice(&rsp_body)?;
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
pub mod policy_states {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Subsequent post calls to the next link"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `next_link`: Next link for list operation."]
        pub fn next_link(&self, next_link: impl Into<String>) -> next_link::Builder {
            next_link::Builder {
                client: self.0.clone(),
                next_link: next_link.into(),
                skiptoken: None,
            }
        }
        #[doc = "Queries policy states for the resources under the management group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_name`: Management group name."]
        pub fn list_query_results_for_management_group(
            &self,
            policy_states_resource: impl Into<String>,
            management_groups_namespace: impl Into<String>,
            management_group_name: impl Into<String>,
        ) -> list_query_results_for_management_group::Builder {
            list_query_results_for_management_group::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_name: management_group_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the resources under the management group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `management_groups_namespace`: The namespace for Microsoft Management RP; only \"Microsoft.Management\" is allowed."]
        #[doc = "* `management_group_name`: Management group name."]
        pub fn summarize_for_management_group(
            &self,
            policy_states_summary_resource: impl Into<String>,
            management_groups_namespace: impl Into<String>,
            management_group_name: impl Into<String>,
        ) -> summarize_for_management_group::Builder {
            summarize_for_management_group::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                management_groups_namespace: management_groups_namespace.into(),
                management_group_name: management_group_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the resources under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn list_query_results_for_subscription(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_query_results_for_subscription::Builder {
            list_query_results_for_subscription::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the resources under the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn summarize_for_subscription(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> summarize_for_subscription::Builder {
            summarize_for_subscription::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the resources under the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        pub fn list_query_results_for_resource_group(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_query_results_for_resource_group::Builder {
            list_query_results_for_resource_group::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the resources under the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        pub fn summarize_for_resource_group(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> summarize_for_resource_group::Builder {
            summarize_for_resource_group::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `resource_id`: Resource ID."]
        pub fn list_query_results_for_resource(
            &self,
            policy_states_resource: impl Into<String>,
            resource_id: impl Into<String>,
        ) -> list_query_results_for_resource::Builder {
            list_query_results_for_resource::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                resource_id: resource_id.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                expand: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `resource_id`: Resource ID."]
        pub fn summarize_for_resource(
            &self,
            policy_states_summary_resource: impl Into<String>,
            resource_id: impl Into<String>,
        ) -> summarize_for_resource::Builder {
            summarize_for_resource::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                resource_id: resource_id.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Triggers a policy evaluation scan for all the resources under the subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        pub fn trigger_subscription_evaluation(&self, subscription_id: impl Into<String>) -> trigger_subscription_evaluation::Builder {
            trigger_subscription_evaluation::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Triggers a policy evaluation scan for all the resources under the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        pub fn trigger_resource_group_evaluation(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> trigger_resource_group_evaluation::Builder {
            trigger_resource_group_evaluation::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Queries policy states for the subscription level policy set definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_set_definition_name`: Policy set definition name."]
        pub fn list_query_results_for_policy_set_definition(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_set_definition_name: impl Into<String>,
        ) -> list_query_results_for_policy_set_definition::Builder {
            list_query_results_for_policy_set_definition::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_set_definition_name: policy_set_definition_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the subscription level policy set definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_set_definition_name`: Policy set definition name."]
        pub fn summarize_for_policy_set_definition(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_set_definition_name: impl Into<String>,
        ) -> summarize_for_policy_set_definition::Builder {
            summarize_for_policy_set_definition::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_set_definition_name: policy_set_definition_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the subscription level policy definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_definition_name`: Policy definition name."]
        pub fn list_query_results_for_policy_definition(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_definition_name: impl Into<String>,
        ) -> list_query_results_for_policy_definition::Builder {
            list_query_results_for_policy_definition::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_definition_name: policy_definition_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the subscription level policy definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_definition_name`: Policy definition name."]
        pub fn summarize_for_policy_definition(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_definition_name: impl Into<String>,
        ) -> summarize_for_policy_definition::Builder {
            summarize_for_policy_definition::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_definition_name: policy_definition_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the subscription level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn list_query_results_for_subscription_level_policy_assignment(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> list_query_results_for_subscription_level_policy_assignment::Builder {
            list_query_results_for_subscription_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the subscription level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn summarize_for_subscription_level_policy_assignment(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> summarize_for_subscription_level_policy_assignment::Builder {
            summarize_for_subscription_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
        #[doc = "Queries policy states for the resource group level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_resource`: The virtual resource under PolicyStates resource type. In a given time range, 'latest' represents the latest policy state(s), whereas 'default' represents all policy state(s)."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn list_query_results_for_resource_group_level_policy_assignment(
            &self,
            policy_states_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> list_query_results_for_resource_group_level_policy_assignment::Builder {
            list_query_results_for_resource_group_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_states_resource: policy_states_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                orderby: None,
                select: None,
                from: None,
                to: None,
                filter: None,
                apply: None,
                skiptoken: None,
            }
        }
        #[doc = "Summarizes policy states for the resource group level policy assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `policy_states_summary_resource`: The virtual resource under PolicyStates resource type for summarize action. In a given time range, 'latest' represents the latest policy state(s) and is the only allowed value."]
        #[doc = "* `subscription_id`: Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: Resource group name."]
        #[doc = "* `authorization_namespace`: The namespace for Microsoft Authorization resource provider; only \"Microsoft.Authorization\" is allowed."]
        #[doc = "* `policy_assignment_name`: Policy assignment name."]
        pub fn summarize_for_resource_group_level_policy_assignment(
            &self,
            policy_states_summary_resource: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            authorization_namespace: impl Into<String>,
            policy_assignment_name: impl Into<String>,
        ) -> summarize_for_resource_group_level_policy_assignment::Builder {
            summarize_for_resource_group_level_policy_assignment::Builder {
                client: self.0.clone(),
                policy_states_summary_resource: policy_states_summary_resource.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                authorization_namespace: authorization_namespace.into(),
                policy_assignment_name: policy_assignment_name.into(),
                top: None,
                from: None,
                to: None,
                filter: None,
            }
        }
    }
    pub mod next_link {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) next_link: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}{}?Next paging op for policy states",
                            this.client.endpoint(),
                            &this.next_link
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(skiptoken) = &this.skiptoken {
                            req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_management_group {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_name,
                            &this.policy_states_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_management_group {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) management_groups_namespace: String,
            pub(crate) management_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/{}/managementGroups/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize",
                            this.client.endpoint(),
                            &this.management_groups_namespace,
                            &this.management_group_name,
                            &this.policy_states_summary_resource
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_subscription {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.policy_states_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_subscription {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.policy_states_summary_resource
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource_group {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_states_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_resource_group {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_states_summary_resource
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) resource_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "The $expand query parameter. For example, to expand components use $expand=components"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.policy_states_resource
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_resource {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) resource_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.policy_states_summary_resource
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod trigger_subscription_evaluation {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/policyStates/latest/triggerEvaluation",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
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
    pub mod trigger_resource_group_evaluation {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/policyStates/latest/triggerEvaluation" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
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
    pub mod list_query_results_for_policy_set_definition {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_set_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policySetDefinitions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_set_definition_name , & this . policy_states_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_policy_set_definition {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_set_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policySetDefinitions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_set_definition_name , & this . policy_states_summary_resource)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_policy_definition {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyDefinitions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_definition_name , & this . policy_states_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_policy_definition {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_definition_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyDefinitions/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_definition_name , & this . policy_states_summary_resource)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_subscription_level_policy_assignment {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_states_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_subscription_level_policy_assignment {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize" , this . client . endpoint () , & this . subscription_id , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_states_summary_resource)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_query_results_for_resource_group_level_policy_assignment {
        use super::models;
        type Response = models::PolicyStatesQueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) apply: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Ordering expression using OData notation. One or more comma-separated column names with an optional \"desc\" (the default) or \"asc\", e.g. \"$orderby=PolicyAssignmentId, ResourceId asc\"."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "OData apply expression for aggregations."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            #[doc = "Skiptoken is only provided if a previous response returned a partial result as a part of nextLink element."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourcegroups/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyStates/{}/queryResults" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_states_resource)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(apply) = &this.apply {
                                    req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyStatesQueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod summarize_for_resource_group_level_policy_assignment {
        use super::models;
        type Response = models::SummarizeResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) policy_states_summary_resource: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) authorization_namespace: String,
            pub(crate) policy_assignment_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the start time of the interval to query. When not specified, the service uses ($to - 1-day)."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "ISO 8601 formatted timestamp specifying the end time of the interval to query. When not specified, the service uses request time."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourcegroups/{}/providers/{}/policyAssignments/{}/providers/Microsoft.PolicyInsights/policyStates/{}/summarize" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . authorization_namespace , & this . policy_assignment_name , & this . policy_states_summary_resource)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("$from", &from.to_string());
                        }
                        if let Some(to) = &this.to {
                            req.url_mut().query_pairs_mut().append_pair("$to", &to.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SummarizeResults = serde_json::from_slice(&rsp_body)?;
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
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists available operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationsListResults;
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
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.PolicyInsights/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsListResults = serde_json::from_slice(&rsp_body)?;
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
pub mod policy_metadata {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get policy metadata resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the policy metadata resource."]
        pub fn get_resource(&self, resource_name: impl Into<String>) -> get_resource::Builder {
            get_resource::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
            }
        }
        #[doc = "Get a list of the policy metadata resources."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                top: None,
            }
        }
    }
    pub mod get_resource {
        use super::models;
        type Response = models::PolicyMetadata;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.PolicyInsights/policyMetadata/{}",
                            this.client.endpoint(),
                            &this.resource_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyMetadata = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::PolicyMetadataCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.PolicyInsights/policyMetadata",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-10-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
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
                                let rsp_value: models::PolicyMetadataCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod policy_restrictions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Checks what restrictions Azure Policy will place on a resource within a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `parameters`: The check policy restrictions parameters."]
        pub fn check_at_subscription_scope(
            &self,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CheckRestrictionsRequest>,
        ) -> check_at_subscription_scope::Builder {
            check_at_subscription_scope::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Checks what restrictions Azure Policy will place on a resource within a resource group. Use this when the resource group the resource will be created in is already known."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `parameters`: The check policy restrictions parameters."]
        pub fn check_at_resource_group_scope(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            parameters: impl Into<models::CheckRestrictionsRequest>,
        ) -> check_at_resource_group_scope::Builder {
            check_at_resource_group_scope::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod check_at_subscription_scope {
        use super::models;
        type Response = models::CheckRestrictionsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CheckRestrictionsRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/checkPolicyRestrictions",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckRestrictionsResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_at_resource_group_scope {
        use super::models;
        type Response = models::CheckRestrictionsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) parameters: models::CheckRestrictionsRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/checkPolicyRestrictions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-07-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckRestrictionsResult = serde_json::from_slice(&rsp_body)?;
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
pub mod attestations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all attestations for the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_for_subscription(&self, subscription_id: impl Into<String>) -> list_for_subscription::Builder {
            list_for_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing attestation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn get_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            attestation_name: impl Into<String>,
        ) -> get_at_subscription::Builder {
            get_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                attestation_name: attestation_name.into(),
            }
        }
        #[doc = "Creates or updates an attestation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        #[doc = "* `parameters`: The attestation parameters."]
        pub fn create_or_update_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            attestation_name: impl Into<String>,
            parameters: impl Into<models::Attestation>,
        ) -> create_or_update_at_subscription::Builder {
            create_or_update_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                attestation_name: attestation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing attestation at subscription scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn delete_at_subscription(
            &self,
            subscription_id: impl Into<String>,
            attestation_name: impl Into<String>,
        ) -> delete_at_subscription::Builder {
            delete_at_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                attestation_name: attestation_name.into(),
            }
        }
        #[doc = "Gets all attestations for the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        pub fn list_for_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_for_resource_group::Builder {
            list_for_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing attestation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn get_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            attestation_name: impl Into<String>,
        ) -> get_at_resource_group::Builder {
            get_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                attestation_name: attestation_name.into(),
            }
        }
        #[doc = "Creates or updates an attestation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        #[doc = "* `parameters`: The attestation parameters."]
        pub fn create_or_update_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            attestation_name: impl Into<String>,
            parameters: impl Into<models::Attestation>,
        ) -> create_or_update_at_resource_group::Builder {
            create_or_update_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                attestation_name: attestation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing attestation at resource group scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn delete_at_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            attestation_name: impl Into<String>,
        ) -> delete_at_resource_group::Builder {
            delete_at_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                attestation_name: attestation_name.into(),
            }
        }
        #[doc = "Gets all attestations for a resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        pub fn list_for_resource(&self, resource_id: impl Into<String>) -> list_for_resource::Builder {
            list_for_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Gets an existing attestation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn get_at_resource(&self, resource_id: impl Into<String>, attestation_name: impl Into<String>) -> get_at_resource::Builder {
            get_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                attestation_name: attestation_name.into(),
            }
        }
        #[doc = "Creates or updates an attestation at resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        #[doc = "* `parameters`: The attestation parameters."]
        pub fn create_or_update_at_resource(
            &self,
            resource_id: impl Into<String>,
            attestation_name: impl Into<String>,
            parameters: impl Into<models::Attestation>,
        ) -> create_or_update_at_resource::Builder {
            create_or_update_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                attestation_name: attestation_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing attestation at individual resource scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_id`: Resource ID."]
        #[doc = "* `attestation_name`: The name of the attestation."]
        pub fn delete_at_resource(
            &self,
            resource_id: impl Into<String>,
            attestation_name: impl Into<String>,
        ) -> delete_at_resource::Builder {
            delete_at_resource::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                attestation_name: attestation_name.into(),
            }
        }
    }
    pub mod list_for_subscription {
        use super::models;
        type Response = models::AttestationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/attestations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::AttestationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_subscription {
        use super::models;
        type Response = models::Attestation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_subscription {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Attestation),
            Created201(models::Attestation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) attestation_name: String,
            pub(crate) parameters: models::Attestation,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_subscription {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
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
    pub mod list_for_resource_group {
        use super::models;
        type Response = models::AttestationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/attestations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::AttestationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_resource_group {
        use super::models;
        type Response = models::Attestation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_resource_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Attestation),
            Created201(models::Attestation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) attestation_name: String,
            pub(crate) parameters: models::Attestation,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_resource_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
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
    pub mod list_for_resource {
        use super::models;
        type Response = models::AttestationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/attestations",
                            this.client.endpoint(),
                            &this.resource_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::AttestationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_at_resource {
        use super::models;
        type Response = models::Attestation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_at_resource {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Attestation),
            Created201(models::Attestation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) attestation_name: String,
            pub(crate) parameters: models::Attestation,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Attestation = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_at_resource {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) attestation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.PolicyInsights/attestations/{}",
                            this.client.endpoint(),
                            &this.resource_id,
                            &this.attestation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
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
