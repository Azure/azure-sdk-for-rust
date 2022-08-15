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
    pub fn migration_recovery_points_client(&self) -> migration_recovery_points::Client {
        migration_recovery_points::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn recovery_points_client(&self) -> recovery_points::Client {
        recovery_points::Client(self.clone())
    }
    pub fn replication_alert_settings_client(&self) -> replication_alert_settings::Client {
        replication_alert_settings::Client(self.clone())
    }
    pub fn replication_appliances_client(&self) -> replication_appliances::Client {
        replication_appliances::Client(self.clone())
    }
    pub fn replication_eligibility_results_client(&self) -> replication_eligibility_results::Client {
        replication_eligibility_results::Client(self.clone())
    }
    pub fn replication_events_client(&self) -> replication_events::Client {
        replication_events::Client(self.clone())
    }
    pub fn replication_fabrics_client(&self) -> replication_fabrics::Client {
        replication_fabrics::Client(self.clone())
    }
    pub fn replication_jobs_client(&self) -> replication_jobs::Client {
        replication_jobs::Client(self.clone())
    }
    pub fn replication_logical_networks_client(&self) -> replication_logical_networks::Client {
        replication_logical_networks::Client(self.clone())
    }
    pub fn replication_migration_items_client(&self) -> replication_migration_items::Client {
        replication_migration_items::Client(self.clone())
    }
    pub fn replication_network_mappings_client(&self) -> replication_network_mappings::Client {
        replication_network_mappings::Client(self.clone())
    }
    pub fn replication_networks_client(&self) -> replication_networks::Client {
        replication_networks::Client(self.clone())
    }
    pub fn replication_policies_client(&self) -> replication_policies::Client {
        replication_policies::Client(self.clone())
    }
    pub fn replication_protectable_items_client(&self) -> replication_protectable_items::Client {
        replication_protectable_items::Client(self.clone())
    }
    pub fn replication_protected_items_client(&self) -> replication_protected_items::Client {
        replication_protected_items::Client(self.clone())
    }
    pub fn replication_protection_container_mappings_client(&self) -> replication_protection_container_mappings::Client {
        replication_protection_container_mappings::Client(self.clone())
    }
    pub fn replication_protection_containers_client(&self) -> replication_protection_containers::Client {
        replication_protection_containers::Client(self.clone())
    }
    pub fn replication_protection_intents_client(&self) -> replication_protection_intents::Client {
        replication_protection_intents::Client(self.clone())
    }
    pub fn replication_recovery_plans_client(&self) -> replication_recovery_plans::Client {
        replication_recovery_plans::Client(self.clone())
    }
    pub fn replication_recovery_services_providers_client(&self) -> replication_recovery_services_providers::Client {
        replication_recovery_services_providers::Client(self.clone())
    }
    pub fn replication_storage_classification_mappings_client(&self) -> replication_storage_classification_mappings::Client {
        replication_storage_classification_mappings::Client(self.clone())
    }
    pub fn replication_storage_classifications_client(&self) -> replication_storage_classifications::Client {
        replication_storage_classifications::Client(self.clone())
    }
    pub fn replication_vault_health_client(&self) -> replication_vault_health::Client {
        replication_vault_health::Client(self.clone())
    }
    pub fn replication_vault_setting_client(&self) -> replication_vault_setting::Client {
        replication_vault_setting::Client(self.clone())
    }
    pub fn replicationv_centers_client(&self) -> replicationv_centers::Client {
        replicationv_centers::Client(self.clone())
    }
    pub fn supported_operating_systems_client(&self) -> supported_operating_systems::Client {
        supported_operating_systems::Client(self.clone())
    }
    pub fn target_compute_sizes_client(&self) -> target_compute_sizes::Client {
        target_compute_sizes::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the list of available operations."]
        #[doc = "Operation to return the list of available operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(&self, resource_group_name: impl Into<String>, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationsDiscoveryCollection;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/operations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsDiscoveryCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_alert_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of configured email notification(alert) configurations."]
        #[doc = "Gets the list of email notification(alert) configurations for the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets an email notification(alert) configuration."]
        #[doc = "Gets the details of the specified email notification(alert) configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `alert_setting_name`: The name of the email notification configuration."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            alert_setting_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                alert_setting_name: alert_setting_name.into(),
            }
        }
        #[doc = "Configures email notifications for this vault."]
        #[doc = "Create or update an email notification(alert) configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `alert_setting_name`: The name of the email notification(alert) configuration."]
        #[doc = "* `request`: The input to configure the email notification(alert)."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            alert_setting_name: impl Into<String>,
            request: impl Into<models::ConfigureAlertRequest>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                alert_setting_name: alert_setting_name.into(),
                request: request.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AlertCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationAlertSettings",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Alert;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) alert_setting_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationAlertSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . alert_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Alert = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Alert;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) alert_setting_name: String,
            pub(crate) request: models::ConfigureAlertRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationAlertSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . alert_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Alert = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_appliances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of appliances."]
        #[doc = "Gets the list of Azure Site Recovery appliances for the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ApplianceCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationAppliances",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                let rsp_value: models::ApplianceCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_eligibility_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the validation errors in case the VM is unsuitable for protection."]
        #[doc = "Validates whether a given VM can be protected or not in which case returns list of errors."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `virtual_machine_name`: Virtual Machine name."]
        pub fn list(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Gets the validation errors in case the VM is unsuitable for protection."]
        #[doc = "Validates whether a given VM can be protected or not in which case returns list of errors."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `virtual_machine_name`: Virtual Machine name."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ReplicationEligibilityResultsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/providers/Microsoft.RecoveryServices/replicationEligibilityResults" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . virtual_machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationEligibilityResultsCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReplicationEligibilityResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/providers/Microsoft.RecoveryServices/replicationEligibilityResults/default" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . virtual_machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationEligibilityResults = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_events {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of Azure Site Recovery events."]
        #[doc = "Gets the list of Azure Site Recovery events for the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
        #[doc = "Get the details of an Azure Site recovery event."]
        #[doc = "The operation to get the details of an Azure Site recovery event."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `event_name`: The name of the Azure Site Recovery event."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            event_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                event_name: event_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::EventCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationEvents",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                let rsp_value: models::EventCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Event;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) event_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationEvents/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
                            &this.event_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Event = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_fabrics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of ASR fabrics."]
        #[doc = "Gets a list of the Azure Site Recovery fabrics in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the details of an ASR fabric."]
        #[doc = "Gets the details of an Azure Site Recovery fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                filter: None,
            }
        }
        #[doc = "Creates an Azure Site Recovery fabric."]
        #[doc = "The operation to create an Azure Site Recovery fabric (for e.g. Hyper-V site)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Name of the ASR fabric."]
        #[doc = "* `input`: Fabric creation input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            input: impl Into<models::FabricCreationInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Purges the site."]
        #[doc = "The operation to purge(force delete) an Azure Site Recovery fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: ASR fabric to purge."]
        pub fn purge(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> purge::Builder {
            purge::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Checks the consistency of the ASR fabric."]
        #[doc = "The operation to perform a consistency check on the fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn check_consistency(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> check_consistency::Builder {
            check_consistency::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Migrates the site to AAD."]
        #[doc = "The operation to migrate an Azure Site Recovery fabric to AAD."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: ASR fabric to migrate."]
        pub fn migrate_to_aad(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> migrate_to_aad::Builder {
            migrate_to_aad::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Perform failover of the process server."]
        #[doc = "The operation to move replications from a process server to another process server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The name of the fabric containing the process server."]
        #[doc = "* `failover_process_server_request`: The input to the failover process server operation."]
        pub fn reassociate_gateway(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            failover_process_server_request: impl Into<models::FailoverProcessServerRequest>,
        ) -> reassociate_gateway::Builder {
            reassociate_gateway::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                failover_process_server_request: failover_process_server_request.into(),
            }
        }
        #[doc = "Deletes the site."]
        #[doc = "The operation to delete or remove an Azure Site Recovery fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: ASR fabric to delete."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Renews certificate for the fabric."]
        #[doc = "Renews the connection certificate for the ASR replication fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: fabric name to renew certs for."]
        #[doc = "* `renew_certificate`: Renew certificate input."]
        pub fn renew_certificate(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            renew_certificate: impl Into<models::RenewCertificateInput>,
        ) -> renew_certificate::Builder {
            renew_certificate::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                renew_certificate: renew_certificate.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::FabricCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FabricCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Fabric;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
                            &this.fabric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                let rsp_value: models::Fabric = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Fabric),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) input: models::FabricCreationInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
                            &this.fabric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Fabric = serde_json::from_slice(&rsp_body)?;
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
    pub mod purge {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
                            &this.fabric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod check_consistency {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Fabric),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/checkConsistency" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Fabric = serde_json::from_slice(&rsp_body)?;
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
    pub mod migrate_to_aad {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/migratetoaad" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod reassociate_gateway {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Fabric),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) failover_process_server_request: models::FailoverProcessServerRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/reassociateGateway" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.failover_process_server_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Fabric = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/remove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod renew_certificate {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Fabric),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) renew_certificate: models::RenewCertificateInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/renewCertificate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.renew_certificate)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Fabric = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_logical_networks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of logical networks under a fabric."]
        #[doc = "Lists all the logical networks of the Azure Site Recovery fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Server Id."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets a logical network with specified server id and logical network name."]
        #[doc = "Gets the details of a logical network."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Server Id."]
        #[doc = "* `logical_network_name`: Logical network name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            logical_network_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                logical_network_name: logical_network_name.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::LogicalNetworkCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationLogicalNetworks" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogicalNetworkCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::LogicalNetwork;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) logical_network_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationLogicalNetworks/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . logical_network_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogicalNetwork = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_networks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of networks under a fabric."]
        #[doc = "Lists the networks available for a fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets a network with specified server id and network name."]
        #[doc = "Gets the details of a network."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Server Id."]
        #[doc = "* `network_name`: Primary network name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
            }
        }
        #[doc = "Gets the list of networks. View-only API."]
        #[doc = "Lists the networks available in a vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::NetworkCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Network;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Network = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::NetworkCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationNetworks",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_network_mappings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the network mappings under a network."]
        #[doc = "Lists all ASR network mappings for the specified network."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Primary fabric name."]
        #[doc = "* `network_name`: Primary network name."]
        pub fn list_by_replication_networks(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
        ) -> list_by_replication_networks::Builder {
            list_by_replication_networks::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
            }
        }
        #[doc = "Gets network mapping by name."]
        #[doc = "Gets the details of an ASR network mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Primary fabric name."]
        #[doc = "* `network_name`: Primary network name."]
        #[doc = "* `network_mapping_name`: Network mapping name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
            network_mapping_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
                network_mapping_name: network_mapping_name.into(),
            }
        }
        #[doc = "Creates network mapping."]
        #[doc = "The operation to create an ASR network mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Primary fabric name."]
        #[doc = "* `network_name`: Primary network name."]
        #[doc = "* `network_mapping_name`: Network mapping name."]
        #[doc = "* `input`: Create network mapping input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
            network_mapping_name: impl Into<String>,
            input: impl Into<models::CreateNetworkMappingInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
                network_mapping_name: network_mapping_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Updates network mapping."]
        #[doc = "The operation to update an ASR network mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Primary fabric name."]
        #[doc = "* `network_name`: Primary network name."]
        #[doc = "* `network_mapping_name`: Network mapping name."]
        #[doc = "* `input`: Update network mapping input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
            network_mapping_name: impl Into<String>,
            input: impl Into<models::UpdateNetworkMappingInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
                network_mapping_name: network_mapping_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Delete network mapping."]
        #[doc = "The operation to delete a network mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Primary fabric name."]
        #[doc = "* `network_name`: Primary network name."]
        #[doc = "* `network_mapping_name`: ARM Resource Name for network mapping."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            network_name: impl Into<String>,
            network_mapping_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                network_name: network_name.into(),
                network_mapping_name: network_mapping_name.into(),
            }
        }
        #[doc = "Gets all the network mappings under a vault."]
        #[doc = "Lists all ASR network mappings in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_networks {
        use super::models;
        type Response = models::NetworkMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}/replicationNetworkMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkMappingCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::NetworkMapping;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
            pub(crate) network_mapping_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}/replicationNetworkMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name , & this . network_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkMapping = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::NetworkMapping),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
            pub(crate) network_mapping_name: String,
            pub(crate) input: models::CreateNetworkMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}/replicationNetworkMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name , & this . network_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkMapping = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::NetworkMapping),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
            pub(crate) network_mapping_name: String,
            pub(crate) input: models::UpdateNetworkMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}/replicationNetworkMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name , & this . network_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkMapping = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) network_name: String,
            pub(crate) network_mapping_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationNetworks/{}/replicationNetworkMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . network_name , & this . network_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod list {
        use super::models;
        type Response = models::NetworkMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationNetworkMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NetworkMappingCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_protection_containers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of protection container for a fabric."]
        #[doc = "Lists the protection containers in the specified fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets the protection container details."]
        #[doc = "Gets the details of a protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
            }
        }
        #[doc = "Create a protection container."]
        #[doc = "Operation to create a protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric ARM name."]
        #[doc = "* `protection_container_name`: Unique protection container ARM name."]
        #[doc = "* `creation_input`: Creation input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            creation_input: impl Into<models::CreateProtectionContainerInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                creation_input: creation_input.into(),
            }
        }
        #[doc = "Adds a protectable item to the replication protection container."]
        #[doc = "The operation to a add a protectable item to a protection container(Add physical server)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The name of the fabric."]
        #[doc = "* `protection_container_name`: The name of the protection container."]
        #[doc = "* `discover_protectable_item_request`: The request object to add a protectable item."]
        pub fn discover_protectable_item(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            discover_protectable_item_request: impl Into<models::DiscoverProtectableItemRequest>,
        ) -> discover_protectable_item::Builder {
            discover_protectable_item::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                discover_protectable_item_request: discover_protectable_item_request.into(),
            }
        }
        #[doc = "Removes a protection container."]
        #[doc = "Operation to remove a protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric ARM name."]
        #[doc = "* `protection_container_name`: Unique protection container ARM name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
            }
        }
        #[doc = "Switches protection from one container to another or one replication provider to another."]
        #[doc = "Operation to switch protection from one container to another or one replication provider to another."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `switch_input`: Switch protection input."]
        pub fn switch_protection(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            switch_input: impl Into<models::SwitchProtectionInput>,
        ) -> switch_protection::Builder {
            switch_protection::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                switch_input: switch_input.into(),
            }
        }
        #[doc = "Gets the list of all protection containers in a vault."]
        #[doc = "Lists the protection containers in a vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::ProtectionContainerCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProtectionContainer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainer = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProtectionContainer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) creation_input: models::CreateProtectionContainerInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.creation_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainer = serde_json::from_slice(&rsp_body)?;
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
    pub mod discover_protectable_item {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectionContainer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) discover_protectable_item_request: models::DiscoverProtectableItemRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/discoverProtectableItem" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.discover_protectable_item_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainer = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/remove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod switch_protection {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectionContainer),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) switch_input: models::SwitchProtectionInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/switchprotection" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.switch_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainer = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::ProtectionContainerCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectionContainers" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_migration_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of migration items in the protection container."]
        #[doc = "Gets the list of ASR migration items in the protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        pub fn list_by_replication_protection_containers(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> list_by_replication_protection_containers::Builder {
            list_by_replication_protection_containers::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                skip_token: None,
                take_token: None,
                filter: None,
            }
        }
        #[doc = "Gets the details of a migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric unique name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
            }
        }
        #[doc = "Enables migration."]
        #[doc = "The operation to create an ASR migration item (enable migration)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `input`: Enable migration input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            input: impl Into<models::EnableMigrationInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Updates migration item."]
        #[doc = "The operation to update the recovery settings of an ASR migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `input`: Update migration item input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            input: impl Into<models::UpdateMigrationItemInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Delete the migration item."]
        #[doc = "The operation to delete an ASR migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                delete_option: None,
            }
        }
        #[doc = "Migrate item."]
        #[doc = "The operation to initiate migration of the item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `migrate_input`: Migrate input."]
        pub fn migrate(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            migrate_input: impl Into<models::MigrateInput>,
        ) -> migrate::Builder {
            migrate::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                migrate_input: migrate_input.into(),
            }
        }
        #[doc = "Resynchronizes replication."]
        #[doc = "The operation to resynchronize replication of an ASR migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `input`: Resync input."]
        pub fn resync(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            input: impl Into<models::ResyncInput>,
        ) -> resync::Builder {
            resync::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Test migrate item."]
        #[doc = "The operation to initiate test migration of the item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `test_migrate_input`: Test migrate input."]
        pub fn test_migrate(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            test_migrate_input: impl Into<models::TestMigrateInput>,
        ) -> test_migrate::Builder {
            test_migrate::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                test_migrate_input: test_migrate_input.into(),
            }
        }
        #[doc = "Test migrate cleanup."]
        #[doc = "The operation to initiate test migrate cleanup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `test_migrate_cleanup_input`: Test migrate cleanup input."]
        pub fn test_migrate_cleanup(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            test_migrate_cleanup_input: impl Into<models::TestMigrateCleanupInput>,
        ) -> test_migrate_cleanup::Builder {
            test_migrate_cleanup::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                test_migrate_cleanup_input: test_migrate_cleanup_input.into(),
            }
        }
        #[doc = "Gets the list of migration items in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                skip_token: None,
                take_token: None,
                filter: None,
            }
        }
    }
    pub mod list_by_replication_protection_containers {
        use super::models;
        type Response = models::MigrationItemCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) take_token: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The pagination token."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "The page size."]
            pub fn take_token(mut self, take_token: impl Into<String>) -> Self {
                self.take_token = Some(take_token.into());
                self
            }
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(take_token) = &this.take_token {
                                    req.url_mut().query_pairs_mut().append_pair("takeToken", take_token);
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
                                let rsp_value: models::MigrationItemCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::MigrationItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) input: models::EnableMigrationInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) input: models::UpdateMigrationItemInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) delete_option: Option<String>,
        }
        impl Builder {
            #[doc = "The delete option."]
            pub fn delete_option(mut self, delete_option: impl Into<String>) -> Self {
                self.delete_option = Some(delete_option.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        if let Some(delete_option) = &this.delete_option {
                            req.url_mut().query_pairs_mut().append_pair("deleteOption", delete_option);
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
    pub mod migrate {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) migrate_input: models::MigrateInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/migrate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.migrate_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod resync {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) input: models::ResyncInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/resync" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_migrate {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) test_migrate_input: models::TestMigrateInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/testMigrate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.test_migrate_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_migrate_cleanup {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrationItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) test_migrate_cleanup_input: models::TestMigrateCleanupInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/testMigrateCleanup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.test_migrate_cleanup_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::MigrationItemCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) take_token: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The pagination token."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "The page size."]
            pub fn take_token(mut self, take_token: impl Into<String>) -> Self {
                self.take_token = Some(take_token.into());
                self
            }
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationMigrationItems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(take_token) = &this.take_token {
                                    req.url_mut().query_pairs_mut().append_pair("takeToken", take_token);
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
                                let rsp_value: models::MigrationItemCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod migration_recovery_points {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the recovery points for a migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric unique name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        pub fn list_by_replication_migration_items(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
        ) -> list_by_replication_migration_items::Builder {
            list_by_replication_migration_items::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
            }
        }
        #[doc = "Gets a recovery point for a migration item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric unique name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `migration_item_name`: Migration item name."]
        #[doc = "* `migration_recovery_point_name`: The migration recovery point name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            migration_item_name: impl Into<String>,
            migration_recovery_point_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                migration_item_name: migration_item_name.into(),
                migration_recovery_point_name: migration_recovery_point_name.into(),
            }
        }
    }
    pub mod list_by_replication_migration_items {
        use super::models;
        type Response = models::MigrationRecoveryPointCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/migrationRecoveryPoints" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationRecoveryPointCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::MigrationRecoveryPoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) migration_item_name: String,
            pub(crate) migration_recovery_point_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationMigrationItems/{}/migrationRecoveryPoints/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . migration_item_name , & this . migration_recovery_point_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrationRecoveryPoint = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_protectable_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of protectable items."]
        #[doc = "Lists the protectable items in a protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        pub fn list_by_replication_protection_containers(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> list_by_replication_protection_containers::Builder {
            list_by_replication_protection_containers::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                filter: None,
                take: None,
                skip_token: None,
            }
        }
        #[doc = "Gets the details of a protectable item."]
        #[doc = "The operation to get the details of a protectable item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `protectable_item_name`: Protectable item name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            protectable_item_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                protectable_item_name: protectable_item_name.into(),
            }
        }
    }
    pub mod list_by_replication_protection_containers {
        use super::models;
        type Response = models::ProtectableItemCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) take: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "take OData query parameter."]
            pub fn take(mut self, take: impl Into<String>) -> Self {
                self.take = Some(take.into());
                self
            }
            #[doc = "skipToken OData query parameter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectableItems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(take) = &this.take {
                                    req.url_mut().query_pairs_mut().append_pair("$take", take);
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
                                let rsp_value: models::ProtectableItemCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProtectableItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) protectable_item_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectableItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . protectable_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectableItem = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_protected_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of Replication protected items."]
        #[doc = "Gets the list of ASR replication protected items in the protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        pub fn list_by_replication_protection_containers(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> list_by_replication_protection_containers::Builder {
            list_by_replication_protection_containers::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
            }
        }
        #[doc = "Gets the details of a Replication protected item."]
        #[doc = "Gets the details of an ASR replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric unique name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Enables protection."]
        #[doc = "The operation to create an ASR replication protected item (Enable replication)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Name of the fabric."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: A name for the replication protected item."]
        #[doc = "* `input`: Enable Protection Input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            input: impl Into<models::EnableProtectionInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Updates the replication protected item settings."]
        #[doc = "The operation to update the recovery settings of an ASR replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `update_protection_input`: Update protection input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            update_protection_input: impl Into<models::UpdateReplicationProtectedItemInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                update_protection_input: update_protection_input.into(),
            }
        }
        #[doc = "Purges protection."]
        #[doc = "The operation to delete or purge a replication protected item. This operation will force delete the replication protected item. Use the remove operation on replication protected item to perform a clean disable replication for the item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        pub fn purge(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> purge::Builder {
            purge::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Add disk(s) for protection."]
        #[doc = "Operation to add disks(s) to the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `add_disks_input`: Add disks input."]
        pub fn add_disks(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            add_disks_input: impl Into<models::AddDisksInput>,
        ) -> add_disks::Builder {
            add_disks::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                add_disks_input: add_disks_input.into(),
            }
        }
        #[doc = "Change or apply recovery point."]
        #[doc = "The operation to change the recovery point of a failed over replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The ARM fabric name."]
        #[doc = "* `protection_container_name`: The protection container name."]
        #[doc = "* `replicated_protected_item_name`: The replicated protected item name."]
        #[doc = "* `apply_recovery_point_input`: The ApplyRecoveryPointInput."]
        pub fn apply_recovery_point(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            apply_recovery_point_input: impl Into<models::ApplyRecoveryPointInput>,
        ) -> apply_recovery_point::Builder {
            apply_recovery_point::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                apply_recovery_point_input: apply_recovery_point_input.into(),
            }
        }
        #[doc = "Execute cancel failover."]
        #[doc = "Operation to cancel the failover of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        pub fn failover_cancel(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> failover_cancel::Builder {
            failover_cancel::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Execute commit failover."]
        #[doc = "Operation to commit the failover of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        pub fn failover_commit(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> failover_commit::Builder {
            failover_commit::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Execute planned failover."]
        #[doc = "Operation to initiate a planned failover of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `failover_input`: Planned failover input."]
        pub fn planned_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            failover_input: impl Into<models::PlannedFailoverInput>,
        ) -> planned_failover::Builder {
            planned_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                failover_input: failover_input.into(),
            }
        }
        #[doc = "Disables protection."]
        #[doc = "The operation to disable replication on a replication protected item. This will also remove the item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `disable_protection_input`: Disable protection input."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            disable_protection_input: impl Into<models::DisableProtectionInput>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                disable_protection_input: disable_protection_input.into(),
            }
        }
        #[doc = "Removes disk(s)."]
        #[doc = "Operation to remove disk(s) from the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `remove_disks_input`: Remove disks input."]
        pub fn remove_disks(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            remove_disks_input: impl Into<models::RemoveDisksInput>,
        ) -> remove_disks::Builder {
            remove_disks::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                remove_disks_input: remove_disks_input.into(),
            }
        }
        #[doc = "Resynchronize or repair replication."]
        #[doc = "The operation to start resynchronize/repair replication for a replication protected item requiring resynchronization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The name of the fabric."]
        #[doc = "* `protection_container_name`: The name of the container."]
        #[doc = "* `replicated_protected_item_name`: The name of the replication protected item."]
        pub fn repair_replication(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> repair_replication::Builder {
            repair_replication::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Execute Reverse Replication\\Reprotect."]
        #[doc = "Operation to reprotect or reverse replicate a failed over replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `reprotect_input`: Reverse replication input."]
        pub fn reprotect(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            reprotect_input: impl Into<models::ReverseReplicationInput>,
        ) -> reprotect::Builder {
            reprotect::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                reprotect_input: reprotect_input.into(),
            }
        }
        #[doc = "Resolve health errors."]
        #[doc = "Operation to resolve health issues of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `resolve_health_input`: Health issue input object."]
        pub fn resolve_health_errors(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            resolve_health_input: impl Into<models::ResolveHealthInput>,
        ) -> resolve_health_errors::Builder {
            resolve_health_errors::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                resolve_health_input: resolve_health_input.into(),
            }
        }
        #[doc = "Execute switch provider."]
        #[doc = "Operation to initiate a switch provider of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `switch_provider_input`: Switch provider input."]
        pub fn switch_provider(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            switch_provider_input: impl Into<models::SwitchProviderInput>,
        ) -> switch_provider::Builder {
            switch_provider::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                switch_provider_input: switch_provider_input.into(),
            }
        }
        #[doc = "Execute test failover."]
        #[doc = "Operation to perform a test failover of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `testfailover_input`: Test failover input."]
        pub fn test_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            testfailover_input: impl Into<models::TestFailoverInput>,
        ) -> test_failover::Builder {
            test_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                testfailover_input: testfailover_input.into(),
            }
        }
        #[doc = "Execute test failover cleanup."]
        #[doc = "Operation to clean up the test failover of a replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `cleanup_input`: Test failover cleanup input."]
        pub fn test_failover_cleanup(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            cleanup_input: impl Into<models::TestFailoverCleanupInput>,
        ) -> test_failover_cleanup::Builder {
            test_failover_cleanup::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                cleanup_input: cleanup_input.into(),
            }
        }
        #[doc = "Execute unplanned failover."]
        #[doc = "Operation to initiate a failover of the replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Unique fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `failover_input`: Failover input."]
        pub fn unplanned_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            failover_input: impl Into<models::UnplannedFailoverInput>,
        ) -> unplanned_failover::Builder {
            unplanned_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                failover_input: failover_input.into(),
            }
        }
        #[doc = "Updates appliance for replication protected Item."]
        #[doc = "The operation to update appliance of an ASR replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        #[doc = "* `appliance_update_input`: Appliance update protection input."]
        pub fn update_appliance(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            appliance_update_input: impl Into<models::UpdateApplianceForReplicationProtectedItemInput>,
        ) -> update_appliance::Builder {
            update_appliance::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                appliance_update_input: appliance_update_input.into(),
            }
        }
        #[doc = "Update the mobility service on a protected item."]
        #[doc = "The operation to update(push update) the installed mobility service software on a replication protected item to the latest available version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The name of the fabric containing the protected item."]
        #[doc = "* `protection_container_name`: The name of the container containing the protected item."]
        #[doc = "* `replication_protected_item_name`: The name of the protected item on which the agent is to be updated."]
        #[doc = "* `update_mobility_service_request`: Request to update the mobility service on the protected item."]
        pub fn update_mobility_service(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replication_protected_item_name: impl Into<String>,
            update_mobility_service_request: impl Into<models::UpdateMobilityServiceRequest>,
        ) -> update_mobility_service::Builder {
            update_mobility_service::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replication_protected_item_name: replication_protected_item_name.into(),
                update_mobility_service_request: update_mobility_service_request.into(),
            }
        }
        #[doc = "Gets the list of replication protected items."]
        #[doc = "Gets the list of ASR replication protected items in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                skip_token: None,
                filter: None,
            }
        }
    }
    pub mod list_by_replication_protection_containers {
        use super::models;
        type Response = models::ReplicationProtectedItemCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItemCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReplicationProtectedItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) input: models::EnableProtectionInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) update_protection_input: models::UpdateReplicationProtectedItemInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_protection_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod purge {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod add_disks {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) add_disks_input: models::AddDisksInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/addDisks" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.add_disks_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod apply_recovery_point {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) apply_recovery_point_input: models::ApplyRecoveryPointInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/applyRecoveryPoint" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.apply_recovery_point_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod failover_cancel {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/failoverCancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod failover_commit {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/failoverCommit" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod planned_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) failover_input: models::PlannedFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/plannedFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.failover_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) disable_protection_input: models::DisableProtectionInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/remove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.disable_protection_input)?;
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
    pub mod remove_disks {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) remove_disks_input: models::RemoveDisksInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/removeDisks" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.remove_disks_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod repair_replication {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/repairReplication" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod reprotect {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) reprotect_input: models::ReverseReplicationInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/reProtect" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.reprotect_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod resolve_health_errors {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) resolve_health_input: models::ResolveHealthInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/resolveHealthErrors" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.resolve_health_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod switch_provider {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) switch_provider_input: models::SwitchProviderInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/switchProvider" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.switch_provider_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) testfailover_input: models::TestFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/testFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.testfailover_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_failover_cleanup {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) cleanup_input: models::TestFailoverCleanupInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/testFailoverCleanup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.cleanup_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod unplanned_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) failover_input: models::UnplannedFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/unplannedFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.failover_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_appliance {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) appliance_update_input: models::UpdateApplianceForReplicationProtectedItemInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/updateAppliance" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.appliance_update_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_mobility_service {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ReplicationProtectedItem),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replication_protected_item_name: String,
            pub(crate) update_mobility_service_request: models::UpdateMobilityServiceRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/updateMobilityService" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replication_protected_item_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_mobility_service_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectedItem = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::ReplicationProtectedItemCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The pagination token. Possible values: \"FabricId\" or \"FabricId_CloudId\" or null."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectedItems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
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
                                let rsp_value: models::ReplicationProtectedItemCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod recovery_points {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of recovery points for a replication protected item."]
        #[doc = "Lists the available recovery points for a replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The fabric name."]
        #[doc = "* `protection_container_name`: The protection container name."]
        #[doc = "* `replicated_protected_item_name`: The replication protected item name."]
        pub fn list_by_replication_protected_items(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> list_by_replication_protected_items::Builder {
            list_by_replication_protected_items::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
        #[doc = "Gets a recovery point."]
        #[doc = "Get the details of specified recovery point."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: The fabric name."]
        #[doc = "* `protection_container_name`: The protection container name."]
        #[doc = "* `replicated_protected_item_name`: The replication protected item name."]
        #[doc = "* `recovery_point_name`: The recovery point name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
            recovery_point_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
                recovery_point_name: recovery_point_name.into(),
            }
        }
    }
    pub mod list_by_replication_protected_items {
        use super::models;
        type Response = models::RecoveryPointCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/recoveryPoints" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPointCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RecoveryPoint;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
            pub(crate) recovery_point_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/recoveryPoints/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name , & this . recovery_point_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPoint = serde_json::from_slice(&rsp_body)?;
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
pub mod target_compute_sizes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of target compute sizes for the replication protected item."]
        #[doc = "Lists the available target compute sizes for a replication protected item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: protection container name."]
        #[doc = "* `replicated_protected_item_name`: Replication protected item name."]
        pub fn list_by_replication_protected_items(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            replicated_protected_item_name: impl Into<String>,
        ) -> list_by_replication_protected_items::Builder {
            list_by_replication_protected_items::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                replicated_protected_item_name: replicated_protected_item_name.into(),
            }
        }
    }
    pub mod list_by_replication_protected_items {
        use super::models;
        type Response = models::TargetComputeSizeCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) replicated_protected_item_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectedItems/{}/targetComputeSizes" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . replicated_protected_item_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TargetComputeSizeCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_protection_container_mappings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of protection container mappings for a protection container."]
        #[doc = "Lists the protection container mappings for a protection container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        pub fn list_by_replication_protection_containers(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
        ) -> list_by_replication_protection_containers::Builder {
            list_by_replication_protection_containers::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
            }
        }
        #[doc = "Gets a protection container mapping."]
        #[doc = "Gets the details of a protection container mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `mapping_name`: Protection Container mapping name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            mapping_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                mapping_name: mapping_name.into(),
            }
        }
        #[doc = "Create protection container mapping."]
        #[doc = "The operation to create a protection container mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `mapping_name`: Protection container mapping name."]
        #[doc = "* `creation_input`: Mapping creation input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            mapping_name: impl Into<String>,
            creation_input: impl Into<models::CreateProtectionContainerMappingInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                mapping_name: mapping_name.into(),
                creation_input: creation_input.into(),
            }
        }
        #[doc = "Update protection container mapping."]
        #[doc = "The operation to update protection container mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `mapping_name`: Protection container mapping name."]
        #[doc = "* `update_input`: Mapping update input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            mapping_name: impl Into<String>,
            update_input: impl Into<models::UpdateProtectionContainerMappingInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                mapping_name: mapping_name.into(),
                update_input: update_input.into(),
            }
        }
        #[doc = "Purge protection container mapping."]
        #[doc = "The operation to purge(force delete) a protection container mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `mapping_name`: Protection container mapping name."]
        pub fn purge(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            mapping_name: impl Into<String>,
        ) -> purge::Builder {
            purge::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                mapping_name: mapping_name.into(),
            }
        }
        #[doc = "Remove protection container mapping."]
        #[doc = "The operation to delete or remove a protection container mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `protection_container_name`: Protection container name."]
        #[doc = "* `mapping_name`: Protection container mapping name."]
        #[doc = "* `removal_input`: Removal input."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            protection_container_name: impl Into<String>,
            mapping_name: impl Into<String>,
            removal_input: impl Into<models::RemoveProtectionContainerMappingInput>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                protection_container_name: protection_container_name.into(),
                mapping_name: mapping_name.into(),
                removal_input: removal_input.into(),
            }
        }
        #[doc = "Gets the list of all protection container mappings in a vault."]
        #[doc = "Lists the protection container mappings in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_protection_containers {
        use super::models;
        type Response = models::ProtectionContainerMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerMappingCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProtectionContainerMapping;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) mapping_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerMapping = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProtectionContainerMapping),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) mapping_name: String,
            pub(crate) creation_input: models::CreateProtectionContainerMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.creation_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerMapping = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectionContainerMapping),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) mapping_name: String,
            pub(crate) update_input: models::UpdateProtectionContainerMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerMapping = serde_json::from_slice(&rsp_body)?;
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
    pub mod purge {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) mapping_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) protection_container_name: String,
            pub(crate) mapping_name: String,
            pub(crate) removal_input: models::RemoveProtectionContainerMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationProtectionContainers/{}/replicationProtectionContainerMappings/{}/remove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . protection_container_name , & this . mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.removal_input)?;
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
    pub mod list {
        use super::models;
        type Response = models::ProtectionContainerMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectionContainerMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerMappingCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_recovery_services_providers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of registered recovery services providers for the fabric."]
        #[doc = "Lists the registered recovery services providers for the specified fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets the details of a recovery services provider."]
        #[doc = "Gets the details of registered recovery services provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `provider_name`: Recovery services provider name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            provider_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                provider_name: provider_name.into(),
            }
        }
        #[doc = "Adds a recovery services provider."]
        #[doc = "The operation to add a recovery services provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `provider_name`: Recovery services provider name."]
        #[doc = "* `add_provider_input`: Add provider input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            provider_name: impl Into<String>,
            add_provider_input: impl Into<models::AddRecoveryServicesProviderInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                provider_name: provider_name.into(),
                add_provider_input: add_provider_input.into(),
            }
        }
        #[doc = "Purges recovery service provider from fabric."]
        #[doc = "The operation to purge(force delete) a recovery services provider from the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `provider_name`: Recovery services provider name."]
        pub fn purge(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            provider_name: impl Into<String>,
        ) -> purge::Builder {
            purge::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                provider_name: provider_name.into(),
            }
        }
        #[doc = "Refresh details from the recovery services provider."]
        #[doc = "The operation to refresh the information from the recovery services provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `provider_name`: Recovery services provider name."]
        pub fn refresh_provider(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            provider_name: impl Into<String>,
        ) -> refresh_provider::Builder {
            refresh_provider::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                provider_name: provider_name.into(),
            }
        }
        #[doc = "Deletes provider from fabric. Note: Deleting provider for any fabric other than SingleHost is unsupported. To maintain backward compatibility for released clients the object \"deleteRspInput\" is used (if the object is empty we assume that it is old client and continue the old behavior)."]
        #[doc = "The operation to removes/delete(unregister) a recovery services provider from the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `provider_name`: Recovery services provider name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            provider_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                provider_name: provider_name.into(),
            }
        }
        #[doc = "Gets the list of registered recovery services providers in the vault. This is a view only api."]
        #[doc = "Lists the registered recovery services providers in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::RecoveryServicesProviderCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryServicesProviderCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RecoveryServicesProvider;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) provider_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . provider_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryServicesProvider = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::RecoveryServicesProvider),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) provider_name: String,
            pub(crate) add_provider_input: models::AddRecoveryServicesProviderInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . provider_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.add_provider_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryServicesProvider = serde_json::from_slice(&rsp_body)?;
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
    pub mod purge {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) provider_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . provider_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod refresh_provider {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryServicesProvider),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) provider_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders/{}/refreshProvider" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . provider_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryServicesProvider = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) provider_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationRecoveryServicesProviders/{}/remove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . provider_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod list {
        use super::models;
        type Response = models::RecoveryServicesProviderCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryServicesProviders" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryServicesProviderCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_storage_classifications {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of storage classification objects under a fabric."]
        #[doc = "Lists the storage classifications available in the specified fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Site name of interest."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets the details of a storage classification."]
        #[doc = "Gets the details of the specified storage classification."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `storage_classification_name`: Storage classification name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            storage_classification_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                storage_classification_name: storage_classification_name.into(),
            }
        }
        #[doc = "Gets the list of storage classification objects under a vault."]
        #[doc = "Lists the storage classifications in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::StorageClassificationCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StorageClassification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) storage_classification_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . storage_classification_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassification = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StorageClassificationCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationStorageClassifications" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_storage_classification_mappings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of storage classification mappings objects under a storage."]
        #[doc = "Lists the storage classification mappings for the fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `storage_classification_name`: Storage classification name."]
        pub fn list_by_replication_storage_classifications(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            storage_classification_name: impl Into<String>,
        ) -> list_by_replication_storage_classifications::Builder {
            list_by_replication_storage_classifications::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                storage_classification_name: storage_classification_name.into(),
            }
        }
        #[doc = "Gets the details of a storage classification mapping."]
        #[doc = "Gets the details of the specified storage classification mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `storage_classification_name`: Storage classification name."]
        #[doc = "* `storage_classification_mapping_name`: Storage classification mapping name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            storage_classification_name: impl Into<String>,
            storage_classification_mapping_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                storage_classification_name: storage_classification_name.into(),
                storage_classification_mapping_name: storage_classification_mapping_name.into(),
            }
        }
        #[doc = "Create storage classification mapping."]
        #[doc = "The operation to create a storage classification mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `storage_classification_name`: Storage classification name."]
        #[doc = "* `storage_classification_mapping_name`: Storage classification mapping name."]
        #[doc = "* `pairing_input`: Pairing input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            storage_classification_name: impl Into<String>,
            storage_classification_mapping_name: impl Into<String>,
            pairing_input: impl Into<models::StorageClassificationMappingInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                storage_classification_name: storage_classification_name.into(),
                storage_classification_mapping_name: storage_classification_mapping_name.into(),
                pairing_input: pairing_input.into(),
            }
        }
        #[doc = "Delete a storage classification mapping."]
        #[doc = "The operation to delete a storage classification mapping."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `storage_classification_name`: Storage classification name."]
        #[doc = "* `storage_classification_mapping_name`: Storage classification mapping name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            storage_classification_name: impl Into<String>,
            storage_classification_mapping_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                storage_classification_name: storage_classification_name.into(),
                storage_classification_mapping_name: storage_classification_mapping_name.into(),
            }
        }
        #[doc = "Gets the list of storage classification mappings objects under a vault."]
        #[doc = "Lists the storage classification mappings in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_storage_classifications {
        use super::models;
        type Response = models::StorageClassificationMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) storage_classification_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications/{}/replicationStorageClassificationMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . storage_classification_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationMappingCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StorageClassificationMapping;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) storage_classification_name: String,
            pub(crate) storage_classification_mapping_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications/{}/replicationStorageClassificationMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . storage_classification_name , & this . storage_classification_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationMapping = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::StorageClassificationMapping),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) storage_classification_name: String,
            pub(crate) storage_classification_mapping_name: String,
            pub(crate) pairing_input: models::StorageClassificationMappingInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications/{}/replicationStorageClassificationMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . storage_classification_name , & this . storage_classification_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.pairing_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationMapping = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) storage_classification_name: String,
            pub(crate) storage_classification_mapping_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationStorageClassifications/{}/replicationStorageClassificationMappings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . storage_classification_name , & this . storage_classification_mapping_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod list {
        use super::models;
        type Response = models::StorageClassificationMappingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationStorageClassificationMappings" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageClassificationMappingCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replicationv_centers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of vCenter registered under a fabric."]
        #[doc = "Lists the vCenter servers registered in a fabric."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        pub fn list_by_replication_fabrics(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list_by_replication_fabrics::Builder {
            list_by_replication_fabrics::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
            }
        }
        #[doc = "Gets the details of a vCenter."]
        #[doc = "Gets the details of a registered vCenter server(Add vCenter server)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `vcenter_name`: vcenter name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            vcenter_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                vcenter_name: vcenter_name.into(),
            }
        }
        #[doc = "Add vCenter."]
        #[doc = "The operation to create a vCenter object.."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `vcenter_name`: vcenter name."]
        #[doc = "* `add_v_center_request`: The input to the add vCenter operation."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            vcenter_name: impl Into<String>,
            add_v_center_request: impl Into<models::AddVCenterRequest>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                vcenter_name: vcenter_name.into(),
                add_v_center_request: add_v_center_request.into(),
            }
        }
        #[doc = "Update vCenter operation."]
        #[doc = "The operation to update a registered vCenter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `vcenter_name`: vcenter name."]
        #[doc = "* `update_v_center_request`: The input to the update vCenter operation."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            vcenter_name: impl Into<String>,
            update_v_center_request: impl Into<models::UpdateVCenterRequest>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                vcenter_name: vcenter_name.into(),
                update_v_center_request: update_v_center_request.into(),
            }
        }
        #[doc = "Remove vcenter operation."]
        #[doc = "The operation to remove(unregister) a registered vCenter server from the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name."]
        #[doc = "* `vcenter_name`: vcenter name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            vcenter_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                vcenter_name: vcenter_name.into(),
            }
        }
        #[doc = "Gets the list of vCenter registered under the vault."]
        #[doc = "Lists the vCenter servers registered in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_by_replication_fabrics {
        use super::models;
        type Response = models::VCenterCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationvCenters" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VCenterCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VCenter;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) vcenter_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationvCenters/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . vcenter_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VCenter = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::VCenter),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) vcenter_name: String,
            pub(crate) add_v_center_request: models::AddVCenterRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationvCenters/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . vcenter_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.add_v_center_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VCenter = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::VCenter),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) vcenter_name: String,
            pub(crate) update_v_center_request: models::UpdateVCenterRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationvCenters/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . vcenter_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_v_center_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VCenter = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) vcenter_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationFabrics/{}/replicationvCenters/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . fabric_name , & this . vcenter_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod list {
        use super::models;
        type Response = models::VCenterCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationvCenters",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VCenterCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of jobs."]
        #[doc = "Gets the list of Azure Site Recovery Jobs for the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
        #[doc = "Gets the job details."]
        #[doc = "Get the details of an Azure Site Recovery job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Job identifier."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Cancels the specified job."]
        #[doc = "The operation to cancel an Azure Site Recovery job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Job identifier."]
        pub fn cancel(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Restarts the specified job."]
        #[doc = "The operation to restart an Azure Site Recovery job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Job identifier."]
        pub fn restart(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
        ) -> restart::Builder {
            restart::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
            }
        }
        #[doc = "Resumes the specified job."]
        #[doc = "The operation to resume an Azure Site Recovery job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Job identifier."]
        #[doc = "* `resume_job_params`: Resume rob comments."]
        pub fn resume(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
            resume_job_params: impl Into<models::ResumeJobParams>,
        ) -> resume::Builder {
            resume::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
                resume_job_params: resume_job_params.into(),
            }
        }
        #[doc = "Exports the details of the Azure Site Recovery jobs of the vault."]
        #[doc = "The operation to export the details of the Azure Site Recovery jobs of the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_query_parameter`: job query filter."]
        pub fn export(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_query_parameter: impl Into<models::JobQueryParameter>,
        ) -> export::Builder {
            export::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_query_parameter: job_query_parameter.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::JobCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                let rsp_value: models::JobCollection = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
            Ok200(models::Job),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs/{}/cancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . job_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Job = serde_json::from_slice(&rsp_body)?;
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
    pub mod restart {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Job),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs/{}/restart" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . job_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Job = serde_json::from_slice(&rsp_body)?;
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
    pub mod resume {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Job),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_name: String,
            pub(crate) resume_job_params: models::ResumeJobParams,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs/{}/resume" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . job_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.resume_job_params)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Job = serde_json::from_slice(&rsp_body)?;
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
    pub mod export {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Job),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_query_parameter: models::JobQueryParameter,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationJobs/export",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.job_query_parameter)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Job = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of replication policies."]
        #[doc = "Lists the replication policies for a vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the requested policy."]
        #[doc = "Gets the details of a replication policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Replication policy name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Creates the policy."]
        #[doc = "The operation to create a replication policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Replication policy name."]
        #[doc = "* `input`: Create policy input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
            input: impl Into<models::CreatePolicyInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Updates the policy."]
        #[doc = "The operation to update a replication policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Policy Id."]
        #[doc = "* `input`: Update Policy Input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
            input: impl Into<models::UpdatePolicyInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Delete the policy."]
        #[doc = "The operation to delete a replication policy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Replication policy name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PolicyCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PolicyCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Policy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Policy = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Policy),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
            pub(crate) input: models::CreatePolicyInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Policy = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Policy),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
            pub(crate) input: models::UpdatePolicyInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Policy = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
