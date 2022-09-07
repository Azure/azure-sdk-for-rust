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
    pub fn custom_domains_client(&self) -> custom_domains::Client {
        custom_domains::Client(self.clone())
    }
    pub fn edge_nodes_client(&self) -> edge_nodes::Client {
        edge_nodes::Client(self.clone())
    }
    pub fn endpoints_client(&self) -> endpoints::Client {
        endpoints::Client(self.clone())
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
    pub fn profiles_client(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
    pub fn resource_usage_client(&self) -> resource_usage::Client {
        resource_usage::Client(self.clone())
    }
}
pub mod profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the CDN profiles within an Azure subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all of the CDN profiles within a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_by_resource_group(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_resource_group::RequestBuilder {
            list_by_resource_group::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets a CDN profile with the specified profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new CDN profile with a profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `profile`: Profile properties needed to create a new profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            profile: impl Into<models::Profile>,
            subscription_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                profile: profile.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing CDN profile with the specified profile name under the specified subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `profile_update_parameters`: Profile properties needed to update an existing profile."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            profile_update_parameters: impl Into<models::ProfileUpdateParameters>,
            subscription_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                profile_update_parameters: profile_update_parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes an existing CDN profile with the specified parameters. Deleting a profile will result in the deletion of all of the sub-resources including endpoints, origins and custom domains."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
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
        ) -> generate_sso_uri::RequestBuilder {
            generate_sso_uri::RequestBuilder {
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
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_supported_optimization_types(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_supported_optimization_types::RequestBuilder {
            list_supported_optimization_types::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Checks the quota and actual usage of endpoints under the given CDN profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the Resource group within the Azure subscription."]
        #[doc = "* `profile_name`: Name of the CDN profile which is unique within the resource group."]
        #[doc = "* `subscription_id`: Azure Subscription ID."]
        pub fn list_resource_usage(
            &self,
            resource_group_name: impl Into<String>,
            profile_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_resource_usage::RequestBuilder {
            list_resource_usage::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProfileListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProfileListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ProfileListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProfileListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProfileListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ProfileListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Profile = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Profile = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) profile: models::Profile,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.profile)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Profile = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) profile_update_parameters: models::ProfileUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.profile_update_parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Profile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod generate_sso_uri {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SsoUri> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SsoUri = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SsoUri> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod list_supported_optimization_types {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SupportedOptimizationTypesListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SupportedOptimizationTypesListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SupportedOptimizationTypesListResult> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod list_resource_usage {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceUsageListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceUsageListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ResourceUsageListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
        ) -> list_by_profile::RequestBuilder {
            list_by_profile::RequestBuilder {
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
        ) -> get::RequestBuilder {
            get::RequestBuilder {
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
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                profile_name: profile_name.into(),
                endpoint_name: endpoint_name.into(),
                endpoint: endpoint.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates an existing CDN endpoint with the specified endpoint name under the specified subscription, resource group and profile. To update origins, use the Update Origin operation. To update origin groups, use the Update Origin Group operation. To update custom domains, use the Update Custom Domain operation."]
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
        ) -> update::RequestBuilder {
            update::RequestBuilder {
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
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
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
        ) -> start::RequestBuilder {
            start::RequestBuilder {
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
        ) -> stop::RequestBuilder {
            stop::RequestBuilder {
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
        ) -> purge_content::RequestBuilder {
            purge_content::RequestBuilder {
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
        ) -> load_content::RequestBuilder {
            load_content::RequestBuilder {
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
        ) -> validate_custom_domain::RequestBuilder {
            validate_custom_domain::RequestBuilder {
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
        ) -> list_resource_usage::RequestBuilder {
            list_resource_usage::RequestBuilder {
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
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EndpointListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::EndpointListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::EndpointListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Endpoint = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Endpoint = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint: models::Endpoint,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Endpoint = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) endpoint_update_properties: models::EndpointUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.endpoint_update_properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod start {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Endpoint = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod stop {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Endpoint = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Endpoint> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod purge_content {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) content_file_paths: models::PurgeParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.content_file_paths)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod load_content {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) content_file_paths: models::LoadParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.content_file_paths)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod validate_custom_domain {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ValidateCustomDomainOutput> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ValidateCustomDomainOutput = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_properties: models::ValidateCustomDomainInput,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ValidateCustomDomainOutput> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod list_resource_usage {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceUsageListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceUsageListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ResourceUsageListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
        ) -> list_by_endpoint::RequestBuilder {
            list_by_endpoint::RequestBuilder {
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
        ) -> get::RequestBuilder {
            get::RequestBuilder {
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
        ) -> create::RequestBuilder {
            create::RequestBuilder {
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
        ) -> update::RequestBuilder {
            update::RequestBuilder {
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
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
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
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OriginListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OriginListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::OriginListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Origin = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Origin = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin: models::Origin,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Origin = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) origin_update_properties: models::OriginUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_update_properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Origin> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
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
        ) -> list_by_endpoint::RequestBuilder {
            list_by_endpoint::RequestBuilder {
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
        ) -> get::RequestBuilder {
            get::RequestBuilder {
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
        ) -> create::RequestBuilder {
            create::RequestBuilder {
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
        ) -> update::RequestBuilder {
            update::RequestBuilder {
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
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
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
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroupListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OriginGroupListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::OriginGroupListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OriginGroup = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OriginGroup = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group: models::OriginGroup,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OriginGroup = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) origin_group_update_properties: models::OriginGroupUpdateParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.origin_group_update_properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::OriginGroup> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) origin_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
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
        ) -> list_by_endpoint::RequestBuilder {
            list_by_endpoint::RequestBuilder {
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
        ) -> get::RequestBuilder {
            get::RequestBuilder {
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
        ) -> create::RequestBuilder {
            create::RequestBuilder {
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
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
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
        ) -> disable_custom_https::RequestBuilder {
            disable_custom_https::RequestBuilder {
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
        ) -> enable_custom_https::RequestBuilder {
            enable_custom_https::RequestBuilder {
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
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomDomainListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomDomainListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::CustomDomainListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomDomain> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomDomain = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::CustomDomain> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomDomain> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomDomain = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) custom_domain_properties: models::CustomDomainParameters,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.custom_domain_properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::CustomDomain> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod disable_custom_https {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod enable_custom_https {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) profile_name: String,
            pub(crate) endpoint_name: String,
            pub(crate) custom_domain_name: String,
            pub(crate) subscription_id: String,
            pub(crate) custom_domain_https_parameters: Option<models::CustomDomainHttpsParameters>,
        }
        impl RequestBuilder {
            #[doc = "The configuration specifying how to enable HTTPS for the custom domain - using CDN managed certificate or user's own certificate. If not specified, enabling ssl uses CDN managed certificate by default."]
            pub fn custom_domain_https_parameters(
                mut self,
                custom_domain_https_parameters: impl Into<models::CustomDomainHttpsParameters>,
            ) -> Self {
                self.custom_domain_https_parameters = Some(custom_domain_https_parameters.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                        let req_body = if let Some(custom_domain_https_parameters) = &this.custom_domain_https_parameters {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(custom_domain_https_parameters)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
impl Client {
    #[doc = "Check the availability of a resource name. This is needed for resources where name is globally unique, such as a CDN endpoint."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `check_name_availability_input`: Input to check."]
    pub fn check_name_availability(
        &self,
        check_name_availability_input: impl Into<models::CheckNameAvailabilityInput>,
    ) -> check_name_availability::RequestBuilder {
        check_name_availability::RequestBuilder {
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
    ) -> check_name_availability_with_subscription::RequestBuilder {
        check_name_availability_with_subscription::RequestBuilder {
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
    ) -> validate_probe::RequestBuilder {
        validate_probe::RequestBuilder {
            client: self.clone(),
            validate_probe_input: validate_probe_input.into(),
            subscription_id: subscription_id.into(),
        }
    }
}
pub mod check_name_availability {
    use super::models;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::CheckNameAvailabilityOutput = serde_json::from_slice(&bytes)?;
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
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
    }
    impl RequestBuilder {
        #[doc = "Send the request and returns the response."]
        pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        #[doc = "Send the request and return the response body."]
        pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
            self.send().await?.into_body().await
        }
    }
}
pub mod check_name_availability_with_subscription {
    use super::models;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::CheckNameAvailabilityOutput = serde_json::from_slice(&bytes)?;
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
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
        pub(crate) subscription_id: String,
    }
    impl RequestBuilder {
        #[doc = "Send the request and returns the response."]
        pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        #[doc = "Send the request and return the response body."]
        pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
            self.send().await?.into_body().await
        }
    }
}
pub mod validate_probe {
    use super::models;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::ValidateProbeOutput> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::ValidateProbeOutput = serde_json::from_slice(&bytes)?;
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
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) validate_probe_input: models::ValidateProbeInput,
        pub(crate) subscription_id: String,
    }
    impl RequestBuilder {
        #[doc = "Send the request and returns the response."]
        pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
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
                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.validate_probe_input)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        #[doc = "Send the request and return the response body."]
        pub async fn into_body(self) -> azure_core::Result<models::ValidateProbeOutput> {
            self.send().await?.into_body().await
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
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceUsageListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceUsageListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::ResourceUsageListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
        }
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available CDN REST API operations."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationsListResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::OperationsListResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
}
pub mod edge_nodes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Edgenodes are the global Point of Presence (POP) locations used to deliver CDN content to end users."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EdgenodeResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::EdgenodeResult = serde_json::from_slice(&bytes)?;
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
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::EdgenodeResult, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-12-31");
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
        }
    }
}
