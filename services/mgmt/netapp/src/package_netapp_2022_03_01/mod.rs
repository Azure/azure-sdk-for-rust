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
    pub fn account_backups_client(&self) -> account_backups::Client {
        account_backups::Client(self.clone())
    }
    pub fn accounts_client(&self) -> accounts::Client {
        accounts::Client(self.clone())
    }
    pub fn backup_policies_client(&self) -> backup_policies::Client {
        backup_policies::Client(self.clone())
    }
    pub fn backups_client(&self) -> backups::Client {
        backups::Client(self.clone())
    }
    pub fn net_app_resource_client(&self) -> net_app_resource::Client {
        net_app_resource::Client(self.clone())
    }
    pub fn net_app_resource_quota_limits_client(&self) -> net_app_resource_quota_limits::Client {
        net_app_resource_quota_limits::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn pools_client(&self) -> pools::Client {
        pools::Client(self.clone())
    }
    pub fn snapshot_policies_client(&self) -> snapshot_policies::Client {
        snapshot_policies::Client(self.clone())
    }
    pub fn snapshots_client(&self) -> snapshots::Client {
        snapshots::Client(self.clone())
    }
    pub fn subvolumes_client(&self) -> subvolumes::Client {
        subvolumes::Client(self.clone())
    }
    pub fn vaults_client(&self) -> vaults::Client {
        vaults::Client(self.clone())
    }
    pub fn volume_groups_client(&self) -> volume_groups::Client {
        volume_groups::Client(self.clone())
    }
    pub fn volume_quota_rules_client(&self) -> volume_quota_rules::Client {
        volume_quota_rules::Client(self.clone())
    }
    pub fn volumes_client(&self) -> volumes::Client {
        volumes::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describes the Resource Provider"]
        #[doc = "Lists all of the available Microsoft.NetApp Rest API operations"]
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
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.NetApp/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod net_app_resource {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check resource name availability"]
        #[doc = "Check if a resource name is available."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Name availability request."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `location`: The location"]
        pub fn check_name_availability(
            &self,
            body: impl Into<models::ResourceNameAvailabilityRequest>,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
        ) -> check_name_availability::Builder {
            check_name_availability::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
        #[doc = "Check file path availability"]
        #[doc = "Check if a file path is available."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: File path availability request."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `location`: The location"]
        pub fn check_file_path_availability(
            &self,
            body: impl Into<models::FilePathAvailabilityRequest>,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
        ) -> check_file_path_availability::Builder {
            check_file_path_availability::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
        #[doc = "Check quota availability"]
        #[doc = "Check if a quota is available."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Quota availability request."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `location`: The location"]
        pub fn check_quota_availability(
            &self,
            body: impl Into<models::QuotaAvailabilityRequest>,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
        ) -> check_quota_availability::Builder {
            check_quota_availability::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
    }
    pub mod check_name_availability {
        use super::models;
        type Response = models::CheckAvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::ResourceNameAvailabilityRequest,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/locations/{}/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckAvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_file_path_availability {
        use super::models;
        type Response = models::CheckAvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::FilePathAvailabilityRequest,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/locations/{}/checkFilePathAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckAvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_quota_availability {
        use super::models;
        type Response = models::CheckAvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::QuotaAvailabilityRequest,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/locations/{}/checkQuotaAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckAvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod net_app_resource_quota_limits {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get quota limits"]
        #[doc = "Get the default and current limits for quotas"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `location`: The location"]
        pub fn list(&self, subscription_id: impl Into<String>, location: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
        #[doc = "Get quota limits"]
        #[doc = "Get the default and current subscription quota limit"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `location`: The location"]
        #[doc = "* `quota_limit_name`: The name of the Quota Limit"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
            quota_limit_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                quota_limit_name: quota_limit_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SubscriptionQuotaItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/locations/{}/quotaLimits",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionQuotaItemList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SubscriptionQuotaItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) quota_limit_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/locations/{}/quotaLimits/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location,
                            &this.quota_limit_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionQuotaItem = serde_json::from_slice(&rsp_body)?;
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
pub mod accounts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describe all NetApp Accounts in a subscription"]
        #[doc = "List and describe all NetApp accounts in the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Describe all NetApp Accounts in a resource group"]
        #[doc = "List and describe all NetApp accounts in the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        pub fn list(&self, subscription_id: impl Into<String>, resource_group_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Describe a NetApp Account"]
        #[doc = "Get the NetApp account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Create or update a NetApp account"]
        #[doc = "Create or update the specified NetApp account within the resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: NetApp Account object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn create_or_update(
            &self,
            body: impl Into<models::NetAppAccount>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Update a NetApp account"]
        #[doc = "Patch the specified NetApp account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: NetApp Account object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn update(
            &self,
            body: impl Into<models::NetAppAccountPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Delete a NetApp account"]
        #[doc = "Delete the specified NetApp account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::NetAppAccountList;
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
                            "{}/subscriptions/{}/providers/Microsoft.NetApp/netAppAccounts",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccountList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::NetAppAccountList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccountList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::NetAppAccount;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccount = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::NetAppAccount),
            Created201(models::NetAppAccount),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::NetAppAccount,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccount = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccount = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::NetAppAccount),
            Accepted202(models::NetAppAccount),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::NetAppAccountPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccount = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetAppAccount = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod pools {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describe all Capacity Pools"]
        #[doc = "List all capacity pools in the NetApp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Describe a Capacity Pool"]
        #[doc = "Get details of the specified capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
            }
        }
        #[doc = "Create or Update the specified capacity pool within the resource group"]
        #[doc = "Create or Update a capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Capacity pool object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        pub fn create_or_update(
            &self,
            body: impl Into<models::CapacityPool>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
            }
        }
        #[doc = "Update a capacity pool"]
        #[doc = "Patch the specified capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Capacity pool object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        pub fn update(
            &self,
            body: impl Into<models::CapacityPoolPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
            }
        }
        #[doc = "Delete a capacity pool"]
        #[doc = "Delete the specified capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CapacityPoolList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CapacityPoolList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::CapacityPool;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CapacityPool = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::CapacityPool),
            Created201(models::CapacityPool),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::CapacityPool,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CapacityPool = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CapacityPool = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::CapacityPool),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::CapacityPoolPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CapacityPool = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod volumes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describe all volumes"]
        #[doc = "List all volumes within the capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
            }
        }
        #[doc = "Describe a volume"]
        #[doc = "Get the details of the specified volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Create or Update a volume"]
        #[doc = "Create or update the specified volume within the capacity pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Volume object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn create_or_update(
            &self,
            body: impl Into<models::Volume>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Update a volume"]
        #[doc = "Patch the specified volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Volume object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn update(
            &self,
            body: impl Into<models::VolumePatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Delete a volume"]
        #[doc = "Delete the specified volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                force_delete: None,
            }
        }
        #[doc = "Revert a volume to one of its snapshots"]
        #[doc = "Revert a volume to the snapshot specified in the body"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `body`: Object for snapshot to revert supplied in the body of the operation."]
        pub fn revert(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            body: impl Into<models::VolumeRevert>,
        ) -> revert::Builder {
            revert::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Reset cifs password"]
        #[doc = "Reset cifs password from volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn reset_cifs_password(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> reset_cifs_password::Builder {
            reset_cifs_password::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Break volume replication"]
        #[doc = "Break the replication connection on the destination volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn break_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> break_replication::Builder {
            break_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                body: None,
            }
        }
        #[doc = "Re-establish volume replication"]
        #[doc = "Re-establish a previously deleted replication between 2 volumes that have a common ad-hoc or policy-based snapshots"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `body`: body for the id of the source volume."]
        pub fn reestablish_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            body: impl Into<models::ReestablishReplicationRequest>,
        ) -> reestablish_replication::Builder {
            reestablish_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Get volume replication status"]
        #[doc = "Get the status of the replication"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn replication_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> replication_status::Builder {
            replication_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "List replications for volume"]
        #[doc = "List all replications for a specified volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn list_replications(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> list_replications::Builder {
            list_replications::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Resync volume replication"]
        #[doc = "Resync the connection on the destination volume. If the operation is ran on the source volume it will reverse-resync the connection and sync from destination to source."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn resync_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> resync_replication::Builder {
            resync_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Delete volume replication"]
        #[doc = "Delete the replication connection on the destination volume, and send release to the source replication"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn delete_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> delete_replication::Builder {
            delete_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Authorize source volume replication"]
        #[doc = "Authorize the replication connection on the source volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `body`: Authorize request object supplied in the body of the operation."]
        pub fn authorize_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            body: impl Into<models::AuthorizeRequest>,
        ) -> authorize_replication::Builder {
            authorize_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                body: body.into(),
            }
        }
        #[doc = "ReInitialize volume replication"]
        #[doc = "Re-Initializes the replication connection on the destination volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn re_initialize_replication(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> re_initialize_replication::Builder {
            re_initialize_replication::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Change pool for volume"]
        #[doc = "Moves volume to another pool"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `body`: Move volume to the pool supplied in the body of the operation."]
        pub fn pool_change(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            body: impl Into<models::PoolChangeRequest>,
        ) -> pool_change::Builder {
            pool_change::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Relocate volume"]
        #[doc = "Relocates volume to a new stamp"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn relocate(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> relocate::Builder {
            relocate::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Finalize volume relocation"]
        #[doc = "Finalizes the relocation of the volume and cleans up the old volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn finalize_relocation(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> finalize_relocation::Builder {
            finalize_relocation::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Revert volume relocation"]
        #[doc = "Reverts the volume relocation process, cleans up the new volume and starts using the former-existing volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn revert_relocation(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> revert_relocation::Builder {
            revert_relocation::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::VolumeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.pool_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Volume;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Volume = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Volume),
            Created201(models::Volume),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::Volume,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Volume = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Volume = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
            Ok200(models::Volume),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::VolumePatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Volume = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) force_delete: Option<bool>,
        }
        impl Builder {
            #[doc = "An option to force delete the volume. Will cleanup resources connected to the particular volume"]
            pub fn force_delete(mut self, force_delete: bool) -> Self {
                self.force_delete = Some(force_delete);
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        if let Some(force_delete) = &this.force_delete {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("forceDelete", &force_delete.to_string());
                        }
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
    pub mod revert {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) body: models::VolumeRevert,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/revert" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
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
    pub mod reset_cifs_password {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/resetCifsPassword" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod break_replication {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) body: Option<models::BreakReplicationRequest>,
        }
        impl Builder {
            #[doc = "Optional body to force break the replication."]
            pub fn body(mut self, body: impl Into<models::BreakReplicationRequest>) -> Self {
                self.body = Some(body.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/breakReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod reestablish_replication {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) body: models::ReestablishReplicationRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/reestablishReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
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
    pub mod replication_status {
        use super::models;
        type Response = models::ReplicationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/replicationStatus" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationStatus = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_replications {
        use super::models;
        type Response = models::ListReplications;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/listReplications" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ListReplications = serde_json::from_slice(&rsp_body)?;
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
    pub mod resync_replication {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/resyncReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod delete_replication {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/deleteReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod authorize_replication {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) body: models::AuthorizeRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/authorizeReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
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
    pub mod re_initialize_replication {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/reinitializeReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod pool_change {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) body: models::PoolChangeRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/poolChange" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
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
    pub mod relocate {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/relocate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod finalize_relocation {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/finalizeRelocation" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod revert_relocation {
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/revertRelocation" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod snapshots {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describe all snapshots"]
        #[doc = "List all snapshots associated with the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Describe a snapshot"]
        #[doc = "Get details of the specified snapshot"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `snapshot_name`: The name of the snapshot"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            snapshot_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                snapshot_name: snapshot_name.into(),
            }
        }
        #[doc = "Create a snapshot"]
        #[doc = "Create the specified snapshot within the given volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Snapshot object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `snapshot_name`: The name of the snapshot"]
        pub fn create(
            &self,
            body: impl Into<models::Snapshot>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            snapshot_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                snapshot_name: snapshot_name.into(),
            }
        }
        #[doc = "Update a snapshot"]
        #[doc = "Patch a snapshot"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Snapshot object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `snapshot_name`: The name of the snapshot"]
        pub fn update(
            &self,
            body: impl Into<models::SnapshotPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            snapshot_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                snapshot_name: snapshot_name.into(),
            }
        }
        #[doc = "Delete a snapshot"]
        #[doc = "Delete snapshot"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `snapshot_name`: The name of the snapshot"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            snapshot_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                snapshot_name: snapshot_name.into(),
            }
        }
        #[doc = "Create a new Snapshot Restore Files request"]
        #[doc = "Restore the specified files from the specified snapshot to the active filesystem"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Restore payload supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `snapshot_name`: The name of the snapshot"]
        pub fn restore_files(
            &self,
            body: impl Into<models::SnapshotRestoreFiles>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            snapshot_name: impl Into<String>,
        ) -> restore_files::Builder {
            restore_files::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                snapshot_name: snapshot_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SnapshotsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotsList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Snapshot;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) snapshot_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . snapshot_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Snapshot = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::Snapshot),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::Snapshot,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) snapshot_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . snapshot_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Snapshot = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
            Ok200(models::Snapshot),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SnapshotPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) snapshot_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . snapshot_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Snapshot = serde_json::from_slice(&rsp_body)?;
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
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) snapshot_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . snapshot_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod restore_files {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SnapshotRestoreFiles,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) snapshot_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/snapshots/{}/restoreFiles" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . snapshot_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
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
pub mod snapshot_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List snapshot policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Get a snapshot Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `snapshot_policy_name`: The name of the snapshot policy"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            snapshot_policy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                snapshot_policy_name: snapshot_policy_name.into(),
            }
        }
        #[doc = "Create a snapshot policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Snapshot policy object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `snapshot_policy_name`: The name of the snapshot policy"]
        pub fn create(
            &self,
            body: impl Into<models::SnapshotPolicy>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            snapshot_policy_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                snapshot_policy_name: snapshot_policy_name.into(),
            }
        }
        #[doc = "Patch a snapshot policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Snapshot policy object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `snapshot_policy_name`: The name of the snapshot policy"]
        pub fn update(
            &self,
            body: impl Into<models::SnapshotPolicyPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            snapshot_policy_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                snapshot_policy_name: snapshot_policy_name.into(),
            }
        }
        #[doc = "Delete snapshot policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `snapshot_policy_name`: The name of the snapshot policy"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            snapshot_policy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                snapshot_policy_name: snapshot_policy_name.into(),
            }
        }
        #[doc = "Get volumes for snapshot policy"]
        #[doc = "Get volumes associated with snapshot policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `snapshot_policy_name`: The name of the snapshot policy"]
        pub fn list_volumes(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            snapshot_policy_name: impl Into<String>,
        ) -> list_volumes::Builder {
            list_volumes::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                snapshot_policy_name: snapshot_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SnapshotPoliciesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPoliciesList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SnapshotPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) snapshot_policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.snapshot_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SnapshotPolicy),
            Created201(models::SnapshotPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SnapshotPolicy,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) snapshot_policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.snapshot_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SnapshotPolicy),
            Accepted202(models::SnapshotPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SnapshotPolicyPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) snapshot_policy_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.snapshot_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicy = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) snapshot_policy_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.snapshot_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod list_volumes {
        use super::models;
        type Response = models::SnapshotPolicyVolumeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) snapshot_policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/snapshotPolicies/{}/volumes" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . snapshot_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SnapshotPolicyVolumeList = serde_json::from_slice(&rsp_body)?;
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
pub mod backups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get volume's backup status"]
        #[doc = "Get the status of the backup for a volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn get_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> get_status::Builder {
            get_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Get volume's restore status"]
        #[doc = "Get the status of the restore for a volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn get_volume_restore_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> get_volume_restore_status::Builder {
            get_volume_restore_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "List Backups"]
        #[doc = "List all backups for a volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Get a backup"]
        #[doc = "Gets the specified backup of the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `backup_name`: The name of the backup"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            backup_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                backup_name: backup_name.into(),
            }
        }
        #[doc = "Create a backup"]
        #[doc = "Create a backup for the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `backup_name`: The name of the backup"]
        #[doc = "* `body`: Backup object supplied in the body of the operation."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            backup_name: impl Into<String>,
            body: impl Into<models::Backup>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                backup_name: backup_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Patch a backup"]
        #[doc = "Patch a backup for the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `backup_name`: The name of the backup"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            backup_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                backup_name: backup_name.into(),
                body: None,
            }
        }
        #[doc = "Delete backup"]
        #[doc = "Delete a backup of the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `backup_name`: The name of the backup"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            backup_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                backup_name: backup_name.into(),
            }
        }
    }
    pub mod get_status {
        use super::models;
        type Response = models::BackupStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backupStatus" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupStatus = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_volume_restore_status {
        use super::models;
        type Response = models::RestoreStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/restoreStatus" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RestoreStatus = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupsList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Backup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) backup_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . backup_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Backup),
            Created201(models::Backup),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) backup_name: String,
            pub(crate) body: models::Backup,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . backup_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
            Ok200(models::Backup),
            Accepted202(models::Backup),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) backup_name: String,
            pub(crate) body: Option<models::BackupPatch>,
        }
        impl Builder {
            #[doc = "Backup object supplied in the body of the operation."]
            pub fn body(mut self, body: impl Into<models::BackupPatch>) -> Self {
                self.body = Some(body.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . backup_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) backup_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/backups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . backup_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod account_backups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Backups for a Netapp Account"]
        #[doc = "List all Backups for a Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Get Backup for a Netapp Account"]
        #[doc = "Gets the specified backup for a Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_name`: The name of the backup"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_name: backup_name.into(),
            }
        }
        #[doc = "Delete Backup for a Netapp Account"]
        #[doc = "Delete the specified Backup for a Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_name`: The name of the backup"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_name: backup_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BackupsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/accountBackups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupsList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Backup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/accountBackups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Backup = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/accountBackups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod backup_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List backup policies"]
        #[doc = "List backup policies for Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Get a backup Policy"]
        #[doc = "Get a particular backup Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_policy_name`: Backup policy Name which uniquely identify backup policy."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_policy_name: backup_policy_name.into(),
            }
        }
        #[doc = "Create a backup policy"]
        #[doc = "Create a backup policy for Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_policy_name`: Backup policy Name which uniquely identify backup policy."]
        #[doc = "* `body`: Backup policy object supplied in the body of the operation."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            body: impl Into<models::BackupPolicy>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_policy_name: backup_policy_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Patch a backup policy"]
        #[doc = "Patch a backup policy for Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_policy_name`: Backup policy Name which uniquely identify backup policy."]
        #[doc = "* `body`: Backup policy object supplied in the body of the operation."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            body: impl Into<models::BackupPolicyPatch>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_policy_name: backup_policy_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Delete a backup policy"]
        #[doc = "Delete backup policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `backup_policy_name`: Backup policy Name which uniquely identify backup policy."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                backup_policy_name: backup_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BackupPoliciesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/backupPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPoliciesList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::BackupPolicy),
            Created201(models::BackupPolicy),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) body: models::BackupPolicy,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
            Ok200(models::BackupPolicy),
            Accepted202(models::BackupPolicy),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) body: models::BackupPolicyPatch,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) backup_policy_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.backup_policy_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod volume_quota_rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all quota rules for a volume"]
        #[doc = "List all quota rules associated with the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn list_by_volume(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> list_by_volume::Builder {
            list_by_volume::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Describe a quota rule"]
        #[doc = "Get details of the specified quota rule"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `volume_quota_rule_name`: The name of volume quota rule"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            volume_quota_rule_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                volume_quota_rule_name: volume_quota_rule_name.into(),
            }
        }
        #[doc = "Create a quota rule"]
        #[doc = "Create the specified quota rule within the given volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Quota rule object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `volume_quota_rule_name`: The name of volume quota rule"]
        pub fn create(
            &self,
            body: impl Into<models::VolumeQuotaRule>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            volume_quota_rule_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                volume_quota_rule_name: volume_quota_rule_name.into(),
            }
        }
        #[doc = "Update a quota rule"]
        #[doc = "Patch a quota rule"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Quota rule object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `volume_quota_rule_name`: The name of volume quota rule"]
        pub fn update(
            &self,
            body: impl Into<models::VolumeQuotaRulePatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            volume_quota_rule_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                volume_quota_rule_name: volume_quota_rule_name.into(),
            }
        }
        #[doc = "Delete a quota rule"]
        #[doc = "Delete quota rule"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `volume_quota_rule_name`: The name of volume quota rule"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            volume_quota_rule_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                volume_quota_rule_name: volume_quota_rule_name.into(),
            }
        }
    }
    pub mod list_by_volume {
        use super::models;
        type Response = models::VolumeQuotaRulesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/volumeQuotaRules" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeQuotaRulesList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VolumeQuotaRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) volume_quota_rule_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/volumeQuotaRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . volume_quota_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeQuotaRule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::VolumeQuotaRule),
            Created201(models::VolumeQuotaRule),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::VolumeQuotaRule,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) volume_quota_rule_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/volumeQuotaRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . volume_quota_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeQuotaRule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeQuotaRule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::VolumeQuotaRule),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::VolumeQuotaRulePatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) volume_quota_rule_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/volumeQuotaRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . volume_quota_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeQuotaRule = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) volume_quota_rule_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/volumeQuotaRules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . volume_quota_rule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod vaults {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List vaults"]
        #[doc = "List vaults for a Netapp Account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::VaultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/vaults",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultList = serde_json::from_slice(&rsp_body)?;
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
pub mod volume_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Describe all volume groups"]
        #[doc = "List all volume groups for given account"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        pub fn list_by_net_app_account(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
        ) -> list_by_net_app_account::Builder {
            list_by_net_app_account::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
            }
        }
        #[doc = "Describe a Volume Group"]
        #[doc = "Get details of the specified volume group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `volume_group_name`: The name of the volumeGroup"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            volume_group_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                volume_group_name: volume_group_name.into(),
            }
        }
        #[doc = "Create the specified volume group and volumes. Creating volume group will create all the volumes specified in request body implicitly. Once volumes are created using volume group, those will be treated as regular volumes thereafter."]
        #[doc = "Create a volume group along with specified volumes"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Volume Group object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `volume_group_name`: The name of the volumeGroup"]
        pub fn create(
            &self,
            body: impl Into<models::VolumeGroupDetails>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            volume_group_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                volume_group_name: volume_group_name.into(),
            }
        }
        #[doc = "Delete a volume group"]
        #[doc = "Delete the specified volume group only if there are no volumes under volume group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `volume_group_name`: The name of the volumeGroup"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            volume_group_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                volume_group_name: volume_group_name.into(),
            }
        }
    }
    pub mod list_by_net_app_account {
        use super::models;
        type Response = models::VolumeGroupList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/volumeGroups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeGroupList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VolumeGroupDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) volume_group_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/volumeGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.volume_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeGroupDetails = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VolumeGroupDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::VolumeGroupDetails,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) volume_group_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/volumeGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.volume_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeGroupDetails = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) volume_group_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/volumeGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.account_name,
                            &this.volume_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