pub mod replication_protection_intents {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of replication protection intent objects."]
        #[doc = "Gets the list of ASR replication protection intent objects in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                skip_token: None,
                take_token: None,
            }
        }
        #[doc = "Gets the details of a Replication protection intent item."]
        #[doc = "Gets the details of an ASR replication protection intent."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `intent_object_name`: Replication protection intent name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            intent_object_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                intent_object_name: intent_object_name.into(),
            }
        }
        #[doc = "Create protection intent Resource."]
        #[doc = "The operation to create an ASR replication protection intent item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `intent_object_name`: A name for the replication protection item."]
        #[doc = "* `input`: Create Protection Intent Input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            intent_object_name: impl Into<String>,
            input: impl Into<models::CreateProtectionIntentInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                intent_object_name: intent_object_name.into(),
                input: input.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ReplicationProtectionIntentCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) take_token: Option<String>,
        }
        impl Builder {
            #[doc = "The pagination token."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "The page size."]
            pub fn take_token(mut self, take_token: impl Into<String>) -> Self {
                self.take_token = Some(take_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectionIntents" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(take_token) = &this.take_token {
                                    req.url_mut().query_pairs_mut().append_pair("takeToken", take_token);
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
                                let rsp_value: models::ReplicationProtectionIntentCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReplicationProtectionIntent;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) intent_object_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectionIntents/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . intent_object_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectionIntent = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReplicationProtectionIntent;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) intent_object_name: String,
            pub(crate) input: models::CreateProtectionIntentInput,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationProtectionIntents/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . intent_object_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationProtectionIntent = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_recovery_plans {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of recovery plans."]
        #[doc = "Lists the recovery plans in the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the requested recovery plan."]
        #[doc = "Gets the details of the recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Name of the recovery plan."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
            }
        }
        #[doc = "Creates a recovery plan with the given details."]
        #[doc = "The operation to create a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Recovery Plan creation input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::CreateRecoveryPlanInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Updates the given recovery plan."]
        #[doc = "The operation to update a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Update recovery plan input."]
        pub fn update(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::UpdateRecoveryPlanInput>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Deletes the specified recovery plan."]
        #[doc = "Delete a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        pub fn delete(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
            }
        }
        #[doc = "Execute cancel failover of the recovery plan."]
        #[doc = "The operation to cancel the failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        pub fn failover_cancel(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
        ) -> failover_cancel::Builder {
            failover_cancel::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
            }
        }
        #[doc = "Execute commit failover of the recovery plan."]
        #[doc = "The operation to commit the failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        pub fn failover_commit(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
        ) -> failover_commit::Builder {
            failover_commit::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
            }
        }
        #[doc = "Execute planned failover of the recovery plan."]
        #[doc = "The operation to start the planned failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Failover input."]
        pub fn planned_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::RecoveryPlanPlannedFailoverInput>,
        ) -> planned_failover::Builder {
            planned_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Execute reprotect of the recovery plan."]
        #[doc = "The operation to reprotect(reverse replicate) a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        pub fn reprotect(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
        ) -> reprotect::Builder {
            reprotect::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
            }
        }
        #[doc = "Execute test failover of the recovery plan."]
        #[doc = "The operation to start the test failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Recovery plan test failover input."]
        pub fn test_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::RecoveryPlanTestFailoverInput>,
        ) -> test_failover::Builder {
            test_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Execute test failover cleanup of the recovery plan."]
        #[doc = "The operation to cleanup test failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Recovery plan test failover cleanup input."]
        pub fn test_failover_cleanup(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::RecoveryPlanTestFailoverCleanupInput>,
        ) -> test_failover_cleanup::Builder {
            test_failover_cleanup::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
        #[doc = "Execute unplanned failover of the recovery plan."]
        #[doc = "The operation to start the unplanned failover of a recovery plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `recovery_plan_name`: Recovery plan name."]
        #[doc = "* `input`: Recovery plan unplanned failover input."]
        pub fn unplanned_failover(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            recovery_plan_name: impl Into<String>,
            input: impl Into<models::RecoveryPlanUnplannedFailoverInput>,
        ) -> unplanned_failover::Builder {
            unplanned_failover::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                recovery_plan_name: recovery_plan_name.into(),
                input: input.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::RecoveryPlanCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlanCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RecoveryPlan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::CreateRecoveryPlanInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::UpdateRecoveryPlanInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
    pub mod failover_cancel {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/failoverCancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod failover_commit {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/failoverCommit" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod planned_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::RecoveryPlanPlannedFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/plannedFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod reprotect {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/reProtect" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::RecoveryPlanTestFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/testFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod test_failover_cleanup {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::RecoveryPlanTestFailoverCleanupInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/testFailoverCleanup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
    pub mod unplanned_failover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::RecoveryPlan),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) recovery_plan_name: String,
            pub(crate) input: models::RecoveryPlanUnplannedFailoverInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationRecoveryPlans/{}/unplannedFailover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . recovery_plan_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPlan = serde_json::from_slice(&rsp_body)?;
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
pub mod supported_operating_systems {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the data of supported operating systems by SRS."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                instance_type: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::SupportedOperatingSystems;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) instance_type: Option<String>,
        }
        impl Builder {
            #[doc = "The instance type."]
            pub fn instance_type(mut self, instance_type: impl Into<String>) -> Self {
                self.instance_type = Some(instance_type.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationSupportedOperatingSystems" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        if let Some(instance_type) = &this.instance_type {
                            req.url_mut().query_pairs_mut().append_pair("instanceType", instance_type);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SupportedOperatingSystems = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_vault_health {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the health summary for the vault."]
        #[doc = "Gets the health details of the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Refreshes health summary of the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn refresh(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> refresh::Builder {
            refresh::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::VaultHealthDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationVaultHealth",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultHealthDetails = serde_json::from_slice(&rsp_body)?;
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
    pub mod refresh {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::VaultHealthDetails),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationVaultHealth/default/refresh" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultHealthDetails = serde_json::from_slice(&rsp_body)?;
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
pub mod replication_vault_setting {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of vault setting."]
        #[doc = "Gets the list of vault setting. This includes the Migration Hub connection settings."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the vault setting."]
        #[doc = "Gets the vault setting. This includes the Migration Hub connection settings."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `vault_setting_name`: Vault setting name."]
        pub fn get(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            vault_setting_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                vault_setting_name: vault_setting_name.into(),
            }
        }
        #[doc = "Updates vault setting. A vault setting object is a singleton per vault and it is always present by default."]
        #[doc = "The operation to configure vault setting."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `vault_setting_name`: Vault setting name."]
        #[doc = "* `input`: Vault setting creation input."]
        pub fn create(
            &self,
            resource_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            vault_setting_name: impl Into<String>,
            input: impl Into<models::VaultSettingCreationInput>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_name: resource_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                vault_setting_name: vault_setting_name.into(),
                input: input.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::VaultSettingCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationVaultSettings",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultSettingCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VaultSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) vault_setting_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationVaultSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . vault_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultSetting = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VaultSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) vault_setting_name: String,
            pub(crate) input: models::VaultSettingCreationInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/replicationVaultSettings/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_name , & this . vault_setting_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VaultSetting = serde_json::from_slice(&rsp_body)?;
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
