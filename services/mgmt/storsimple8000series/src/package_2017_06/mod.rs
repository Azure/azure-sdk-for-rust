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
    pub fn access_control_records_client(&self) -> access_control_records::Client {
        access_control_records::Client(self.clone())
    }
    pub fn alerts_client(&self) -> alerts::Client {
        alerts::Client(self.clone())
    }
    pub fn backup_policies_client(&self) -> backup_policies::Client {
        backup_policies::Client(self.clone())
    }
    pub fn backup_schedules_client(&self) -> backup_schedules::Client {
        backup_schedules::Client(self.clone())
    }
    pub fn backups_client(&self) -> backups::Client {
        backups::Client(self.clone())
    }
    pub fn bandwidth_settings_client(&self) -> bandwidth_settings::Client {
        bandwidth_settings::Client(self.clone())
    }
    pub fn cloud_appliances_client(&self) -> cloud_appliances::Client {
        cloud_appliances::Client(self.clone())
    }
    pub fn device_settings_client(&self) -> device_settings::Client {
        device_settings::Client(self.clone())
    }
    pub fn devices_client(&self) -> devices::Client {
        devices::Client(self.clone())
    }
    pub fn hardware_component_groups_client(&self) -> hardware_component_groups::Client {
        hardware_component_groups::Client(self.clone())
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn managers_client(&self) -> managers::Client {
        managers::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn storage_account_credentials_client(&self) -> storage_account_credentials::Client {
        storage_account_credentials::Client(self.clone())
    }
    pub fn volume_containers_client(&self) -> volume_containers::Client {
        volume_containers::Client(self.clone())
    }
    pub fn volumes_client(&self) -> volumes::Client {
        volumes::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available REST API operations of the Microsoft.StorSimple provider"]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AvailableProviderOperationList;
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
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.StorSimple/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AvailableProviderOperationList = serde_json::from_slice(&rsp_body)?;
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
pub mod managers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the managers in a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieves all the managers in a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
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
        #[doc = "Returns the properties of the specified manager name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The manager."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            parameters: impl Into<models::Manager>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Updates the StorSimple Manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The manager update parameters."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn update(
            &self,
            parameters: impl Into<models::ManagerPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the public encryption key of the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_device_public_encryption_key(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_device_public_encryption_key::Builder {
            get_device_public_encryption_key::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the encryption settings of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_encryption_settings(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_encryption_settings::Builder {
            get_encryption_settings::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the extended information of the specified manager name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_extended_info(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_extended_info::Builder {
            get_extended_info::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates the extended info of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The manager extended information."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_extended_info(
            &self,
            parameters: impl Into<models::ManagerExtendedInfo>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_extended_info::Builder {
            create_extended_info::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Updates the extended info of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The manager extended information."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `if_match`: Pass the ETag of ExtendedInfo fetched from GET call"]
        pub fn update_extended_info(
            &self,
            parameters: impl Into<models::ManagerExtendedInfo>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            if_match: impl Into<String>,
        ) -> update_extended_info::Builder {
            update_extended_info::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                if_match: if_match.into(),
            }
        }
        #[doc = "Deletes the extended info of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete_extended_info(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete_extended_info::Builder {
            delete_extended_info::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Lists the features and their support status"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_feature_support_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_feature_support_status::Builder {
            list_feature_support_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Returns the activation key of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_activation_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_activation_key::Builder {
            get_activation_key::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the symmetric encrypted public encryption key of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_public_encryption_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_public_encryption_key::Builder {
            get_public_encryption_key::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the metrics for the specified manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `filter`: OData Filter options"]
        pub fn list_metrics(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            filter: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: filter.into(),
            }
        }
        #[doc = "Gets the metric definitions for the specified manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Re-generates and returns the activation key of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn regenerate_activation_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> regenerate_activation_key::Builder {
            regenerate_activation_key::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ManagerList;
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
                            "{}/subscriptions/{}/providers/Microsoft.StorSimple/managers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagerList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ManagerList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagerList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Manager;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Manager = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Manager),
            Created201(models::Manager),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::Manager,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Manager = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Manager = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Manager;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::ManagerPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Manager = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod get_device_public_encryption_key {
        use super::models;
        type Response = models::PublicKey;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/publicEncryptionKey" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod get_encryption_settings {
        use super::models;
        type Response = models::EncryptionSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/encryptionSettings/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EncryptionSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_extended_info {
        use super::models;
        type Response = models::ManagerExtendedInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/extendedInformation/vaultExtendedInfo" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagerExtendedInfo = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_extended_info {
        use super::models;
        type Response = models::ManagerExtendedInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::ManagerExtendedInfo,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/extendedInformation/vaultExtendedInfo" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagerExtendedInfo = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_extended_info {
        use super::models;
        type Response = models::ManagerExtendedInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::ManagerExtendedInfo,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) if_match: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/extendedInformation/vaultExtendedInfo" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.insert_header("if-match", &this.if_match);
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagerExtendedInfo = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_extended_info {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/extendedInformation/vaultExtendedInfo" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_feature_support_status {
        use super::models;
        type Response = models::FeatureList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData Filter options"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/features",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeatureList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_activation_key {
        use super::models;
        type Response = models::Key;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/listActivationKey",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Key = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_public_encryption_key {
        use super::models;
        type Response = models::SymmetricEncryptedSecret;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/listPublicEncryptionKey",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SymmetricEncryptedSecret = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) filter: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/metrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let filter = &this.filter;
                        req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_definition {
        use super::models;
        type Response = models::MetricDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/metricsDefinitions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricDefinitionList = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_activation_key {
        use super::models;
        type Response = models::Key;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/regenerateActivationKey",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Key = serde_json::from_slice(&rsp_body)?;
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
pub mod access_control_records {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the access control records in a manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the properties of the specified access control record name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `access_control_record_name`: Name of access control record to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            access_control_record_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                access_control_record_name: access_control_record_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or Updates an access control record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `access_control_record_name`: The name of the access control record."]
        #[doc = "* `parameters`: The access control record to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            access_control_record_name: impl Into<String>,
            parameters: impl Into<models::AccessControlRecord>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                access_control_record_name: access_control_record_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the access control record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `access_control_record_name`: The name of the access control record to delete."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            access_control_record_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                access_control_record_name: access_control_record_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::AccessControlRecordList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/accessControlRecords",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessControlRecordList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AccessControlRecord;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) access_control_record_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/accessControlRecords/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.access_control_record_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessControlRecord = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AccessControlRecord),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) access_control_record_name: String,
            pub(crate) parameters: models::AccessControlRecord,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/accessControlRecords/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.access_control_record_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessControlRecord = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) access_control_record_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/accessControlRecords/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.access_control_record_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
pub mod alerts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the alerts in a manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Clear the alerts."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The clear alert request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn clear(
            &self,
            parameters: impl Into<models::ClearAlertRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> clear::Builder {
            clear::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Sends a test alert email."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: The send test alert email request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn send_test_email(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::SendTestAlertEmailRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> send_test_email::Builder {
            send_test_email::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::AlertList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/alerts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                let rsp_value: models::AlertList = serde_json::from_slice(&rsp_body)?;
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
    pub mod clear {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::ClearAlertRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/clearAlerts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod send_test_email {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::SendTestAlertEmailRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/sendTestAlertEmail" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod bandwidth_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the bandwidth setting in a manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the properties of the specified bandwidth setting name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bandwidth_setting_name`: The name of bandwidth setting to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            bandwidth_setting_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                bandwidth_setting_name: bandwidth_setting_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the bandwidth setting"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bandwidth_setting_name`: The bandwidth setting name."]
        #[doc = "* `parameters`: The bandwidth setting to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            bandwidth_setting_name: impl Into<String>,
            parameters: impl Into<models::BandwidthSetting>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                bandwidth_setting_name: bandwidth_setting_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the bandwidth setting"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bandwidth_setting_name`: The name of the bandwidth setting."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            bandwidth_setting_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                bandwidth_setting_name: bandwidth_setting_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::BandwidthSettingList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/bandwidthSettings",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BandwidthSettingList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BandwidthSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bandwidth_setting_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/bandwidthSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.bandwidth_setting_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BandwidthSetting = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::BandwidthSetting),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bandwidth_setting_name: String,
            pub(crate) parameters: models::BandwidthSetting,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/bandwidthSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.bandwidth_setting_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BandwidthSetting = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) bandwidth_setting_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/bandwidthSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.bandwidth_setting_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
pub mod cloud_appliances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists supported cloud appliance models and supported configurations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_supported_configurations(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_supported_configurations::Builder {
            list_supported_configurations::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Provisions cloud appliance."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The cloud appliance"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn provision(
            &self,
            parameters: impl Into<models::CloudAppliance>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> provision::Builder {
            provision::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_supported_configurations {
        use super::models;
        type Response = models::CloudApplianceConfigurationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/cloudApplianceConfigurations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudApplianceConfigurationList = serde_json::from_slice(&rsp_body)?;
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
    pub mod provision {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::CloudAppliance,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/provisionCloudAppliance",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
pub mod devices {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Complete minimal setup before using the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `parameters`: The minimal properties to configure a device."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn configure(
            &self,
            parameters: impl Into<models::ConfigureDeviceRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> configure::Builder {
            configure::Builder {
                client: self.0.clone(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the list of devices for the specified manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                expand: None,
            }
        }
        #[doc = "Returns the properties of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                expand: None,
            }
        }
        #[doc = "Patches the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: Patch representation of the device."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn update(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::DevicePatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Authorizes the specified device for service data encryption key rollover."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn authorize_for_service_encryption_key_rollover(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> authorize_for_service_encryption_key_rollover::Builder {
            authorize_for_service_encryption_key_rollover::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deactivates the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn deactivate(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> deactivate::Builder {
            deactivate::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Downloads and installs the updates on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn install_updates(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> install_updates::Builder {
            install_updates::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns all failover sets for a given device and their eligibility for participating in a failover. A failover set refers to a set of volume containers that need to be failed-over as a single unit to maintain data integrity."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_failover_sets(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_failover_sets::Builder {
            list_failover_sets::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the metrics for the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `filter`: OData Filter options"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            filter: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: filter.into(),
            }
        }
        #[doc = "Gets the metric definitions for the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Scans for updates on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn scan_for_updates(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> scan_for_updates::Builder {
            scan_for_updates::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the update summary of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_update_summary(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_update_summary::Builder {
            get_update_summary::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Failovers a set of volume containers from a specified source device to a target device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `source_device_name`: The source device name on which failover is performed."]
        #[doc = "* `parameters`: FailoverRequest containing the source device and the list of volume containers to be failed over."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn failover(
            &self,
            source_device_name: impl Into<String>,
            parameters: impl Into<models::FailoverRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> failover::Builder {
            failover::Builder {
                client: self.0.clone(),
                source_device_name: source_device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Given a list of volume containers to be failed over from a source device, this method returns the eligibility result, as a failover target, for all devices under that resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `source_device_name`: The source device name on which failover is performed."]
        #[doc = "* `parameters`: ListFailoverTargetsRequest containing the list of volume containers to be failed over."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_failover_targets(
            &self,
            source_device_name: impl Into<String>,
            parameters: impl Into<models::ListFailoverTargetsRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_failover_targets::Builder {
            list_failover_targets::Builder {
                client: self.0.clone(),
                source_device_name: source_device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod configure {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) parameters: models::ConfigureDeviceRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/configureDevice",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod list_by_manager {
        use super::models;
        type Response = models::DeviceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Specify $expand=details to populate additional fields related to the device or $expand=rolloverdetails to populate additional fields related to the service data encryption key rollover on device"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                let rsp_value: models::DeviceList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Device;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Specify $expand=details to populate additional fields related to the device or $expand=rolloverdetails to populate additional fields related to the service data encryption key rollover on device"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                let rsp_value: models::Device = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Device;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::DevicePatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Device = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod authorize_for_service_encryption_key_rollover {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/authorizeForServiceEncryptionKeyRollover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod deactivate {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/deactivate",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod install_updates {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/installUpdates",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod list_failover_sets {
        use super::models;
        type Response = models::FailoverSetsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/listFailoverSets",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FailoverSetsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) filter: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/metrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let filter = &this.filter;
                        req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_definition {
        use super::models;
        type Response = models::MetricDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricDefinitionList = serde_json::from_slice(&rsp_body)?;
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
    pub mod scan_for_updates {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/scanForUpdates",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod get_update_summary {
        use super::models;
        type Response = models::Updates;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/updateSummary/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Updates = serde_json::from_slice(&rsp_body)?;
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
    pub mod failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) source_device_name: String,
            pub(crate) parameters: models::FailoverRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/failover",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.source_device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod list_failover_targets {
        use super::models;
        type Response = models::FailoverTargetsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) source_device_name: String,
            pub(crate) parameters: models::ListFailoverTargetsRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/listFailoverTargets" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . source_device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FailoverTargetsList = serde_json::from_slice(&rsp_body)?;
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
pub mod device_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the alert settings of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_alert_settings(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_alert_settings::Builder {
            get_alert_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the alert settings of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: The alert settings to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update_alert_settings(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::AlertSettings>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update_alert_settings::Builder {
            create_or_update_alert_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the network settings of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_network_settings(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_network_settings::Builder {
            get_network_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Updates the network settings on the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: The network settings to be updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn update_network_settings(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::NetworkSettingsPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> update_network_settings::Builder {
            update_network_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the Security properties of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_security_settings(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_security_settings::Builder {
            get_security_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Patch Security properties of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: The security settings properties to be patched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn update_security_settings(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::SecuritySettingsPatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> update_security_settings::Builder {
            update_security_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "sync Remote management Certificate between appliance and Service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn sync_remotemanagement_certificate(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> sync_remotemanagement_certificate::Builder {
            sync_remotemanagement_certificate::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the time settings of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_time_settings(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_time_settings::Builder {
            get_time_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the time settings of the specified device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `parameters`: The time settings to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update_time_settings(
            &self,
            device_name: impl Into<String>,
            parameters: impl Into<models::TimeSettings>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update_time_settings::Builder {
            create_or_update_time_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod get_alert_settings {
        use super::models;
        type Response = models::AlertSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/alertSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_alert_settings {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AlertSettings),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::AlertSettings,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/alertSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_network_settings {
        use super::models;
        type Response = models::NetworkSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/networkSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_network_settings {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::NetworkSettings),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::NetworkSettingsPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/networkSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_security_settings {
        use super::models;
        type Response = models::SecuritySettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/securitySettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecuritySettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_security_settings {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::SecuritySettings),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::SecuritySettingsPatch,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/securitySettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecuritySettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod sync_remotemanagement_certificate {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/securitySettings/default/syncRemoteManagementCertificate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod get_time_settings {
        use super::models;
        type Response = models::TimeSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/timeSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TimeSettings = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_time_settings {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::TimeSettings),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) parameters: models::TimeSettings,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/timeSettings/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TimeSettings = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the backup policies in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the properties of the specified backup policy name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The name of backup policy to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the backup policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The name of the backup policy to be created/updated."]
        #[doc = "* `parameters`: The backup policy."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            parameters: impl Into<models::BackupPolicy>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the backup policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The name of the backup policy."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Backup the backup policy now."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The backup policy name."]
        #[doc = "* `backup_type`: The backup Type. This can be cloudSnapshot or localSnapshot."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn backup_now(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            backup_type: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> backup_now::Builder {
            backup_now::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                backup_type: backup_type.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::BackupPolicyList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicyList = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::BackupPolicy),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) parameters: models::BackupPolicy,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupPolicy = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod backup_now {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) backup_type: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}/backup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let backup_type = &this.backup_type;
                        req.url_mut().query_pairs_mut().append_pair("backupType", backup_type);
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
}
pub mod backup_schedules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the backup schedules in a backup policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The backup policy name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_backup_policy(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_backup_policy::Builder {
            list_by_backup_policy::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the properties of the specified backup schedule name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The backup policy name."]
        #[doc = "* `backup_schedule_name`: The name of the backup schedule to be fetched"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            backup_schedule_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                backup_schedule_name: backup_schedule_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the backup schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The backup policy name."]
        #[doc = "* `backup_schedule_name`: The backup schedule name."]
        #[doc = "* `parameters`: The backup schedule."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            backup_schedule_name: impl Into<String>,
            parameters: impl Into<models::BackupSchedule>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                backup_schedule_name: backup_schedule_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the backup schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_policy_name`: The backup policy name."]
        #[doc = "* `backup_schedule_name`: The name the backup schedule."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            backup_policy_name: impl Into<String>,
            backup_schedule_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_policy_name: backup_policy_name.into(),
                backup_schedule_name: backup_schedule_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_backup_policy {
        use super::models;
        type Response = models::BackupScheduleList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}/schedules" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_policy_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupScheduleList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupSchedule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) backup_schedule_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}/schedules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_policy_name , & this . backup_schedule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupSchedule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::BackupSchedule),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) backup_schedule_name: String,
            pub(crate) parameters: models::BackupSchedule,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}/schedules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_policy_name , & this . backup_schedule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupSchedule = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) device_name: String,
            pub(crate) backup_policy_name: String,
            pub(crate) backup_schedule_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupPolicies/{}/schedules/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_policy_name , & this . backup_schedule_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
pub mod backups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the backups in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Deletes the backup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_name`: The backup name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            backup_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_name: backup_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Clones the backup element as a new volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_name`: The backup name."]
        #[doc = "* `backup_element_name`: The backup element name."]
        #[doc = "* `parameters`: The clone request object."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn clone(
            &self,
            device_name: impl Into<String>,
            backup_name: impl Into<String>,
            backup_element_name: impl Into<String>,
            parameters: impl Into<models::CloneRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> clone::Builder {
            clone::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_name: backup_name.into(),
                backup_element_name: backup_element_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Restores the backup on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `backup_name`: The backupSet name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn restore(
            &self,
            device_name: impl Into<String>,
            backup_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> restore::Builder {
            restore::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_name: backup_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::BackupList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                let rsp_value: models::BackupList = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) device_name: String,
            pub(crate) backup_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod clone {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_name: String,
            pub(crate) backup_element_name: String,
            pub(crate) parameters: models::CloneRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backups/{}/elements/{}/clone" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_name , & this . backup_element_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod restore {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) backup_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backups/{}/restore" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
pub mod hardware_component_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the hardware component groups at device-level."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Changes the power state of the controller."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `hardware_component_group_name`: The hardware component group name."]
        #[doc = "* `parameters`: The controller power state change request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn change_controller_power_state(
            &self,
            device_name: impl Into<String>,
            hardware_component_group_name: impl Into<String>,
            parameters: impl Into<models::ControllerPowerStateChangeRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> change_controller_power_state::Builder {
            change_controller_power_state::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                hardware_component_group_name: hardware_component_group_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::HardwareComponentGroupList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/hardwareComponentGroups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HardwareComponentGroupList = serde_json::from_slice(&rsp_body)?;
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
    pub mod change_controller_power_state {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) hardware_component_group_name: String,
            pub(crate) parameters: models::ControllerPowerStateChangeRequest,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/hardwareComponentGroups/{}/changeControllerPowerState" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . hardware_component_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the jobs for specified device. With optional OData query parameters, a filtered set of jobs is returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Gets the details of the specified job name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `job_name`: The job Name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            job_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                job_name: job_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Cancels a job on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `job_name`: The jobName."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn cancel(
            &self,
            device_name: impl Into<String>,
            job_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                job_name: job_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets all the jobs for the specified manager. With optional OData query parameters, a filtered set of jobs is returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::JobList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
            pub(crate) device_name: String,
            pub(crate) job_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/jobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.job_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
            pub(crate) device_name: String,
            pub(crate) job_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/jobs/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.job_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod list_by_manager {
        use super::models;
        type Response = models::JobList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/jobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
pub mod volume_containers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the volume containers in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the properties of the specified volume container name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The name of the volume container."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the volume container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The name of the volume container."]
        #[doc = "* `parameters`: The volume container to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            parameters: impl Into<models::VolumeContainer>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the volume container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The name of the volume container."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the metrics for the specified volume container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `filter`: OData Filter options"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            filter: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: filter.into(),
            }
        }
        #[doc = "Gets the metric definitions for the specified volume container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::VolumeContainerList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeContainerList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VolumeContainer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeContainer = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::VolumeContainer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) parameters: models::VolumeContainer,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VolumeContainer = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) filter: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let filter = &this.filter;
                        req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_definition {
        use super::models;
        type Response = models::MetricDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricDefinitionList = serde_json::from_slice(&rsp_body)?;
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
pub mod volumes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the volumes in a volume container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_volume_container(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_volume_container::Builder {
            list_by_volume_container::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the properties of the specified volume name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `volume_name`: The volume name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            volume_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                volume_name: volume_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `volume_name`: The volume name."]
        #[doc = "* `parameters`: Volume to be created or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            volume_name: impl Into<String>,
            parameters: impl Into<models::Volume>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                volume_name: volume_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `volume_name`: The volume name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            volume_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                volume_name: volume_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the metrics for the specified volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `volume_name`: The volume name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `filter`: OData Filter options"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            volume_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            filter: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                volume_name: volume_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: filter.into(),
            }
        }
        #[doc = "Gets the metric definitions for the specified volume."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `volume_container_name`: The volume container name."]
        #[doc = "* `volume_name`: The volume name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            volume_container_name: impl Into<String>,
            volume_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                volume_container_name: volume_container_name.into(),
                volume_name: volume_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Retrieves all the volumes in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_device(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_device::Builder {
            list_by_device::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_volume_container {
        use super::models;
        type Response = models::VolumeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
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
                })
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Volume;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) volume_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) volume_name: String,
            pub(crate) parameters: models::Volume,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) volume_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) volume_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
            pub(crate) filter: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let filter = &this.filter;
                        req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_definition {
        use super::models;
        type Response = models::MetricDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) volume_container_name: String,
            pub(crate) volume_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumeContainers/{}/volumes/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . volume_container_name , & this . volume_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricDefinitionList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_device {
        use super::models;
        type Response = models::VolumeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/volumes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
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
                })
            }
        }
    }
}
pub mod storage_account_credentials {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the storage account credentials in a manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_manager(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_manager::Builder {
            list_by_manager::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the properties of the specified storage account credential name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_account_credential_name`: The name of storage account credential to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            storage_account_credential_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                storage_account_credential_name: storage_account_credential_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the storage account credential."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_account_credential_name`: The storage account credential name."]
        #[doc = "* `parameters`: The storage account credential to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            storage_account_credential_name: impl Into<String>,
            parameters: impl Into<models::StorageAccountCredential>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                storage_account_credential_name: storage_account_credential_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the storage account credential."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_account_credential_name`: The name of the storage account credential."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            storage_account_credential_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                storage_account_credential_name: storage_account_credential_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::StorageAccountCredentialList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageAccountCredentials",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageAccountCredentialList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StorageAccountCredential;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_account_credential_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageAccountCredentials/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_account_credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageAccountCredential = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::StorageAccountCredential),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_account_credential_name: String,
            pub(crate) parameters: models::StorageAccountCredential,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageAccountCredentials/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_account_credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageAccountCredential = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) storage_account_credential_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageAccountCredentials/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_account_credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2017-06-01");
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