pub mod subvolumes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List of all the subvolumes"]
        #[doc = "Returns a list of the subvolumes in the volume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        pub fn list_by_volume(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
        ) -> list_by_volume::Builder {
            list_by_volume::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
            }
        }
        #[doc = "Get the path associated with the subvolumeName"]
        #[doc = "Returns the path associated with the subvolumeName provided"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `subvolume_name`: The name of the subvolume."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            subvolume_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                subvolume_name: subvolume_name.into(),
            }
        }
        #[doc = "Create or clone a new subvolume"]
        #[doc = "Creates a subvolume in the path or clones the subvolume mentioned in the parentPath"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Subvolume object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `subvolume_name`: The name of the subvolume."]
        pub fn create(
            &self,
            body: impl Into<models::SubvolumeInfo>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            subvolume_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                subvolume_name: subvolume_name.into(),
            }
        }
        #[doc = "Update a subvolume"]
        #[doc = "Patch a subvolume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `body`: Subvolume object supplied in the body of the operation."]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `subvolume_name`: The name of the subvolume."]
        pub fn update(
            &self,
            body: impl Into<models::SubvolumePatchRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            subvolume_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                body: body.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                subvolume_name: subvolume_name.into(),
            }
        }
        #[doc = "Delete a subvolume"]
        #[doc = "Delete subvolume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `subvolume_name`: The name of the subvolume."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            subvolume_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                subvolume_name: subvolume_name.into(),
            }
        }
        #[doc = "Describe a subvolume"]
        #[doc = "Get details of the specified subvolume"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription credentials which uniquely identify Microsoft Azure subscription. The subscription ID forms part of the URI for every service call."]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `account_name`: The name of the NetApp account"]
        #[doc = "* `pool_name`: The name of the capacity pool"]
        #[doc = "* `volume_name`: The name of the volume"]
        #[doc = "* `subvolume_name`: The name of the subvolume."]
        pub fn get_metadata(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            account_name: impl Into<String>,
            pool_name: impl Into<String>,
            volume_name: impl Into<String>,
            subvolume_name: impl Into<String>,
        ) -> get_metadata::Builder {
            get_metadata::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                account_name: account_name.into(),
                pool_name: pool_name.into(),
                volume_name: volume_name.into(),
                subvolume_name: subvolume_name.into(),
            }
        }
    }
    pub mod list_by_volume {
        use super::models;
        type Response = models::SubvolumesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumesList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SubvolumeInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) subvolume_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . subvolume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumeInfo = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SubvolumeInfo),
            Created201(models::SubvolumeInfo),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SubvolumeInfo,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) subvolume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . subvolume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumeInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumeInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
            Ok200(models::SubvolumeInfo),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) body: models::SubvolumePatchRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) subvolume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . subvolume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumeInfo = serde_json::from_slice(&rsp_body)?;
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
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) subvolume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . subvolume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
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
    pub mod get_metadata {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::SubvolumeModel),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) account_name: String,
            pub(crate) pool_name: String,
            pub(crate) volume_name: String,
            pub(crate) subvolume_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.NetApp/netAppAccounts/{}/capacityPools/{}/volumes/{}/subvolumes/{}/getMetadata" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . account_name , & this . pool_name , & this . volume_name , & this . subvolume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubvolumeModel = serde_json::from_slice(&rsp_body)?;
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
}
