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
    pub fn endpoints_client(&self) -> endpoints::Client {
        endpoints::Client(self.clone())
    }
    pub fn geographic_hierarchies_client(&self) -> geographic_hierarchies::Client {
        geographic_hierarchies::Client(self.clone())
    }
    pub fn heat_map_client(&self) -> heat_map::Client {
        heat_map::Client(self.clone())
    }
    pub fn profiles_client(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
}
pub mod endpoints {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a Traffic Manager endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager endpoint."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `endpoint_type`: The type of the Traffic Manager endpoint."]
        #[doc = "* `endpoint_name`: The name of the Traffic Manager endpoint."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_type: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_type: endpoint_type.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Create or update a Traffic Manager endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager endpoint to be created or updated."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `endpoint_type`: The type of the Traffic Manager endpoint to be created or updated."]
        #[doc = "* `endpoint_name`: The name of the Traffic Manager endpoint to be created or updated."]
        #[doc = "* `parameters`: The Traffic Manager endpoint parameters supplied to the CreateOrUpdate operation."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_type: impl Into<String>,
            endpoint_name: impl Into<String>,
            parameters: impl Into<models::Endpoint>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_type: endpoint_type.into(),
                endpoint_name: endpoint_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Update a Traffic Manager endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager endpoint to be updated."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `endpoint_type`: The type of the Traffic Manager endpoint to be updated."]
        #[doc = "* `endpoint_name`: The name of the Traffic Manager endpoint to be updated."]
        #[doc = "* `parameters`: The Traffic Manager endpoint parameters supplied to the Update operation."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_type: impl Into<String>,
            endpoint_name: impl Into<String>,
            parameters: impl Into<models::Endpoint>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_type: endpoint_type.into(),
                endpoint_name: endpoint_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a Traffic Manager endpoint."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager endpoint to be deleted."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `endpoint_type`: The type of the Traffic Manager endpoint to be deleted."]
        #[doc = "* `endpoint_name`: The name of the Traffic Manager endpoint to be deleted."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            endpoint_type: impl Into<String>,
            endpoint_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_type: endpoint_type.into(),
                endpoint_name: endpoint_name.into(),
                subscription_id: subscription_id.into(),
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
            pub(crate) endpoint_type: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_type,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Endpoint),
            Created201(models::Endpoint),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_type: String,
            pub(crate) endpoint_name: String,
            pub(crate) parameters: models::Endpoint,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_type,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
        type Response = models::Endpoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_type: String,
            pub(crate) endpoint_name: String,
            pub(crate) parameters: models::Endpoint,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_type,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DeleteOperationResult),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_type: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/{}/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.endpoint_type,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeleteOperationResult = serde_json::from_slice(&rsp_body)?;
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
pub mod profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Checks the availability of a Traffic Manager Relative DNS name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The Traffic Manager name parameters supplied to the CheckTrafficManagerNameAvailability operation."]
        pub fn check_traffic_manager_relative_dns_name_availability(
            &self,
            parameters: impl Into<models::CheckTrafficManagerRelativeDnsNameAvailabilityParameters>,
        ) -> check_traffic_manager_relative_dns_name_availability::Builder {
            check_traffic_manager_relative_dns_name_availability::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Lists all Traffic Manager profiles within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager profiles to be listed."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
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
        #[doc = "Lists all Traffic Manager profiles within a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets a Traffic Manager profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager profile."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
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
        #[doc = "Create or update a Traffic Manager profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager profile."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `parameters`: The Traffic Manager profile parameters supplied to the CreateOrUpdate operation."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            parameters: impl Into<models::Profile>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Update a Traffic Manager profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager profile."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `parameters`: The Traffic Manager profile parameters supplied to the Update operation."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            parameters: impl Into<models::Profile>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a Traffic Manager profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager profile to be deleted."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile to be deleted."]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
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
    }
    pub mod check_traffic_manager_relative_dns_name_availability {
        use super::models;
        type Response = models::TrafficManagerNameAvailability;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::CheckTrafficManagerRelativeDnsNameAvailabilityParameters,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Network/checkTrafficManagerNameAvailability",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TrafficManagerNameAvailability = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProfileListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
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
                })
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::ProfileListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.Network/trafficmanagerprofiles",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
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
                })
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Profile),
            Created201(models::Profile),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) parameters: models::Profile,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
        type Response = models::Profile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) parameters: models::Profile,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DeleteOperationResult),
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
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeleteOperationResult = serde_json::from_slice(&rsp_body)?;
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
pub mod geographic_hierarchies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the default Geographic Hierarchy used by the Geographic traffic routing method."]
        pub fn get_default(&self) -> get_default::Builder {
            get_default::Builder { client: self.0.clone() }
        }
    }
    pub mod get_default {
        use super::models;
        type Response = models::TrafficManagerGeographicHierarchy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Network/trafficManagerGeographicHierarchies/default",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TrafficManagerGeographicHierarchy = serde_json::from_slice(&rsp_body)?;
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
pub mod heat_map {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets latest heatmap for Traffic Manager profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Gets subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group containing the Traffic Manager endpoint."]
        #[doc = "* `profile_name`: The name of the Traffic Manager profile."]
        #[doc = "* `heat_map_type`: The type of HeatMap for the Traffic Manager profile."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            heat_map_type: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                heat_map_type: heat_map_type.into(),
                top_left: Vec::new(),
                bot_right: Vec::new(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::HeatMapModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) heat_map_type: String,
            pub(crate) top_left: Vec<f64>,
            pub(crate) bot_right: Vec<f64>,
        }
        impl Builder {
            #[doc = "The top left latitude,longitude pair of the rectangular viewport to query for."]
            pub fn top_left(mut self, top_left: Vec<f64>) -> Self {
                self.top_left = top_left;
                self
            }
            #[doc = "The bottom right latitude,longitude pair of the rectangular viewport to query for."]
            pub fn bot_right(mut self, bot_right: Vec<f64>) -> Self {
                self.bot_right = bot_right;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/trafficmanagerprofiles/{}/heatMaps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.profile_name,
                            &this.heat_map_type
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HeatMapModel = serde_json::from_slice(&rsp_body)?;
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
