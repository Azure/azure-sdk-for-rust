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
    pub fn afd_custom_domains_client(&self) -> afd_custom_domains::Client {
        afd_custom_domains::Client(self.clone())
    }
    pub fn afd_endpoints_client(&self) -> afd_endpoints::Client {
        afd_endpoints::Client(self.clone())
    }
    pub fn afd_origin_groups_client(&self) -> afd_origin_groups::Client {
        afd_origin_groups::Client(self.clone())
    }
    pub fn afd_origins_client(&self) -> afd_origins::Client {
        afd_origins::Client(self.clone())
    }
    pub fn afd_profiles_client(&self) -> afd_profiles::Client {
        afd_profiles::Client(self.clone())
    }
    pub fn custom_domains_client(&self) -> custom_domains::Client {
        custom_domains::Client(self.clone())
    }
    pub fn edge_nodes_client(&self) -> edge_nodes::Client {
        edge_nodes::Client(self.clone())
    }
    pub fn endpoints_client(&self) -> endpoints::Client {
        endpoints::Client(self.clone())
    }
    pub fn log_analytics_client(&self) -> log_analytics::Client {
        log_analytics::Client(self.clone())
    }
    pub fn managed_rule_sets_client(&self) -> managed_rule_sets::Client {
        managed_rule_sets::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn origin_groups_client(&self) -> origin_groups::Client {
        origin_groups::Client(self.clone())
    }
    pub fn origins_client(&self) -> origins::Client {
        origins::Client(self.clone())
    }
    pub fn policies_client(&self) -> policies::Client {
        policies::Client(self.clone())
    }
    pub fn profiles_client(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
    pub fn resource_usage_client(&self) -> resource_usage::Client {
        resource_usage::Client(self.clone())
    }
    pub fn routes_client(&self) -> routes::Client {
        routes::Client(self.clone())
    }
    pub fn rule_sets_client(&self) -> rule_sets::Client {
        rule_sets::Client(self.clone())
    }
    pub fn rules_client(&self) -> rules::Client {
        rules::Client(self.clone())
    }
    pub fn secrets_client(&self) -> secrets::Client {
        secrets::Client(self.clone())
    }
    pub fn security_policies_client(&self) -> security_policies::Client {
        security_policies::Client(self.clone())
    }
    pub fn validate_client(&self) -> validate::Client {
        validate::Client(self.clone())
    }
}
impl Client {
    #[doc = "Check the availability of a resource name. This is needed for resources where name is globally unique, such as a afdx endpoint."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `check_endpoint_name_availability_input`: Input to check."]
    #[doc = "* `subscription_id`: Azure Subscription ID."]
    #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
    pub fn check_endpoint_name_availability(
        &self,
        check_endpoint_name_availability_input: impl Into<models::CheckEndpointNameAvailabilityInput>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> check_endpoint_name_availability::Builder {
        check_endpoint_name_availability::Builder {
            client: self.clone(),
            check_endpoint_name_availability_input: check_endpoint_name_availability_input.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
        }
    }
    #[doc = "Check the availability of a resource name. This is needed for resources where name is globally unique, such as a CDN endpoint."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `check_name_availability_input`: Input to check."]
    pub fn check_name_availability(
        &self,
        check_name_availability_input: impl Into<models::CheckNameAvailabilityInput>,
    ) -> check_name_availability::Builder {
        check_name_availability::Builder {
            client: self.clone(),
            check_name_availability_input: check_name_availability_input.into(),
        }
    }
    #[doc = "Check the availability of a resource name. This is needed for resources where name is globally unique, such as a CDN endpoint."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `check_name_availability_input`: Input to check."]
    #[doc = "* `subscription_id`: Azure Subscription ID."]
    pub fn check_name_availability_with_subscription(
        &self,
        check_name_availability_input: impl Into<models::CheckNameAvailabilityInput>,
        subscription_id: impl Into<String>,
    ) -> check_name_availability_with_subscription::Builder {
        check_name_availability_with_subscription::Builder {
            client: self.clone(),
            check_name_availability_input: check_name_availability_input.into(),
            subscription_id: subscription_id.into(),
        }
    }
    #[doc = "Check if the probe path is a valid path and the file can be accessed. Probe path is the path to a file hosted on the origin server to help accelerate the delivery of dynamic content via the CDN endpoint. This path is relative to the origin path specified in the endpoint configuration."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `validate_probe_input`: Input to check."]
    #[doc = "* `subscription_id`: Azure Subscription ID."]
    pub fn validate_probe(
        &self,
        validate_probe_input: impl Into<models::ValidateProbeInput>,
        subscription_id: impl Into<String>,
    ) -> validate_probe::Builder {
        validate_probe::Builder {
            client: self.clone(),
            validate_probe_input: validate_probe_input.into(),
            subscription_id: subscription_id.into(),
        }
    }
}
pub mod check_endpoint_name_availability {
    use super::models;
    type Response = models::CheckEndpointNameAvailabilityOutput;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) check_endpoint_name_availability_input: models::CheckEndpointNameAvailabilityInput,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/checkEndpointNameAvailability",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_endpoint_name_availability_input)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CheckEndpointNameAvailabilityOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod check_name_availability {
    use super::models;
    type Response = models::CheckNameAvailabilityOutput;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url =
                        azure_core::Url::parse(&format!("{}/providers/Microsoft.Cdn/checkNameAvailability", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CheckNameAvailabilityOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod check_name_availability_with_subscription {
    use super::models;
    type Response = models::CheckNameAvailabilityOutput;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
        pub(crate) subscription_id: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.Cdn/checkNameAvailability",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CheckNameAvailabilityOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod validate_probe {
    use super::models;
    type Response = models::ValidateProbeOutput;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) validate_probe_input: models::ValidateProbeInput,
        pub(crate) subscription_id: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.Cdn/validateProbe",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.validate_probe_input)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::ValidateProbeOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod afd_profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Checks the quota and actual usage of AzureFrontDoor endpoints under the given CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Check the name availability of a host name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `check_host_name_availability_input`: Custom domain to be validated."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn check_host_name_availability(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            check_host_name_availability_input: impl Into<models::CheckHostNameAvailabilityInput>,
            subscription_id: impl Into<String>,
        ) -> check_host_name_availability::Builder {
            check_host_name_availability::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                check_host_name_availability_input: check_host_name_availability_input.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_resource_usage {
        use super::models;
        type Response = models::UsagesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/usages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::UsagesListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_host_name_availability {
        use super::models;
        type Response = models::CheckNameAvailabilityOutput;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) check_host_name_availability_input: models::CheckHostNameAvailabilityInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/checkHostNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.check_host_name_availability_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckNameAvailabilityOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod afd_custom_domains {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists existing AzureFrontDoor domains."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing AzureFrontDoor domain with the specified domain name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `custom_domain_name`: Name of the domain under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new domain within the specified profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `custom_domain_name`: Name of the domain under the profile which is unique globally"]
        #[doc = "* `custom_domain`: Domain properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            custom_domain: impl Into<models::AfdDomain>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                custom_domain_name: custom_domain_name.into(),
                custom_domain: custom_domain.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing domain within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `custom_domain_name`: Name of the domain under the profile which is unique globally"]
        #[doc = "* `custom_domain_update_properties`: Domain properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            custom_domain_update_properties: impl Into<models::AfdDomainUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                custom_domain_name: custom_domain_name.into(),
                custom_domain_update_properties: custom_domain_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing AzureFrontDoor domain with the specified domain name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `custom_domain_name`: Name of the domain under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates the domain validation token."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `custom_domain_name`: Name of the domain under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn refresh_validation_token(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> refresh_validation_token::Builder {
            refresh_validation_token::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::AfdDomainListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomainListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AfdDomain;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AfdDomain),
            Created201(models::AfdDomain),
            Accepted202(models::AfdDomain),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) custom_domain: models::AfdDomain,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AfdDomain),
            Accepted202(models::AfdDomain),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) custom_domain_update_properties: models::AfdDomainUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod refresh_validation_token {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/customDomains/{}/refreshValidationToken" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . profile_name , & this . custom_domain_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
}
pub mod afd_endpoints {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists existing AzureFrontDoor endpoints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing AzureFrontDoor endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new AzureFrontDoor endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `endpoint`: Endpoint properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            endpoint: impl Into<models::AfdEndpoint>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                endpoint: endpoint.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing AzureFrontDoor endpoint with the specified endpoint name under the specified subscription, resource group and profile. Only tags can be updated after creating an endpoint. To update origins, use the Update Origin operation. To update origin groups, use the Update Origin group operation. To update domains, use the Update Custom Domain operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `endpoint_update_properties`: Endpoint update properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            endpoint_update_properties: impl Into<models::AfdEndpointUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                endpoint_update_properties: endpoint_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing AzureFrontDoor endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Removes a content from AzureFrontDoor."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `contents`: The list of paths to the content and the list of linked domains to be purged. Path can be a full URL, e.g. '/pictures/city.png' which removes a single file, or a directory with a wildcard, e.g. '/pictures/*' which removes all folders and files in the directory."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn purge_content(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            contents: impl Into<models::AfdPurgeParameters>,
            subscription_id: impl Into<String>,
        ) -> purge_content::Builder {
            purge_content::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                contents: contents.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and actual usage of the given AzureFrontDoor endpoint under the given CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Validates the custom domain mapping to ensure it maps to the correct CDN endpoint in DNS.This api isn't work for apex domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_properties`: Custom domain to be validated."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn validate_custom_domain(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_properties: impl Into<models::ValidateCustomDomainInput>,
            subscription_id: impl Into<String>,
        ) -> validate_custom_domain::Builder {
            validate_custom_domain::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_properties: custom_domain_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::AfdEndpointListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpointListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AfdEndpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AfdEndpoint),
            Created201(models::AfdEndpoint),
            Accepted202(models::AfdEndpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint: models::AfdEndpoint,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AfdEndpoint),
            Accepted202(models::AfdEndpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint_update_properties: models::AfdEndpointUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdEndpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod purge_content {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) contents: models::AfdPurgeParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/purge",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.contents)?;
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
    pub mod list_resource_usage {
        use super::models;
        type Response = models::UsagesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/usages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::UsagesListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod validate_custom_domain {
        use super::models;
        type Response = models::ValidateCustomDomainOutput;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_properties: models::ValidateCustomDomainInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/validateCustomDomain" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . profile_name , & this . endpoint_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ValidateCustomDomainOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod afd_origin_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing origin groups within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing origin group within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new origin group within the specified profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `origin_group`: Origin group properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_group: impl Into<models::AfdOriginGroup>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_group: origin_group.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing origin group within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `origin_group_update_properties`: Origin group properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_group_update_properties: impl Into<models::AfdOriginGroupUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_group_update_properties: origin_group_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing origin group within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and actual usage of the given AzureFrontDoor origin group under the given CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::AfdOriginGroupListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroupListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AfdOriginGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AfdOriginGroup),
            Created201(models::AfdOriginGroup),
            Accepted202(models::AfdOriginGroup),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group: models::AfdOriginGroup,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AfdOriginGroup),
            Accepted202(models::AfdOriginGroup),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group_update_properties: models::AfdOriginGroupUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod list_resource_usage {
        use super::models;
        type Response = models::UsagesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/usages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::UsagesListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod afd_origins {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing origins within an origin group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_origin_group(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_origin_group::Builder {
            list_by_origin_group::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing origin within an origin group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_name: origin_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new origin within the specified origin group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `origin_name`: Name of the origin that is unique within the profile."]
        #[doc = "* `origin`: Origin properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_name: impl Into<String>,
            origin: impl Into<models::AfdOrigin>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_name: origin_name.into(),
                origin: origin.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing origin within an origin group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the profile."]
        #[doc = "* `origin_update_properties`: Origin properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_name: impl Into<String>,
            origin_update_properties: impl Into<models::AfdOriginUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_name: origin_name.into(),
                origin_update_properties: origin_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing origin within an origin group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the profile."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_name: origin_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_origin_group {
        use super::models;
        type Response = models::AfdOriginListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/origins",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOriginListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AfdOrigin;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AfdOrigin),
            Created201(models::AfdOrigin),
            Accepted202(models::AfdOrigin),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin: models::AfdOrigin,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AfdOrigin),
            Accepted202(models::AfdOrigin),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin_update_properties: models::AfdOriginUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AfdOrigin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/originGroups/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.origin_group_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod routes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing origins within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_endpoint(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_endpoint::Builder {
            list_by_endpoint::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing route with the specified route name under the specified subscription, resource group, profile, and AzureFrontDoor endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `route_name`: Name of the routing rule."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            route_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                route_name: route_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new route with the specified route name under the specified subscription, resource group, profile, and AzureFrontDoor endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `route_name`: Name of the routing rule."]
        #[doc = "* `route`: Route properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            route_name: impl Into<String>,
            route: impl Into<models::Route>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                route_name: route_name.into(),
                route: route.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing route with the specified route name under the specified subscription, resource group, profile, and AzureFrontDoor endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `route_name`: Name of the routing rule."]
        #[doc = "* `route_update_properties`: Route update properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            route_name: impl Into<String>,
            route_update_properties: impl Into<models::RouteUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                route_name: route_name.into(),
                route_update_properties: route_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing route with the specified route name under the specified subscription, resource group, profile, and AzureFrontDoor endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `route_name`: Name of the routing rule."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            route_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                route_name: route_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_endpoint {
        use super::models;
        type Response = models::RouteListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/routes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RouteListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Route;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) route_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/routes/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.route_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Route),
            Created201(models::Route),
            Accepted202(models::Route),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) route_name: String,
            pub(crate) route: models::Route,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/routes/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.route_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.route)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Route),
            Accepted202(models::Route),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) route_name: String,
            pub(crate) route_update_properties: models::RouteUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/routes/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.route_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.route_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Route = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) route_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/afdEndpoints/{}/routes/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.route_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod rule_sets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists existing AzureFrontDoor rule sets within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing AzureFrontDoor rule set with the specified rule set name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new rule set within the specified profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile which is unique globally"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing AzureFrontDoor rule set with the specified rule set name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and actual usage of the given AzureFrontDoor rule set under the given CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::RuleSetListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RuleSetListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RuleSet;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RuleSet = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::RuleSet),
            Created201(models::RuleSet),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RuleSet = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RuleSet = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod list_resource_usage {
        use super::models;
        type Response = models::UsagesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/usages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::UsagesListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing delivery rules within a rule set."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_rule_set(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_rule_set::Builder {
            list_by_rule_set::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing delivery rule within a rule set."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile."]
        #[doc = "* `rule_name`: Name of the delivery rule which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                rule_name: rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new delivery rule within the specified rule set."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile."]
        #[doc = "* `rule_name`: Name of the delivery rule which is unique within the endpoint."]
        #[doc = "* `rule`: The delivery rule properties."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            rule_name: impl Into<String>,
            rule: impl Into<models::Rule>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                rule_name: rule_name.into(),
                rule: rule.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing delivery rule within a rule set."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile."]
        #[doc = "* `rule_name`: Name of the delivery rule which is unique within the endpoint."]
        #[doc = "* `rule_update_properties`: Delivery rule properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            rule_name: impl Into<String>,
            rule_update_properties: impl Into<models::RuleUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                rule_name: rule_name.into(),
                rule_update_properties: rule_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing delivery rule within a rule set."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `rule_set_name`: Name of the rule set under the profile."]
        #[doc = "* `rule_name`: Name of the delivery rule which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rule_set_name: impl Into<String>,
            rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rule_set_name: rule_set_name.into(),
                rule_name: rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_rule_set {
        use super::models;
        type Response = models::RuleListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/rules",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RuleListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Rule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/rules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name,
                            &this.rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Rule),
            Created201(models::Rule),
            Accepted202(models::Rule),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) rule_name: String,
            pub(crate) rule: models::Rule,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/rules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name,
                            &this.rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.rule)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Rule),
            Accepted202(models::Rule),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) rule_name: String,
            pub(crate) rule_update_properties: models::RuleUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/rules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name,
                            &this.rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.rule_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Rule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) rule_set_name: String,
            pub(crate) rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/ruleSets/{}/rules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.rule_set_name,
                            &this.rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod security_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists security policies associated with the profile"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing security policy within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `security_policy_name`: Name of the security policy under the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            security_policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                security_policy_name: security_policy_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new security policy within the specified profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `security_policy_name`: Name of the security policy under the profile."]
        #[doc = "* `security_policy`: The security policy properties."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            security_policy_name: impl Into<String>,
            security_policy: impl Into<models::SecurityPolicy>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                security_policy_name: security_policy_name.into(),
                security_policy: security_policy.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing security policy within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `security_policy_name`: Name of the security policy under the profile."]
        #[doc = "* `security_policy_update_properties`: Security policy update properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn patch(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            security_policy_name: impl Into<String>,
            security_policy_update_properties: impl Into<models::SecurityPolicyUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                security_policy_name: security_policy_name.into(),
                security_policy_update_properties: security_policy_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing security policy within profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `security_policy_name`: Name of the security policy under the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            security_policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                security_policy_name: security_policy_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::SecurityPolicyListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/securityPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicyListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SecurityPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) security_policy_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/securityPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.security_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SecurityPolicy),
            Created201(models::SecurityPolicy),
            Accepted202(models::SecurityPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) security_policy_name: String,
            pub(crate) security_policy: models::SecurityPolicy,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/securityPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.security_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.security_policy)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
    pub mod patch {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::SecurityPolicy),
            Accepted202(models::SecurityPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) security_policy_name: String,
            pub(crate) security_policy_update_properties: models::SecurityPolicyUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/securityPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.security_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.security_policy_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) security_policy_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/securityPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.security_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod secrets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists existing AzureFrontDoor secrets."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing Secret within a profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `secret_name`: Name of the Secret under the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            secret_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                secret_name: secret_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new Secret within the specified profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `secret_name`: Name of the Secret under the profile."]
        #[doc = "* `secret`: The Secret properties."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            secret_name: impl Into<String>,
            secret: impl Into<models::Secret>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                secret_name: secret_name.into(),
                secret: secret.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing Secret within profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group."]
        #[doc = "* `secret_name`: Name of the Secret under the profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            secret_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                secret_name: secret_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::SecretListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/secrets",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecretListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Secret;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) secret_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/secrets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.secret_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Secret = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Secret),
            Created201(models::Secret),
            Accepted202(models::Secret),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) secret_name: String,
            pub(crate) secret: models::Secret,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/secrets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.secret_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.secret)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Secret = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Secret = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Secret = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) secret_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/secrets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.secret_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod validate {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Validate a Secret in the profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `validate_secret_input`: The Secret source."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn secret(
            &self,
            validate_secret_input: impl Into<models::ValidateSecretInput>,
            subscription_id: impl Into<String>,
        ) -> secret::Builder {
            secret::Builder {
                client: self.0.clone(),
                validate_secret_input: validate_secret_input.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod secret {
        use super::models;
        type Response = models::ValidateSecretOutput;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) validate_secret_input: models::ValidateSecretInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Cdn/validateSecret",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.validate_secret_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ValidateSecretOutput = serde_json::from_slice(&rsp_body)?;
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
pub mod log_analytics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get log report for AFD profile"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_log_analytics_metrics(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            metrics: Vec<String>,
            date_time_begin: impl Into<time::OffsetDateTime>,
            date_time_end: impl Into<time::OffsetDateTime>,
            granularity: impl Into<String>,
            custom_domains: Vec<String>,
            protocols: Vec<String>,
        ) -> get_log_analytics_metrics::Builder {
            get_log_analytics_metrics::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                metrics,
                date_time_begin: date_time_begin.into(),
                date_time_end: date_time_end.into(),
                granularity: granularity.into(),
                custom_domains,
                protocols,
                group_by: Vec::new(),
                continents: Vec::new(),
                country_or_regions: Vec::new(),
            }
        }
        #[doc = "Get log analytics ranking report for AFD profile"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_log_analytics_rankings(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            rankings: Vec<String>,
            metrics: Vec<String>,
            max_ranking: i32,
            date_time_begin: impl Into<time::OffsetDateTime>,
            date_time_end: impl Into<time::OffsetDateTime>,
        ) -> get_log_analytics_rankings::Builder {
            get_log_analytics_rankings::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                rankings,
                metrics,
                max_ranking,
                date_time_begin: date_time_begin.into(),
                date_time_end: date_time_end.into(),
                custom_domains: Vec::new(),
            }
        }
        #[doc = "Get all available location names for AFD log analytics report."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_log_analytics_locations(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
        ) -> get_log_analytics_locations::Builder {
            get_log_analytics_locations::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
            }
        }
        #[doc = "Get all endpoints and custom domains available for AFD log report"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_log_analytics_resources(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
        ) -> get_log_analytics_resources::Builder {
            get_log_analytics_resources::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
            }
        }
        #[doc = "Get Waf related log analytics report for AFD profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_waf_log_analytics_metrics(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            metrics: Vec<String>,
            date_time_begin: impl Into<time::OffsetDateTime>,
            date_time_end: impl Into<time::OffsetDateTime>,
            granularity: impl Into<String>,
        ) -> get_waf_log_analytics_metrics::Builder {
            get_waf_log_analytics_metrics::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                metrics,
                date_time_begin: date_time_begin.into(),
                date_time_end: date_time_end.into(),
                granularity: granularity.into(),
                actions: Vec::new(),
                group_by: Vec::new(),
                rule_types: Vec::new(),
            }
        }
        #[doc = "Get WAF log analytics charts for AFD profile"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium profile which is unique within the resource group. which is unique within the resource group."]
        pub fn get_waf_log_analytics_rankings(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            metrics: Vec<String>,
            date_time_begin: impl Into<time::OffsetDateTime>,
            date_time_end: impl Into<time::OffsetDateTime>,
            max_ranking: i32,
            rankings: Vec<String>,
        ) -> get_waf_log_analytics_rankings::Builder {
            get_waf_log_analytics_rankings::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                metrics,
                date_time_begin: date_time_begin.into(),
                date_time_end: date_time_end.into(),
                max_ranking,
                rankings,
                actions: Vec::new(),
                rule_types: Vec::new(),
            }
        }
    }
    pub mod get_log_analytics_metrics {
        use super::models;
        type Response = models::MetricsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) metrics: Vec<String>,
            pub(crate) date_time_begin: time::OffsetDateTime,
            pub(crate) date_time_end: time::OffsetDateTime,
            pub(crate) granularity: String,
            pub(crate) custom_domains: Vec<String>,
            pub(crate) protocols: Vec<String>,
            pub(crate) group_by: Vec<String>,
            pub(crate) continents: Vec<String>,
            pub(crate) country_or_regions: Vec<String>,
        }
        impl Builder {
            pub fn group_by(mut self, group_by: Vec<String>) -> Self {
                self.group_by = group_by;
                self
            }
            pub fn continents(mut self, continents: Vec<String>) -> Self {
                self.continents = continents;
                self
            }
            pub fn country_or_regions(mut self, country_or_regions: Vec<String>) -> Self {
                self.country_or_regions = country_or_regions;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getLogAnalyticsMetrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let metrics = &this.metrics;
                        for value in &this.metrics {
                            req.url_mut().query_pairs_mut().append_pair("metrics", &value.to_string());
                        }
                        let date_time_begin = &this.date_time_begin;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeBegin", &date_time_begin.to_string());
                        let date_time_end = &this.date_time_end;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeEnd", &date_time_end.to_string());
                        let granularity = &this.granularity;
                        req.url_mut().query_pairs_mut().append_pair("granularity", granularity);
                        let group_by = &this.group_by;
                        for value in &this.group_by {
                            req.url_mut().query_pairs_mut().append_pair("groupBy", &value.to_string());
                        }
                        let continents = &this.continents;
                        for value in &this.continents {
                            req.url_mut().query_pairs_mut().append_pair("continents", &value.to_string());
                        }
                        let country_or_regions = &this.country_or_regions;
                        for value in &this.country_or_regions {
                            req.url_mut().query_pairs_mut().append_pair("countryOrRegions", &value.to_string());
                        }
                        let custom_domains = &this.custom_domains;
                        for value in &this.custom_domains {
                            req.url_mut().query_pairs_mut().append_pair("customDomains", &value.to_string());
                        }
                        let protocols = &this.protocols;
                        for value in &this.protocols {
                            req.url_mut().query_pairs_mut().append_pair("protocols", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_analytics_rankings {
        use super::models;
        type Response = models::RankingsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) rankings: Vec<String>,
            pub(crate) metrics: Vec<String>,
            pub(crate) max_ranking: i32,
            pub(crate) date_time_begin: time::OffsetDateTime,
            pub(crate) date_time_end: time::OffsetDateTime,
            pub(crate) custom_domains: Vec<String>,
        }
        impl Builder {
            pub fn custom_domains(mut self, custom_domains: Vec<String>) -> Self {
                self.custom_domains = custom_domains;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getLogAnalyticsRankings",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let rankings = &this.rankings;
                        for value in &this.rankings {
                            req.url_mut().query_pairs_mut().append_pair("rankings", &value.to_string());
                        }
                        let metrics = &this.metrics;
                        for value in &this.metrics {
                            req.url_mut().query_pairs_mut().append_pair("metrics", &value.to_string());
                        }
                        let max_ranking = &this.max_ranking;
                        req.url_mut().query_pairs_mut().append_pair("maxRanking", &max_ranking.to_string());
                        let date_time_begin = &this.date_time_begin;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeBegin", &date_time_begin.to_string());
                        let date_time_end = &this.date_time_end;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeEnd", &date_time_end.to_string());
                        let custom_domains = &this.custom_domains;
                        for value in &this.custom_domains {
                            req.url_mut().query_pairs_mut().append_pair("customDomains", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RankingsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_analytics_locations {
        use super::models;
        type Response = models::ContinentsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getLogAnalyticsLocations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ContinentsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_analytics_resources {
        use super::models;
        type Response = models::ResourcesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getLogAnalyticsResources",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourcesResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_waf_log_analytics_metrics {
        use super::models;
        type Response = models::WafMetricsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) metrics: Vec<String>,
            pub(crate) date_time_begin: time::OffsetDateTime,
            pub(crate) date_time_end: time::OffsetDateTime,
            pub(crate) granularity: String,
            pub(crate) actions: Vec<String>,
            pub(crate) group_by: Vec<String>,
            pub(crate) rule_types: Vec<String>,
        }
        impl Builder {
            pub fn actions(mut self, actions: Vec<String>) -> Self {
                self.actions = actions;
                self
            }
            pub fn group_by(mut self, group_by: Vec<String>) -> Self {
                self.group_by = group_by;
                self
            }
            pub fn rule_types(mut self, rule_types: Vec<String>) -> Self {
                self.rule_types = rule_types;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getWafLogAnalyticsMetrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let metrics = &this.metrics;
                        for value in &this.metrics {
                            req.url_mut().query_pairs_mut().append_pair("metrics", &value.to_string());
                        }
                        let date_time_begin = &this.date_time_begin;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeBegin", &date_time_begin.to_string());
                        let date_time_end = &this.date_time_end;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeEnd", &date_time_end.to_string());
                        let granularity = &this.granularity;
                        req.url_mut().query_pairs_mut().append_pair("granularity", granularity);
                        let actions = &this.actions;
                        for value in &this.actions {
                            req.url_mut().query_pairs_mut().append_pair("actions", &value.to_string());
                        }
                        let group_by = &this.group_by;
                        for value in &this.group_by {
                            req.url_mut().query_pairs_mut().append_pair("groupBy", &value.to_string());
                        }
                        let rule_types = &this.rule_types;
                        for value in &this.rule_types {
                            req.url_mut().query_pairs_mut().append_pair("ruleTypes", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WafMetricsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_waf_log_analytics_rankings {
        use super::models;
        type Response = models::WafRankingsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) metrics: Vec<String>,
            pub(crate) date_time_begin: time::OffsetDateTime,
            pub(crate) date_time_end: time::OffsetDateTime,
            pub(crate) max_ranking: i32,
            pub(crate) rankings: Vec<String>,
            pub(crate) actions: Vec<String>,
            pub(crate) rule_types: Vec<String>,
        }
        impl Builder {
            pub fn actions(mut self, actions: Vec<String>) -> Self {
                self.actions = actions;
                self
            }
            pub fn rule_types(mut self, rule_types: Vec<String>) -> Self {
                self.rule_types = rule_types;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getWafLogAnalyticsRankings",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let metrics = &this.metrics;
                        for value in &this.metrics {
                            req.url_mut().query_pairs_mut().append_pair("metrics", &value.to_string());
                        }
                        let date_time_begin = &this.date_time_begin;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeBegin", &date_time_begin.to_string());
                        let date_time_end = &this.date_time_end;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("dateTimeEnd", &date_time_end.to_string());
                        let max_ranking = &this.max_ranking;
                        req.url_mut().query_pairs_mut().append_pair("maxRanking", &max_ranking.to_string());
                        let rankings = &this.rankings;
                        for value in &this.rankings {
                            req.url_mut().query_pairs_mut().append_pair("rankings", &value.to_string());
                        }
                        let actions = &this.actions;
                        for value in &this.actions {
                            req.url_mut().query_pairs_mut().append_pair("actions", &value.to_string());
                        }
                        let rule_types = &this.rule_types;
                        for value in &this.rule_types {
                            req.url_mut().query_pairs_mut().append_pair("ruleTypes", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WafRankingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the Azure Front Door Standard, Azure Front Door Premium, and CDN profiles within an Azure subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all of the Azure Front Door Standard, Azure Front Door Premium, and CDN profiles within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_resource_group(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an Azure Front Door Standard or Azure Front Door Premium or CDN profile with the specified profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new Azure Front Door Standard or Azure Front Door Premium or CDN profile with a profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `profile`: Profile properties needed to create a new profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            profile: impl Into<models::Profile>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                profile: profile.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing Azure Front Door Standard or Azure Front Door Premium or CDN profile with the specified profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `profile_update_parameters`: Profile properties needed to update an existing profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            profile_update_parameters: impl Into<models::ProfileUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                profile_update_parameters: profile_update_parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing  Azure Front Door Standard or Azure Front Door Premium or CDN profile with the specified parameters. Deleting a profile will result in the deletion of all of the sub-resources including endpoints, origins and custom domains."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Generates a dynamic SSO URI used to sign in to the CDN supplemental portal. Supplemental portal is used to configure advanced feature capabilities that are not yet available in the Azure portal, such as core reports in a standard profile; rules engine, advanced HTTP reports, and real-time stats and alerts in a premium profile. The SSO URI changes approximately every 10 minutes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn generate_sso_uri(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> generate_sso_uri::Builder {
            generate_sso_uri::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the supported optimization types for the current profile. A user can create an endpoint with an optimization type from the listed values."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_supported_optimization_types(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_supported_optimization_types::Builder {
            list_supported_optimization_types::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and actual usage of endpoints under the given Azure Front Door Standard or Azure Front Door Premium or CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the Azure Front Door Standard or Azure Front Door Premium or CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProfileListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.Cdn/profiles",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProfileListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_resource_group {
        use super::models;
        type Response = models::ProfileListResult;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProfileListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Profile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Profile),
            Created201(models::Profile),
            Accepted202(models::Profile),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) profile: models::Profile,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.profile)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Profile),
            Accepted202(models::Profile),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) profile_update_parameters: models::ProfileUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.profile_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Profile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod generate_sso_uri {
        use super::models;
        type Response = models::SsoUri;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/generateSsoUri",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SsoUri = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_supported_optimization_types {
        use super::models;
        type Response = models::SupportedOptimizationTypesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/getSupportedOptimizationTypes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SupportedOptimizationTypesListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_resource_usage {
        use super::models;
        type Response = models::ResourceUsageListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/checkResourceUsage",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::ResourceUsageListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod endpoints {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists existing CDN endpoints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_profile(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_profile::Builder {
            list_by_profile::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing CDN endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new CDN endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `endpoint`: Endpoint properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            endpoint: impl Into<models::Endpoint>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                endpoint: endpoint.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing CDN endpoint with the specified endpoint name under the specified subscription, resource group and profile. Only tags can be updated after creating an endpoint. To update origins, use the Update Origin operation. To update origin groups, use the Update Origin group operation. To update custom domains, use the Update Custom Domain operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `endpoint_update_properties`: Endpoint update properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            endpoint_update_properties: impl Into<models::EndpointUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                endpoint_update_properties: endpoint_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing CDN endpoint with the specified endpoint name under the specified subscription, resource group and profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Starts an existing CDN endpoint that is on a stopped state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn start(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> start::Builder {
            start::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Stops an existing running CDN endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn stop(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> stop::Builder {
            stop::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Removes a content from CDN."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `content_file_paths`: The path to the content to be purged. Path can be a full URL, e.g. '/pictures/city.png' which removes a single file, or a directory with a wildcard, e.g. '/pictures/*' which removes all folders and files in the directory."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn purge_content(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            content_file_paths: impl Into<models::PurgeParameters>,
            subscription_id: impl Into<String>,
        ) -> purge_content::Builder {
            purge_content::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                content_file_paths: content_file_paths.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Pre-loads a content to CDN. Available for Verizon Profiles."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `content_file_paths`: The path to the content to be loaded. Path should be a full URL, e.g. ‘/pictures/city.png' which loads a single file "]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn load_content(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            content_file_paths: impl Into<models::LoadParameters>,
            subscription_id: impl Into<String>,
        ) -> load_content::Builder {
            load_content::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                content_file_paths: content_file_paths.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Validates the custom domain mapping to ensure it maps to the correct CDN endpoint in DNS."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_properties`: Custom domain to be validated."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn validate_custom_domain(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_properties: impl Into<models::ValidateCustomDomainInput>,
            subscription_id: impl Into<String>,
        ) -> validate_custom_domain::Builder {
            validate_custom_domain::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_properties: custom_domain_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and usage of geo filters and custom domains under the given endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::Builder {
            list_resource_usage::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_profile {
        use super::models;
        type Response = models::EndpointListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EndpointListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Endpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Endpoint),
            Created201(models::Endpoint),
            Accepted202(models::Endpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint: models::Endpoint,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Endpoint),
            Accepted202(models::Endpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint_update_properties: models::EndpointUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
    pub mod start {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Endpoint),
            Accepted202(models::Endpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/start",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
    pub mod stop {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Endpoint),
            Accepted202(models::Endpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/stop",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Endpoint = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
    pub mod purge_content {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) content_file_paths: models::PurgeParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/purge",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.content_file_paths)?;
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
    pub mod load_content {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) content_file_paths: models::LoadParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/load",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.content_file_paths)?;
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
    pub mod validate_custom_domain {
        use super::models;
        type Response = models::ValidateCustomDomainOutput;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_properties: models::ValidateCustomDomainInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/validateCustomDomain",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ValidateCustomDomainOutput = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_resource_usage {
        use super::models;
        type Response = models::ResourceUsageListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/checkResourceUsage",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::ResourceUsageListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod origins {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing origins within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_endpoint(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_endpoint::Builder {
            list_by_endpoint::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing origin within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_name: origin_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new origin within the specified endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_name`: Name of the origin that is unique within the endpoint."]
        #[doc = "* `origin`: Origin properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_name: impl Into<String>,
            origin: impl Into<models::Origin>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_name: origin_name.into(),
                origin: origin.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing origin within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the endpoint."]
        #[doc = "* `origin_update_properties`: Origin properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_name: impl Into<String>,
            origin_update_properties: impl Into<models::OriginUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_name: origin_name.into(),
                origin_update_properties: origin_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing origin within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_name`: Name of the origin which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_name: origin_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_endpoint {
        use super::models;
        type Response = models::OriginListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/origins",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Origin;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Origin),
            Created201(models::Origin),
            Accepted202(models::Origin),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin: models::Origin,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Origin),
            Accepted202(models::Origin),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin_update_properties: models::OriginUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Origin = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/origins/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod origin_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing origin groups within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_endpoint(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_endpoint::Builder {
            list_by_endpoint::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing origin group within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new origin group within the specified endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `origin_group`: Origin group properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_group: impl Into<models::OriginGroup>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_group: origin_group.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing origin group within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `origin_group_update_properties`: Origin group properties"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            origin_group_update_properties: impl Into<models::OriginGroupUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_group_name: origin_group_name.into(),
                origin_group_update_properties: origin_group_update_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing origin group within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `origin_group_name`: Name of the origin group which is unique within the endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            origin_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                origin_group_name: origin_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_endpoint {
        use super::models;
        type Response = models::OriginGroupListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/originGroups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroupListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::OriginGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::OriginGroup),
            Created201(models::OriginGroup),
            Accepted202(models::OriginGroup),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group: models::OriginGroup,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::OriginGroup),
            Accepted202(models::OriginGroup),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group_update_properties: models::OriginGroupUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group_update_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OriginGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/originGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.origin_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod custom_domains {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the existing custom domains within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_endpoint(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_endpoint::Builder {
            list_by_endpoint::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an existing custom domain within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_name`: Name of the custom domain within an endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new custom domain within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_name`: Name of the custom domain within an endpoint."]
        #[doc = "* `custom_domain_properties`: Properties required to create a new custom domain."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            custom_domain_properties: impl Into<models::CustomDomainParameters>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_name: custom_domain_name.into(),
                custom_domain_properties: custom_domain_properties.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing custom domain within an endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_name`: Name of the custom domain within an endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Disable https delivery of the custom domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_name`: Name of the custom domain within an endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn disable_custom_https(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> disable_custom_https::Builder {
            disable_custom_https::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Enable https delivery of the custom domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `endpoint_name`: Name of the endpoint under the profile which is unique globally."]
        #[doc = "* `custom_domain_name`: Name of the custom domain within an endpoint."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn enable_custom_https(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_name: impl Into<String>,
            custom_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> enable_custom_https::Builder {
            enable_custom_https::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                custom_domain_name: custom_domain_name.into(),
                subscription_id: subscription_id.into(),
                custom_domain_https_parameters: None,
            }
        }
    }
    pub mod list_by_endpoint {
        use super::models;
        type Response = models::CustomDomainListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomainListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::CustomDomain;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::CustomDomain),
            Created201(models::CustomDomain),
            Accepted202(models::CustomDomain),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) custom_domain_properties: models::CustomDomainParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            Accepted202(models::CustomDomain),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_name,
                            &this.custom_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
    pub mod disable_custom_https {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::CustomDomain),
            Accepted202(models::CustomDomain),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains/{}/disableCustomHttps" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . profile_name , & this . endpoint_name , & this . custom_domain_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
    pub mod enable_custom_https {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::CustomDomain),
            Accepted202(models::CustomDomain),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
            pub(crate) custom_domain_https_parameters: Option<models::CustomDomainHttpsParameters>,
        }
        impl Builder {
            #[doc = "The configuration specifying how to enable HTTPS for the custom domain - using CDN managed certificate or user's own certificate. If not specified, enabling ssl uses CDN managed certificate by default."]
            pub fn custom_domain_https_parameters(
                mut self,
                custom_domain_https_parameters: impl Into<models::CustomDomainHttpsParameters>,
            ) -> Self {
                self.custom_domain_https_parameters = Some(custom_domain_https_parameters.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/profiles/{}/endpoints/{}/customDomains/{}/enableCustomHttps" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . profile_name , & this . endpoint_name , & this . custom_domain_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = if let Some(custom_domain_https_parameters) = &this.custom_domain_https_parameters {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(custom_domain_https_parameters)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomDomain = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
pub mod resource_usage {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check the quota and actual usage of the CDN profiles under the given subscription."]
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
        type Response = models::ResourceUsageListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.Cdn/checkResourceUsage",
                            this.client.endpoint(),
                            &this.subscription_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                let rsp_value: models::ResourceUsageListResult = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Lists all of the available CDN REST API operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Cdn/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod edge_nodes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Edgenodes are the global Point of Presence (POP) locations used to deliver CDN content to end users."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::EdgenodeResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Cdn/edgenodes", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EdgenodeResult = serde_json::from_slice(&rsp_body)?;
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
pub mod policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the protection policies within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list(&self, resource_group_name: impl Into<String>, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve protection policy with specified name within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `policy_name`: The name of the CdnWebApplicationFirewallPolicy."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                policy_name: policy_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Create or update policy with specified rule set name within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `policy_name`: The name of the CdnWebApplicationFirewallPolicy."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `cdn_web_application_firewall_policy`: Policy to be created."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
            cdn_web_application_firewall_policy: impl Into<models::CdnWebApplicationFirewallPolicy>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                policy_name: policy_name.into(),
                subscription_id: subscription_id.into(),
                cdn_web_application_firewall_policy: cdn_web_application_firewall_policy.into(),
            }
        }
        #[doc = "Update an existing CdnWebApplicationFirewallPolicy with the specified policy name under the specified subscription and resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `policy_name`: The name of the CdnWebApplicationFirewallPolicy."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        #[doc = "* `cdn_web_application_firewall_policy_patch_parameters`: CdnWebApplicationFirewallPolicy parameters to be patched."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
            cdn_web_application_firewall_policy_patch_parameters: impl Into<models::CdnWebApplicationFirewallPolicyPatchParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                policy_name: policy_name.into(),
                subscription_id: subscription_id.into(),
                cdn_web_application_firewall_policy_patch_parameters: cdn_web_application_firewall_policy_patch_parameters.into(),
            }
        }
        #[doc = "Deletes Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `policy_name`: The name of the CdnWebApplicationFirewallPolicy."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                policy_name: policy_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CdnWebApplicationFirewallPolicyList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallPolicies",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicyList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::CdnWebApplicationFirewallPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) policy_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::CdnWebApplicationFirewallPolicy),
            Created201(models::CdnWebApplicationFirewallPolicy),
            Accepted202(models::CdnWebApplicationFirewallPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) policy_name: String,
            pub(crate) subscription_id: String,
            pub(crate) cdn_web_application_firewall_policy: models::CdnWebApplicationFirewallPolicy,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.cdn_web_application_firewall_policy)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::CdnWebApplicationFirewallPolicy),
            Accepted202(models::CdnWebApplicationFirewallPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) policy_name: String,
            pub(crate) subscription_id: String,
            pub(crate) cdn_web_application_firewall_policy_patch_parameters: models::CdnWebApplicationFirewallPolicyPatchParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.cdn_web_application_firewall_policy_patch_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CdnWebApplicationFirewallPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) policy_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
pub mod managed_rule_sets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all available managed rule sets."]
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
        type Response = models::ManagedRuleSetDefinitionList;
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
                            "{}/subscriptions/{}/providers/Microsoft.Cdn/cdnWebApplicationFirewallManagedRuleSets",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagedRuleSetDefinitionList = serde_json::from_slice(&rsp_body)?;
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
