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
    pub fn available_provider_operations_client(&self) -> available_provider_operations::Client {
        available_provider_operations::Client(self.clone())
    }
    pub fn backup_schedule_groups_client(&self) -> backup_schedule_groups::Client {
        backup_schedule_groups::Client(self.clone())
    }
    pub fn backups_client(&self) -> backups::Client {
        backups::Client(self.clone())
    }
    pub fn chap_settings_client(&self) -> chap_settings::Client {
        chap_settings::Client(self.clone())
    }
    pub fn devices_client(&self) -> devices::Client {
        devices::Client(self.clone())
    }
    pub fn file_servers_client(&self) -> file_servers::Client {
        file_servers::Client(self.clone())
    }
    pub fn file_shares_client(&self) -> file_shares::Client {
        file_shares::Client(self.clone())
    }
    pub fn iscsi_disks_client(&self) -> iscsi_disks::Client {
        iscsi_disks::Client(self.clone())
    }
    pub fn iscsi_servers_client(&self) -> iscsi_servers::Client {
        iscsi_servers::Client(self.clone())
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn managers_client(&self) -> managers::Client {
        managers::Client(self.clone())
    }
    pub fn storage_account_credentials_client(&self) -> storage_account_credentials::Client {
        storage_account_credentials::Client(self.clone())
    }
    pub fn storage_domains_client(&self) -> storage_domains::Client {
        storage_domains::Client(self.clone())
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
        #[doc = "* `manager`: The manager."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            manager: impl Into<models::Manager>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                manager: manager.into(),
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
        #[doc = "Upload Vault Cred Certificate.\r\nReturns UploadCertificateResponse"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `certificate_name`: Certificate Name"]
        #[doc = "* `upload_certificate_requestrequest`: UploadCertificateRequest Request"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn upload_registration_certificate(
            &self,
            certificate_name: impl Into<String>,
            upload_certificate_requestrequest: impl Into<models::UploadCertificateRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> upload_registration_certificate::Builder {
            upload_registration_certificate::Builder {
                client: self.0.clone(),
                certificate_name: certificate_name.into(),
                upload_certificate_requestrequest: upload_certificate_requestrequest.into(),
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
        #[doc = "* `manager_extended_info`: The manager extended information."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_extended_info(
            &self,
            manager_extended_info: impl Into<models::ManagerExtendedInfo>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_extended_info::Builder {
            create_extended_info::Builder {
                client: self.0.clone(),
                manager_extended_info: manager_extended_info.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Updates the extended info of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `manager_extended_info`: The manager extended information."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        #[doc = "* `if_match`: Pass the ETag of ExtendedInfo fetched from GET call"]
        pub fn update_extended_info(
            &self,
            manager_extended_info: impl Into<models::ManagerExtendedInfo>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
            if_match: impl Into<String>,
        ) -> update_extended_info::Builder {
            update_extended_info::Builder {
                client: self.0.clone(),
                manager_extended_info: manager_extended_info.into(),
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
        #[doc = "Returns the symmetric encryption key of the manager."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get_encryption_key(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get_encryption_key::Builder {
            get_encryption_key::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the  manager metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definition of all metrics aggregated at manager."]
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) manager: models::Manager,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.manager)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod upload_registration_certificate {
        use super::models;
        type Response = models::UploadCertificateResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) certificate_name: String,
            pub(crate) upload_certificate_requestrequest: models::UploadCertificateRequest,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/certificates/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.certificate_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.upload_certificate_requestrequest)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UploadCertificateResponse = serde_json::from_slice(&rsp_body)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) manager_extended_info: models::ManagerExtendedInfo,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.manager_extended_info)?;
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
            pub(crate) manager_extended_info: models::ManagerExtendedInfo,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.manager_extended_info)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod get_encryption_key {
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/getEncryptionKey",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod available_provider_operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List of AvailableProviderOperations"]
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
        #[doc = "* `access_control_record`: The access control record to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            access_control_record_name: impl Into<String>,
            access_control_record: impl Into<models::AccessControlRecord>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                access_control_record_name: access_control_record_name.into(),
                access_control_record: access_control_record.into(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) access_control_record: models::AccessControlRecord,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.access_control_record)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
        #[doc = "* `request`: The clear alert request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn clear(
            &self,
            request: impl Into<models::ClearAlertRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> clear::Builder {
            clear::Builder {
                client: self.0.clone(),
                request: request.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Sends a test alert email."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `request`: The send test alert email request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn send_test_email(
            &self,
            device_name: impl Into<String>,
            request: impl Into<models::SendTestAlertEmailRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> send_test_email::Builder {
            send_test_email::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                request: request.into(),
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) request: models::ClearAlertRequest,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
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
            pub(crate) request: models::SendTestAlertEmailRequest,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
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
pub mod backups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the backups in a manager."]
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
        #[doc = "Retrieves all the backups in a device. Can be used to get the backups for failover also."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
                for_failover: None,
                filter: None,
            }
        }
        #[doc = "Deletes the backup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Clones the given backup element to a new disk or share with given details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `backup_name`: The backup name."]
        #[doc = "* `element_name`: The backup element name."]
        #[doc = "* `clone_request`: The clone request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn clone(
            &self,
            device_name: impl Into<String>,
            backup_name: impl Into<String>,
            element_name: impl Into<String>,
            clone_request: impl Into<models::CloneRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> clone::Builder {
            clone::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                backup_name: backup_name.into(),
                element_name: element_name.into(),
                clone_request: clone_request.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::BackupList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/backups",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) for_failover: Option<bool>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Set to true if you need backups which can be used for failover."]
            pub fn for_failover(mut self, for_failover: bool) -> Self {
                self.for_failover = Some(for_failover);
                self
            }
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                                if let Some(for_failover) = &this.for_failover {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("forFailover", &for_failover.to_string());
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) element_name: String,
            pub(crate) clone_request: models::CloneRequest,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backups/{}/elements/{}/clone" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . backup_name , & this . element_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.clone_request)?;
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
        #[doc = "Retrieves all the devices in a manager."]
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
        #[doc = "Returns the properties of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "* `device_name`: The device Name."]
        #[doc = "* `device_patch`: Patch representation of the device."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn patch(
            &self,
            device_name: impl Into<String>,
            device_patch: impl Into<models::DevicePatch>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                device_patch: device_patch.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Returns the alert settings of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Creates or updates the alert settings"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `alert_settings`: The alert settings."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update_alert_settings(
            &self,
            device_name: impl Into<String>,
            alert_settings: impl Into<models::AlertSettings>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update_alert_settings::Builder {
            create_or_update_alert_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                alert_settings: alert_settings.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deactivates the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Downloads updates on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn download_updates(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> download_updates::Builder {
            download_updates::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Fails over the device to another device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `failover_request`: The failover request."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn failover(
            &self,
            device_name: impl Into<String>,
            failover_request: impl Into<models::FailoverRequest>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> failover::Builder {
            failover::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                failover_request: failover_request.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Retrieves all the devices which can be used as failover targets for the given device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_failover_target(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_failover_target::Builder {
            list_failover_target::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                expand: None,
            }
        }
        #[doc = "Installs the updates on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Retrieves the device metrics."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the appliance."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definition of all metrics aggregated at device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the appliance."]
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
        #[doc = "Returns the network settings of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Scans for updates on the device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Creates or updates the security settings."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `security_settings`: The security settings."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update_security_settings(
            &self,
            device_name: impl Into<String>,
            security_settings: impl Into<models::SecuritySettings>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update_security_settings::Builder {
            create_or_update_security_settings::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                security_settings: security_settings.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the time settings of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Returns the update summary of the specified device name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
            #[doc = "Specify $expand=details to populate additional fields related to the device."]
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            #[doc = "Specify $expand=details to populate additional fields related to the device."]
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod patch {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Device),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) device_patch: models::DevicePatch,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.device_patch)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Device = serde_json::from_slice(&rsp_body)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) alert_settings: models::AlertSettings,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.alert_settings)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod download_updates {
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/download",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) device_name: String,
            pub(crate) failover_request: models::FailoverRequest,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.failover_request)?;
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
    pub mod list_failover_target {
        use super::models;
        type Response = models::DeviceList;
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
            #[doc = "Specify $expand=details to populate additional fields related to the device."]
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/failoverTargets",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/install",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod create_or_update_security_settings {
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
            pub(crate) security_settings: models::SecuritySettings,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/securitySettings/default/update" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.security_settings)?;
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
}
pub mod backup_schedule_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the backup schedule groups in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
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
        #[doc = "Returns the properties of the specified backup schedule group name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
        #[doc = "* `schedule_group_name`: The name of the schedule group."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            schedule_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                schedule_group_name: schedule_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or Updates the backup schedule Group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
        #[doc = "* `schedule_group_name`: The name of the schedule group."]
        #[doc = "* `schedule_group`: The schedule group to be created"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            schedule_group_name: impl Into<String>,
            schedule_group: impl Into<models::BackupScheduleGroup>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                schedule_group_name: schedule_group_name.into(),
                schedule_group: schedule_group.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the backup schedule group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
        #[doc = "* `schedule_group_name`: The name of the schedule group."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            schedule_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                schedule_group_name: schedule_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::BackupScheduleGroupList;
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupScheduleGroups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupScheduleGroupList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupScheduleGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) schedule_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupScheduleGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . schedule_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupScheduleGroup = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::BackupScheduleGroup),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) schedule_group_name: String,
            pub(crate) schedule_group: models::BackupScheduleGroup,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupScheduleGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . schedule_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.schedule_group)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupScheduleGroup = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) schedule_group_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/backupScheduleGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . schedule_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod chap_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the chap settings in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
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
        #[doc = "Returns the properties of the specified chap setting name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `chap_user_name`: The user name of chap to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            chap_user_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                chap_user_name: chap_user_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the chap setting."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `chap_user_name`: The chap user name."]
        #[doc = "* `chap_setting`: The chap setting to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            chap_user_name: impl Into<String>,
            chap_setting: impl Into<models::ChapSettings>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                chap_user_name: chap_user_name.into(),
                chap_setting: chap_setting.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the chap setting."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `chap_user_name`: The chap user name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            chap_user_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                chap_user_name: chap_user_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::ChapSettingsList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/chapSettings",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChapSettingsList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ChapSettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) chap_user_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/chapSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.chap_user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChapSettings = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ChapSettings),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) chap_user_name: String,
            pub(crate) chap_setting: models::ChapSettings,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/chapSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.chap_user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.chap_setting)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChapSettings = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) chap_user_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/chapSettings/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.chap_user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod iscsi_disks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the iSCSI disks in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Retrieves all the disks in a iSCSI server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_iscsi_server(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_iscsi_server::Builder {
            list_by_iscsi_server::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the properties of the specified iSCSI disk name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `disk_name`: The disk name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            disk_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                disk_name: disk_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the iSCSI disk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `disk_name`: The disk name."]
        #[doc = "* `iscsi_disk`: The iSCSI disk."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            disk_name: impl Into<String>,
            iscsi_disk: impl Into<models::IscsiDisk>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                disk_name: disk_name.into(),
                iscsi_disk: iscsi_disk.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the iSCSI disk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `disk_name`: The disk name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            disk_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                disk_name: disk_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the iSCSI disk metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `disk_name`: The iSCSI disk name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            disk_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                disk_name: disk_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definitions for all metric aggregated at the iSCSI disk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `disk_name`: The iSCSI disk name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            disk_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                disk_name: disk_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::IscsiDiskList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/disks",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiDiskList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_iscsi_server {
        use super::models;
        type Response = models::IscsiDiskList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiDiskList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::IscsiDisk;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
            pub(crate) disk_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name , & this . disk_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiDisk = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::IscsiDisk),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
            pub(crate) disk_name: String,
            pub(crate) iscsi_disk: models::IscsiDisk,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name , & this . disk_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.iscsi_disk)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiDisk = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) iscsi_server_name: String,
            pub(crate) disk_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name , & this . disk_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) iscsi_server_name: String,
            pub(crate) disk_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name , & this . disk_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) iscsi_server_name: String,
            pub(crate) disk_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/disks/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name , & this . disk_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod file_servers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the file servers in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Returns the properties of the specified file server name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the file server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `file_server`: The file server."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            file_server: impl Into<models::FileServer>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                file_server: file_server.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the file server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Backup the file server now."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn backup_now(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> backup_now::Builder {
            backup_now::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the file server metrics."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
        #[doc = "* `file_server_name`: The name of the file server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definitions of all metrics aggregated at the file server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The name of the device."]
        #[doc = "* `file_server_name`: The name of the file server."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Retrieves all the file servers in a manager."]
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
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::FileServerList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileServerList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::FileServer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.file_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileServer = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::FileServer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
            pub(crate) file_server: models::FileServer,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.file_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.file_server)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileServer = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) file_server_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.file_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) file_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/backup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) file_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod list_by_manager {
        use super::models;
        type Response = models::FileServerList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/fileservers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileServerList = serde_json::from_slice(&rsp_body)?;
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
pub mod file_shares {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the file shares in a file server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_by_file_server(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_by_file_server::Builder {
            list_by_file_server::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Returns the properties of the specified file share name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `share_name`: The file share name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            share_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                share_name: share_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the file share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `share_name`: The file share name."]
        #[doc = "* `file_share`: The file share."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            share_name: impl Into<String>,
            file_share: impl Into<models::FileShare>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                share_name: share_name.into(),
                file_share: file_share.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the file share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `share_name`: The file share Name"]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            share_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                share_name: share_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the file share metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `share_name`: The file share name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            share_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                share_name: share_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definitions of all metrics aggregated at the file share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `file_server_name`: The file server name."]
        #[doc = "* `share_name`: The file share name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            file_server_name: impl Into<String>,
            share_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                file_server_name: file_server_name.into(),
                share_name: share_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Retrieves all the file shares in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
    pub mod list_by_file_server {
        use super::models;
        type Response = models::FileShareList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileShareList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::FileShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
            pub(crate) share_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) manager_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name , & this . share_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileShare = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::FileShare),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) file_server_name: String,
            pub(crate) share_name: String,
            pub(crate) file_share: models::FileShare,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name , & this . share_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.file_share)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileShare = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) file_server_name: String,
            pub(crate) share_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name , & this . share_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) file_server_name: String,
            pub(crate) share_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name , & this . share_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) file_server_name: String,
            pub(crate) share_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/fileservers/{}/shares/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . file_server_name , & this . share_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
        type Response = models::FileShareList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/shares",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileShareList = serde_json::from_slice(&rsp_body)?;
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
pub mod iscsi_servers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the iSCSI in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Returns the properties of the specified iSCSI server name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the iSCSI server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `iscsi_server`: The iSCSI server."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            iscsi_server: impl Into<models::IscsiServer>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                iscsi_server: iscsi_server.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the iSCSI server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Backup the iSCSI server now."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn backup_now(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> backup_now::Builder {
            backup_now::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Gets the iSCSI server metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metrics(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metrics::Builder {
            list_metrics::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
                filter: None,
            }
        }
        #[doc = "Retrieves metric definitions for all metrics aggregated at iSCSI server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `iscsi_server_name`: The iSCSI server name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn list_metric_definition(
            &self,
            device_name: impl Into<String>,
            iscsi_server_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> list_metric_definition::Builder {
            list_metric_definition::Builder {
                client: self.0.clone(),
                device_name: device_name.into(),
                iscsi_server_name: iscsi_server_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Retrieves all the iSCSI servers in a manager."]
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
    }
    pub mod list_by_device {
        use super::models;
        type Response = models::IscsiServerList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiServerList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::IscsiServer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.iscsi_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiServer = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::IscsiServer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
            pub(crate) iscsi_server: models::IscsiServer,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.iscsi_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.iscsi_server)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiServer = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) iscsi_server_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.device_name,
                            &this.iscsi_server_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) iscsi_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/backup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod list_metrics {
        use super::models;
        type Response = models::MetricList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) device_name: String,
            pub(crate) iscsi_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/metrics" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) iscsi_server_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/devices/{}/iscsiservers/{}/metricsDefinitions" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . manager_name , & this . device_name , & this . iscsi_server_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
    pub mod list_by_manager {
        use super::models;
        type Response = models::IscsiServerList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/iscsiservers",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IscsiServerList = serde_json::from_slice(&rsp_body)?;
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
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the jobs in a device."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
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
        #[doc = "Returns the properties of the specified job name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `device_name`: The device name."]
        #[doc = "* `job_name`: The job name."]
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
        #[doc = "Retrieves all the jobs in a manager."]
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod storage_account_credentials {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the storage account credentials in a manager."]
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
        #[doc = "Returns the properties of the specified storage account credential name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `credential_name`: The name of storage account credential to be fetched."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            credential_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                credential_name: credential_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the storage account credential"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `credential_name`: The credential name."]
        #[doc = "* `storage_account`: The storage account credential to be added or updated."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            credential_name: impl Into<String>,
            storage_account: impl Into<models::StorageAccountCredential>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                credential_name: credential_name.into(),
                storage_account: storage_account.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the storage account credential"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `credential_name`: The name of the storage account credential."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            credential_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                credential_name: credential_name.into(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) credential_name: String,
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
                            &this.credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
            pub(crate) credential_name: String,
            pub(crate) storage_account: models::StorageAccountCredential,
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
                            &this.credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.storage_account)?;
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
            pub(crate) credential_name: String,
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
                            &this.credential_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
pub mod storage_domains {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all the storage domains in a manager."]
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
        #[doc = "Returns the properties of the specified storage domain name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_domain_name`: The storage domain name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn get(
            &self,
            storage_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                storage_domain_name: storage_domain_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Creates or updates the storage domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_domain_name`: The storage domain name."]
        #[doc = "* `storage_domain`: The storageDomain."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn create_or_update(
            &self,
            storage_domain_name: impl Into<String>,
            storage_domain: impl Into<models::StorageDomain>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                storage_domain_name: storage_domain_name.into(),
                storage_domain: storage_domain.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
        #[doc = "Deletes the storage domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_domain_name`: The storage domain name."]
        #[doc = "* `subscription_id`: The subscription id"]
        #[doc = "* `resource_group_name`: The resource group name"]
        #[doc = "* `manager_name`: The manager name"]
        pub fn delete(
            &self,
            storage_domain_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            manager_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                storage_domain_name: storage_domain_name.into(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                manager_name: manager_name.into(),
            }
        }
    }
    pub mod list_by_manager {
        use super::models;
        type Response = models::StorageDomainList;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageDomains",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageDomainList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StorageDomain;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_domain_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageDomain = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::StorageDomain),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_domain_name: String,
            pub(crate) storage_domain: models::StorageDomain,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.storage_domain)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageDomain = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) storage_domain_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StorSimple/managers/{}/storageDomains/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.manager_name,
                            &this.storage_domain_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2016-10-01");
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
