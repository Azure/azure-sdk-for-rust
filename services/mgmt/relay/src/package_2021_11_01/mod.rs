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
    pub fn hybrid_connections_client(&self) -> hybrid_connections::Client {
        hybrid_connections::Client(self.clone())
    }
    pub fn namespaces_client(&self) -> namespaces::Client {
        namespaces::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn private_endpoint_connections_client(&self) -> private_endpoint_connections::Client {
        private_endpoint_connections::Client(self.clone())
    }
    pub fn private_link_resources_client(&self) -> private_link_resources::Client {
        private_link_resources::Client(self.clone())
    }
    pub fn wcf_relays_client(&self) -> wcf_relays::Client {
        wcf_relays::Client(self.clone())
    }
}
pub mod namespaces {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Authorization rules for a namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_authorization_rules(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_authorization_rules::Builder {
            list_authorization_rules::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Authorization rule for a namespace by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get_authorization_rule::Builder {
            get_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates an authorization rule for a namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: The authorization rule parameters."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::AuthorizationRule>,
            subscription_id: impl Into<String>,
        ) -> create_or_update_authorization_rule::Builder {
            create_or_update_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a namespace authorization rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete_authorization_rule::Builder {
            delete_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Primary and secondary connection strings to the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_keys::Builder {
            list_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Regenerates the primary or secondary connection strings to the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: Parameters supplied to regenerate authorization rule."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn regenerate_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::RegenerateAccessKeyParameters>,
            subscription_id: impl Into<String>,
        ) -> regenerate_keys::Builder {
            regenerate_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Check the specified namespace name availability."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `parameters`: Parameters to check availability of the specified namespace name."]
        pub fn check_name_availability(
            &self,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CheckNameAvailability>,
        ) -> check_name_availability::Builder {
            check_name_availability::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Lists all the available namespaces within the subscription regardless of the resourceGroups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all the available namespaces within the ResourceGroup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
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
        #[doc = "Returns the description for the specified namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Create Azure Relay namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `parameters`: Parameters supplied to create a namespace resource."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            parameters: impl Into<models::RelayNamespace>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates a namespace. Once created, this namespace's resource manifest is immutable. This operation is idempotent."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `parameters`: Parameters for updating a namespace resource."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            parameters: impl Into<models::RelayUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing namespace. This operation also removes all associated resources under the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets NetworkRuleSet for a Namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get_network_rule_set(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get_network_rule_set::Builder {
            get_network_rule_set::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Create or update NetworkRuleSet for a Namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `parameters`: The Namespace IpFilterRule."]
        pub fn create_or_update_network_rule_set(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::NetworkRuleSet>,
        ) -> create_or_update_network_rule_set::Builder {
            create_or_update_network_rule_set::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod list_authorization_rules {
        use super::models;
        type Response = models::AuthorizationRuleListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRuleListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.authorization_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::AuthorizationRule,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.authorization_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_authorization_rule {
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
            pub(crate) namespace_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.authorization_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
    pub mod list_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules/{}/listKeys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.authorization_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::RegenerateAccessKeyParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/authorizationRules/{}/regenerateKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::CheckNameAvailabilityResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CheckNameAvailability,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Relay/checkNameAvailability",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckNameAvailabilityResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RelayNamespaceListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.Relay/namespaces",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespaceListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RelayNamespaceListResult;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespaceListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RelayNamespace;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespace = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::RelayNamespace),
            Created201(models::RelayNamespace),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) parameters: models::RelayNamespace,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespace = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespace = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RelayNamespace),
            Created201(models::RelayNamespace),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) parameters: models::RelayUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespace = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RelayNamespace = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
    pub mod get_network_rule_set {
        use super::models;
        type Response = models::NetworkRuleSet;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/networkRuleSets/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkRuleSet = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_network_rule_set {
        use super::models;
        type Response = models::NetworkRuleSet;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::NetworkRuleSet,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/networkRuleSets/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkRuleSet = serde_json::from_slice(&rsp_body)?;
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
pub mod hybrid_connections {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Authorization rules for a hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_authorization_rules(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_authorization_rules::Builder {
            list_authorization_rules::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Hybrid connection authorization rule for a hybrid connection by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get_authorization_rule::Builder {
            get_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates an authorization rule for a hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: The authorization rule parameters."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::AuthorizationRule>,
            subscription_id: impl Into<String>,
        ) -> create_or_update_authorization_rule::Builder {
            create_or_update_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a hybrid connection authorization rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete_authorization_rule::Builder {
            delete_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Primary and secondary connection strings to the hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_keys::Builder {
            list_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Regenerates the primary or secondary connection strings to the hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: Parameters supplied to regenerate authorization rule."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn regenerate_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::RegenerateAccessKeyParameters>,
            subscription_id: impl Into<String>,
        ) -> regenerate_keys::Builder {
            regenerate_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists the hybrid connection within the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_by_namespace(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_namespace::Builder {
            list_by_namespace::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Returns the description for the specified hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates a service hybrid connection. This operation is idempotent."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `parameters`: Parameters supplied to create a hybrid connection."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            parameters: impl Into<models::HybridConnection>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a hybrid connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `hybrid_connection_name`: The hybrid connection name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            hybrid_connection_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                hybrid_connection_name: hybrid_connection_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_authorization_rules {
        use super::models;
        type Response = models::AuthorizationRuleListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRuleListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::AuthorizationRule,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_authorization_rule {
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
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
    pub mod list_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules/{}/listKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::RegenerateAccessKeyParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}/authorizationRules/{}/regenerateKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . hybrid_connection_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_namespace {
        use super::models;
        type Response = models::HybridConnectionListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HybridConnectionListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::HybridConnection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.hybrid_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HybridConnection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::HybridConnection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) parameters: models::HybridConnection,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.hybrid_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HybridConnection = serde_json::from_slice(&rsp_body)?;
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
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) hybrid_connection_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/hybridConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.hybrid_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
pub mod wcf_relays {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Authorization rules for a WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_authorization_rules(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_authorization_rules::Builder {
            list_authorization_rules::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Get authorizationRule for a WCF relay by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get_authorization_rule::Builder {
            get_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates an authorization rule for a WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: The authorization rule parameters."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::AuthorizationRule>,
            subscription_id: impl Into<String>,
        ) -> create_or_update_authorization_rule::Builder {
            create_or_update_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a WCF relay authorization rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete_authorization_rule(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete_authorization_rule::Builder {
            delete_authorization_rule::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Primary and secondary connection strings to the WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_keys::Builder {
            list_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Regenerates the primary or secondary connection strings to the WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `authorization_rule_name`: The authorization rule name."]
        #[doc = "* `parameters`: Parameters supplied to regenerate authorization rule."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn regenerate_keys(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            authorization_rule_name: impl Into<String>,
            parameters: impl Into<models::RegenerateAccessKeyParameters>,
            subscription_id: impl Into<String>,
        ) -> regenerate_keys::Builder {
            regenerate_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                authorization_rule_name: authorization_rule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists the WCF relays within the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_by_namespace(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_namespace::Builder {
            list_by_namespace::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Returns the description for the specified WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates a WCF relay. This operation is idempotent."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `parameters`: Parameters supplied to create a WCF relay."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            parameters: impl Into<models::WcfRelay>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a WCF relay."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `relay_name`: The relay name."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            relay_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                relay_name: relay_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_authorization_rules {
        use super::models;
        type Response = models::AuthorizationRuleListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.relay_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRuleListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . relay_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_authorization_rule {
        use super::models;
        type Response = models::AuthorizationRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::AuthorizationRule,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . relay_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthorizationRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_authorization_rule {
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
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . relay_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
    pub mod list_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules/{}/listKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . relay_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_keys {
        use super::models;
        type Response = models::AccessKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) authorization_rule_name: String,
            pub(crate) parameters: models::RegenerateAccessKeyParameters,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}/authorizationRules/{}/regenerateKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . namespace_name , & this . relay_name , & this . authorization_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_namespace {
        use super::models;
        type Response = models::WcfRelaysListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WcfRelaysListResult = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::WcfRelay),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.relay_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WcfRelay = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        type Response = models::WcfRelay;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) parameters: models::WcfRelay,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.relay_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WcfRelay = serde_json::from_slice(&rsp_body)?;
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
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) relay_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/wcfRelays/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.relay_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
pub mod private_endpoint_connections {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the available PrivateEndpointConnections within a namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets a description for the specified Private Endpoint Connection name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `private_endpoint_connection_name`: The PrivateEndpointConnection name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates PrivateEndpointConnections of service namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `private_endpoint_connection_name`: The PrivateEndpointConnection name"]
        #[doc = "* `parameters`: Parameters supplied to update Status of PrivateEndPoint Connection to namespace resource."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
            parameters: impl Into<models::PrivateEndpointConnection>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes an existing namespace. This operation also removes all associated resources under the namespace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `private_endpoint_connection_name`: The PrivateEndpointConnection name"]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PrivateEndpointConnectionListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateEndpointConnections",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnectionListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::PrivateEndpointConnection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) private_endpoint_connection_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateEndpointConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.private_endpoint_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::PrivateEndpointConnection),
            Created201(models::PrivateEndpointConnection),
            Accepted202(models::PrivateEndpointConnection),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) private_endpoint_connection_name: String,
            pub(crate) parameters: models::PrivateEndpointConnection,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateEndpointConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.private_endpoint_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
            pub(crate) private_endpoint_connection_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateEndpointConnections/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.private_endpoint_connection_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
pub mod private_link_resources {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a description for the specified Private Endpoint Connection name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `private_link_resource_name`: The PrivateLinkResource name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            private_link_resource_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                private_link_resource_name: private_link_resource_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets lists of resources that supports Privatelinks."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `namespace_name`: The namespace name"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list(
            &self,
            resource_group_name: impl Into<String>,
            namespace_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                namespace_name: namespace_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::PrivateLinkResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) private_link_resource_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateLinkResources/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name,
                            &this.private_link_resource_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateLinkResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::PrivateLinkResourcesListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) namespace_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Relay/namespaces/{}/privateLinkResources",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.namespace_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateLinkResourcesListResult = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Lists all available Relay REST API operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Relay/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-01");
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
