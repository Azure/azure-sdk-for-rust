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
    pub fn backup_engines_client(&self) -> backup_engines::Client {
        backup_engines::Client(self.clone())
    }
    pub fn backup_jobs_client(&self) -> backup_jobs::Client {
        backup_jobs::Client(self.clone())
    }
    pub fn backup_operation_results_client(&self) -> backup_operation_results::Client {
        backup_operation_results::Client(self.clone())
    }
    pub fn backup_operation_statuses_client(&self) -> backup_operation_statuses::Client {
        backup_operation_statuses::Client(self.clone())
    }
    pub fn backup_policies_client(&self) -> backup_policies::Client {
        backup_policies::Client(self.clone())
    }
    pub fn backup_protectable_items_client(&self) -> backup_protectable_items::Client {
        backup_protectable_items::Client(self.clone())
    }
    pub fn backup_protected_items_client(&self) -> backup_protected_items::Client {
        backup_protected_items::Client(self.clone())
    }
    pub fn backup_protection_containers_client(&self) -> backup_protection_containers::Client {
        backup_protection_containers::Client(self.clone())
    }
    pub fn backup_protection_intent_client(&self) -> backup_protection_intent::Client {
        backup_protection_intent::Client(self.clone())
    }
    pub fn backup_resource_encryption_configs_client(&self) -> backup_resource_encryption_configs::Client {
        backup_resource_encryption_configs::Client(self.clone())
    }
    pub fn backup_resource_storage_configs_non_crr_client(&self) -> backup_resource_storage_configs_non_crr::Client {
        backup_resource_storage_configs_non_crr::Client(self.clone())
    }
    pub fn backup_resource_vault_configs_client(&self) -> backup_resource_vault_configs::Client {
        backup_resource_vault_configs::Client(self.clone())
    }
    pub fn backup_status_client(&self) -> backup_status::Client {
        backup_status::Client(self.clone())
    }
    pub fn backup_usage_summaries_client(&self) -> backup_usage_summaries::Client {
        backup_usage_summaries::Client(self.clone())
    }
    pub fn backup_workload_items_client(&self) -> backup_workload_items::Client {
        backup_workload_items::Client(self.clone())
    }
    pub fn backups_client(&self) -> backups::Client {
        backups::Client(self.clone())
    }
    pub fn bms_prepare_data_move_operation_result_client(&self) -> bms_prepare_data_move_operation_result::Client {
        bms_prepare_data_move_operation_result::Client(self.clone())
    }
    pub fn export_jobs_operation_results_client(&self) -> export_jobs_operation_results::Client {
        export_jobs_operation_results::Client(self.clone())
    }
    pub fn feature_support_client(&self) -> feature_support::Client {
        feature_support::Client(self.clone())
    }
    pub fn item_level_recovery_connections_client(&self) -> item_level_recovery_connections::Client {
        item_level_recovery_connections::Client(self.clone())
    }
    pub fn job_cancellations_client(&self) -> job_cancellations::Client {
        job_cancellations::Client(self.clone())
    }
    pub fn job_details_client(&self) -> job_details::Client {
        job_details::Client(self.clone())
    }
    pub fn job_operation_results_client(&self) -> job_operation_results::Client {
        job_operation_results::Client(self.clone())
    }
    pub fn jobs_client(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn operation_client(&self) -> operation::Client {
        operation::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn private_endpoint_client(&self) -> private_endpoint::Client {
        private_endpoint::Client(self.clone())
    }
    pub fn private_endpoint_connection_client(&self) -> private_endpoint_connection::Client {
        private_endpoint_connection::Client(self.clone())
    }
    pub fn protectable_containers_client(&self) -> protectable_containers::Client {
        protectable_containers::Client(self.clone())
    }
    pub fn protected_item_operation_results_client(&self) -> protected_item_operation_results::Client {
        protected_item_operation_results::Client(self.clone())
    }
    pub fn protected_item_operation_statuses_client(&self) -> protected_item_operation_statuses::Client {
        protected_item_operation_statuses::Client(self.clone())
    }
    pub fn protected_items_client(&self) -> protected_items::Client {
        protected_items::Client(self.clone())
    }
    pub fn protection_container_operation_results_client(&self) -> protection_container_operation_results::Client {
        protection_container_operation_results::Client(self.clone())
    }
    pub fn protection_container_refresh_operation_results_client(&self) -> protection_container_refresh_operation_results::Client {
        protection_container_refresh_operation_results::Client(self.clone())
    }
    pub fn protection_containers_client(&self) -> protection_containers::Client {
        protection_containers::Client(self.clone())
    }
    pub fn protection_intent_client(&self) -> protection_intent::Client {
        protection_intent::Client(self.clone())
    }
    pub fn protection_policies_client(&self) -> protection_policies::Client {
        protection_policies::Client(self.clone())
    }
    pub fn protection_policy_operation_results_client(&self) -> protection_policy_operation_results::Client {
        protection_policy_operation_results::Client(self.clone())
    }
    pub fn protection_policy_operation_statuses_client(&self) -> protection_policy_operation_statuses::Client {
        protection_policy_operation_statuses::Client(self.clone())
    }
    pub fn recovery_points_client(&self) -> recovery_points::Client {
        recovery_points::Client(self.clone())
    }
    pub fn recovery_points_recommended_for_move_client(&self) -> recovery_points_recommended_for_move::Client {
        recovery_points_recommended_for_move::Client(self.clone())
    }
    pub fn resource_guard_proxies_client(&self) -> resource_guard_proxies::Client {
        resource_guard_proxies::Client(self.clone())
    }
    pub fn resource_guard_proxy_client(&self) -> resource_guard_proxy::Client {
        resource_guard_proxy::Client(self.clone())
    }
    pub fn restores_client(&self) -> restores::Client {
        restores::Client(self.clone())
    }
    pub fn security_pi_ns_client(&self) -> security_pi_ns::Client {
        security_pi_ns::Client(self.clone())
    }
    pub fn validate_operation_client(&self) -> validate_operation::Client {
        validate_operation::Client(self.clone())
    }
    pub fn validate_operation_results_client(&self) -> validate_operation_results::Client {
        validate_operation_results::Client(self.clone())
    }
    pub fn validate_operation_statuses_client(&self) -> validate_operation_statuses::Client {
        validate_operation_statuses::Client(self.clone())
    }
}
pub mod backup_resource_storage_configs_non_crr {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches resource storage config."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates vault storage model type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Vault storage config request"]
        pub fn update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupResourceConfigResource>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Updates vault storage model type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Vault storage config request"]
        pub fn patch(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupResourceConfigResource>,
        ) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BackupResourceConfigResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                                let rsp_value: models::BackupResourceConfigResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupResourceConfigResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupResourceConfigResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupResourceConfigResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupResourceConfigResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
pub mod protection_intent {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "It will validate followings\r\n1. Vault capacity\r\n2. VM is already protected\r\n3. Any VM related configuration passed in properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Enable backup validation request on Virtual Machine"]
        pub fn validate(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::PreValidateEnableBackupRequest>,
        ) -> validate::Builder {
            validate::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Provides the details of the protection intent up item. This is an asynchronous operation. To know the status of the operation,\r\ncall the GetItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up item."]
        #[doc = "* `intent_object_name`: Backed up item name whose details are to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            intent_object_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                intent_object_name: intent_object_name.into(),
            }
        }
        #[doc = "Create Intent for Enabling backup of an item. This is a synchronous operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backup item."]
        #[doc = "* `intent_object_name`: Intent object name."]
        #[doc = "* `parameters`: resource backed up item"]
        pub fn create_or_update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            intent_object_name: impl Into<String>,
            parameters: impl Into<models::ProtectionIntentResource>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                intent_object_name: intent_object_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Used to remove intent from an item"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the intent."]
        #[doc = "* `intent_object_name`: Intent to be deleted."]
        pub fn delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            intent_object_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                intent_object_name: intent_object_name.into(),
            }
        }
    }
    pub mod validate {
        use super::models;
        type Response = models::PreValidateEnableBackupResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::PreValidateEnableBackupRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/Subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupPreValidateProtection",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PreValidateEnableBackupResponse = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProtectionIntentResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) intent_object_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/backupProtectionIntent/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . intent_object_name)) ? ;
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
                                let rsp_value: models::ProtectionIntentResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProtectionIntentResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) intent_object_name: String,
            pub(crate) parameters: models::ProtectionIntentResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/backupProtectionIntent/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . intent_object_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionIntentResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) intent_object_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/backupProtectionIntent/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . intent_object_name)) ? ;
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
pub mod backup_status {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the container backup status"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Container Backup Status Request"]
        pub fn get(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupStatusRequest>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BackupStatusResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupStatusRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/Subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupStatus",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupStatusResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod feature_support {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "It will validate if given feature with resource properties is supported in service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Feature support request object"]
        pub fn validate(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::FeatureSupportRequest>,
        ) -> validate::Builder {
            validate::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod validate {
        use super::models;
        type Response = models::AzureVmResourceFeatureSupportResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::FeatureSupportRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/Subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupValidateFeatures",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureVmResourceFeatureSupportResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_protection_intent {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a pageable list of all intents that are present within a vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProtectionIntentResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupProtectionIntents",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::ProtectionIntentResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_usage_summaries {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the backup management usage summaries of the vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BackupManagementUsageList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/Subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupUsageSummaries",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        if let Some(skip_token) = &this.skip_token {
                            req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupManagementUsageList = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Returns the list of available operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ClientDiscoveryResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.RecoveryServices/operations",
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
                                let rsp_value: models::ClientDiscoveryResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_resource_vault_configs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches resource vault config."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates vault security config. "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: resource config request"]
        pub fn put(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupResourceVaultConfigResource>,
        ) -> put::Builder {
            put::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Updates vault security config."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: resource config request"]
        pub fn update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupResourceVaultConfigResource>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BackupResourceVaultConfigResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupconfig/vaultconfig",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::BackupResourceVaultConfigResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod put {
        use super::models;
        type Response = models::BackupResourceVaultConfigResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupResourceVaultConfigResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupconfig/vaultconfig",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupResourceVaultConfigResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupResourceVaultConfigResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupResourceVaultConfigResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupconfig/vaultconfig",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupResourceVaultConfigResource = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_resource_encryption_configs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches Vault Encryption config."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Updates Vault encryption config."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Vault encryption input config request"]
        pub fn update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::BackupResourceEncryptionConfigResource>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BackupResourceEncryptionConfigExtendedResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupEncryptionConfigs/backupResourceEncryptionConfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                                let rsp_value: models::BackupResourceEncryptionConfigExtendedResource = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::BackupResourceEncryptionConfigResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupEncryptionConfigs/backupResourceEncryptionConfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod private_endpoint_connection {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get Private Endpoint Connection. This call is made by Backup Admin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
            }
        }
        #[doc = "Approve or Reject Private Endpoint requests. This call is made by Backup Admin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        #[doc = "* `parameters`: Request body for operation"]
        pub fn put(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
            parameters: impl Into<models::PrivateEndpointConnectionResource>,
        ) -> put::Builder {
            put::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete Private Endpoint requests. This call is made by Backup Admin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        pub fn delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::PrivateEndpointConnectionResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) private_endpoint_connection_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . private_endpoint_connection_name)) ? ;
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
                                let rsp_value: models::PrivateEndpointConnectionResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod put {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::PrivateEndpointConnectionResource),
            Created201(models::PrivateEndpointConnectionResource),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) private_endpoint_connection_name: String,
            pub(crate) parameters: models::PrivateEndpointConnectionResource,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . private_endpoint_connection_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnectionResource = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnectionResource = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) private_endpoint_connection_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . private_endpoint_connection_name)) ? ;
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
pub mod private_endpoint {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the operation status for a private endpoint connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        #[doc = "* `operation_id`: Operation id"]
        pub fn get_operation_status(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get_operation_status::Builder {
            get_operation_status::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get_operation_status {
        use super::models;
        type Response = models::OperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) private_endpoint_connection_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/privateEndpointConnections/{}/operationsStatus/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . private_endpoint_connection_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
impl Client {
    #[doc = "Fetches operation status for data move operation on vault"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `vault_name`: The name of the recovery services vault."]
    #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
    #[doc = "* `subscription_id`: The subscription Id."]
    pub fn get_operation_status(
        &self,
        vault_name: impl Into<String>,
        resource_group_name: impl Into<String>,
        subscription_id: impl Into<String>,
        operation_id: impl Into<String>,
    ) -> get_operation_status::Builder {
        get_operation_status::Builder {
            client: self.clone(),
            vault_name: vault_name.into(),
            resource_group_name: resource_group_name.into(),
            subscription_id: subscription_id.into(),
            operation_id: operation_id.into(),
        }
    }
    #[doc = "Prepares source vault for Data Move operation"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `vault_name`: The name of the recovery services vault."]
    #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
    #[doc = "* `subscription_id`: The subscription Id."]
    #[doc = "* `parameters`: Prepare data move request"]
    pub fn bms_prepare_data_move(
        &self,
        vault_name: impl Into<String>,
        resource_group_name: impl Into<String>,
        subscription_id: impl Into<String>,
        parameters: impl Into<models::PrepareDataMoveRequest>,
    ) -> bms_prepare_data_move::Builder {
        bms_prepare_data_move::Builder {
            client: self.clone(),
            vault_name: vault_name.into(),
            resource_group_name: resource_group_name.into(),
            subscription_id: subscription_id.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Triggers Data Move Operation on target vault"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `vault_name`: The name of the recovery services vault."]
    #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
    #[doc = "* `subscription_id`: The subscription Id."]
    #[doc = "* `parameters`: Trigger data move request"]
    pub fn bms_trigger_data_move(
        &self,
        vault_name: impl Into<String>,
        resource_group_name: impl Into<String>,
        subscription_id: impl Into<String>,
        parameters: impl Into<models::TriggerDataMoveRequest>,
    ) -> bms_trigger_data_move::Builder {
        bms_trigger_data_move::Builder {
            client: self.clone(),
            vault_name: vault_name.into(),
            resource_group_name: resource_group_name.into(),
            subscription_id: subscription_id.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Move recovery point from one datastore to another store."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `vault_name`: The name of the recovery services vault."]
    #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
    #[doc = "* `subscription_id`: The subscription Id."]
    #[doc = "* `parameters`: Move Resource Across Tiers Request"]
    pub fn move_recovery_point(
        &self,
        vault_name: impl Into<String>,
        resource_group_name: impl Into<String>,
        subscription_id: impl Into<String>,
        fabric_name: impl Into<String>,
        container_name: impl Into<String>,
        protected_item_name: impl Into<String>,
        recovery_point_id: impl Into<String>,
        parameters: impl Into<models::MoveRpAcrossTiersRequest>,
    ) -> move_recovery_point::Builder {
        move_recovery_point::Builder {
            client: self.clone(),
            vault_name: vault_name.into(),
            resource_group_name: resource_group_name.into(),
            subscription_id: subscription_id.into(),
            fabric_name: fabric_name.into(),
            container_name: container_name.into(),
            protected_item_name: protected_item_name.into(),
            recovery_point_id: recovery_point_id.into(),
            parameters: parameters.into(),
        }
    }
}
pub mod get_operation_status {
    use super::models;
    type Response = models::OperationStatus;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) vault_name: String,
        pub(crate) resource_group_name: String,
        pub(crate) subscription_id: String,
        pub(crate) operation_id: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig/operationStatus/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
                            let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod bms_prepare_data_move {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        Accepted202,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) vault_name: String,
        pub(crate) resource_group_name: String,
        pub(crate) subscription_id: String,
        pub(crate) parameters: models::PrepareDataMoveRequest,
    }
    impl Builder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig/prepareDataMove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
pub mod bms_trigger_data_move {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        Accepted202,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) vault_name: String,
        pub(crate) resource_group_name: String,
        pub(crate) subscription_id: String,
        pub(crate) parameters: models::TriggerDataMoveRequest,
    }
    impl Builder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig/triggerDataMove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
pub mod move_recovery_point {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) vault_name: String,
        pub(crate) resource_group_name: String,
        pub(crate) subscription_id: String,
        pub(crate) fabric_name: String,
        pub(crate) container_name: String,
        pub(crate) protected_item_name: String,
        pub(crate) recovery_point_id: String,
        pub(crate) parameters: models::MoveRpAcrossTiersRequest,
    }
    impl Builder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/move" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
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
                    let req_body = azure_core::to_json(&this.parameters)?;
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
pub mod bms_prepare_data_move_operation_result {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches Operation Result for Prepare Data Move"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::VaultStorageConfigOperationResultResponse),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::VaultStorageConfigOperationResultResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod protected_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the details of the backed up item. This is an asynchronous operation. To know the status of the operation,\r\ncall the GetItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up item."]
        #[doc = "* `container_name`: Container name associated with the backed up item."]
        #[doc = "* `protected_item_name`: Backed up item name whose details are to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                filter: None,
            }
        }
        #[doc = "Enables backup of an item or to modifies the backup policy information of an already backed up item. This is an\r\nasynchronous operation. To know the status of the operation, call the GetItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backup item."]
        #[doc = "* `container_name`: Container name associated with the backup item."]
        #[doc = "* `protected_item_name`: Item name to be backed up."]
        #[doc = "* `parameters`: resource backed up item"]
        pub fn create_or_update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            parameters: impl Into<models::ProtectedItemResource>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Used to disable backup of an item within a container. This is an asynchronous operation. To know the status of the\r\nrequest, call the GetItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up item."]
        #[doc = "* `container_name`: Container name associated with the backed up item."]
        #[doc = "* `protected_item_name`: Backed up item to be deleted."]
        pub fn delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProtectedItemResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                                let rsp_value: models::ProtectedItemResource = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProtectedItemResource),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) parameters: models::ProtectedItemResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectedItemResource = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
pub mod protected_item_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the result of any operation on the backup item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backup item."]
        #[doc = "* `container_name`: Container name associated with the backup item."]
        #[doc = "* `protected_item_name`: Backup item name whose details are to be fetched."]
        #[doc = "* `operation_id`: OperationID which represents the operation whose result needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectedItemResource),
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::ProtectedItemResource = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
pub mod recovery_points {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the backup copies for the backed up item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up item."]
        #[doc = "* `container_name`: Container name associated with the backed up item."]
        #[doc = "* `protected_item_name`: Backed up item whose backup copies are to be fetched."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                filter: None,
            }
        }
        #[doc = "Provides the information of the backed up data identified using RecoveryPointID. This is an asynchronous operation.\r\nTo know the status of the operation, call the GetProtectedItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with backed up item."]
        #[doc = "* `container_name`: Container name associated with backed up item."]
        #[doc = "* `protected_item_name`: Backed up item name whose backup data needs to be fetched."]
        #[doc = "* `recovery_point_id`: RecoveryPointID represents the backed up data to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            recovery_point_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                recovery_point_id: recovery_point_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::RecoveryPointResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
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
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                                let rsp_value: models::RecoveryPointResourceList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RecoveryPointResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) recovery_point_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
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
                                let rsp_value: models::RecoveryPointResource = serde_json::from_slice(&rsp_body)?;
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
pub mod restores {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Restores the specified backed up data. This is an asynchronous operation. To know the status of this API call, use\r\nGetProtectedItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up items."]
        #[doc = "* `container_name`: Container name associated with the backed up items."]
        #[doc = "* `protected_item_name`: Backed up item to be restored."]
        #[doc = "* `recovery_point_id`: Recovery point ID which represents the backed up data to be restored."]
        #[doc = "* `parameters`: resource restore request"]
        pub fn trigger(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            recovery_point_id: impl Into<String>,
            parameters: impl Into<models::RestoreRequestResource>,
        ) -> trigger::Builder {
            trigger::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                recovery_point_id: recovery_point_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod trigger {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) recovery_point_id: String,
            pub(crate) parameters: models::RestoreRequestResource,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/restore" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
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
}
pub mod backup_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists of backup policies associated with Recovery Services Vault. API provides pagination parameters to fetch\r\nscoped results."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProtectionPolicyResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::ProtectionPolicyResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod protection_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the details of the backup policies associated to Recovery Services Vault. This is an asynchronous\r\noperation. Status of the operation can be fetched using GetPolicyOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Backup policy information to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Creates or modifies a backup policy. This is an asynchronous operation. Status of the operation can be fetched\r\nusing GetPolicyOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Backup policy to be created."]
        #[doc = "* `parameters`: resource backup policy"]
        pub fn create_or_update(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
            parameters: impl Into<models::ProtectionPolicyResource>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes specified backup policy from your Recovery Services Vault. This is an asynchronous operation. Status of the\r\noperation can be fetched using GetProtectionPolicyOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Backup policy to be deleted."]
        pub fn delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProtectionPolicyResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
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
                                let rsp_value: models::ProtectionPolicyResource = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProtectionPolicyResource),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
            pub(crate) parameters: models::ProtectionPolicyResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionPolicyResource = serde_json::from_slice(&rsp_body)?;
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
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
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
pub mod protection_policy_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the result of an operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Backup policy name whose operation's result needs to be fetched."]
        #[doc = "* `operation_id`: Operation ID which represents the operation whose result needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProtectionPolicyResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies/{}/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . policy_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::ProtectionPolicyResource = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a pageable list of jobs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::JobResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::JobResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod job_details {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets extended information associated with the job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Name of the job whose details are to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::JobResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
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
                                let rsp_value: models::JobResource = serde_json::from_slice(&rsp_body)?;
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
pub mod job_cancellations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Cancels a job. This is an asynchronous operation. To know the status of the cancellation, call\r\nGetCancelOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Name of the job to cancel."]
        pub fn trigger(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
        ) -> trigger::Builder {
            trigger::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
            }
        }
    }
    pub mod trigger {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobs/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
}
pub mod job_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the result of any operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `job_name`: Job name whose operation result has to be fetched."]
        #[doc = "* `operation_id`: OperationID which represents the operation whose result has to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            job_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                job_name: job_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) job_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobs/{}/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . job_name , & this . operation_id)) ? ;
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
pub mod export_jobs_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the operation result of operation triggered by Export Jobs API. If the operation is successful, then it also\r\ncontains URL of a Blob and a SAS key to access the same. The blob contains exported jobs in JSON serialized format."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `operation_id`: OperationID which represents the export job."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::OperationResultInfoBaseResource),
            Accepted202(models::OperationResultInfoBaseResource),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobs/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::OperationResultInfoBaseResource = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationResultInfoBaseResource = serde_json::from_slice(&rsp_body)?;
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
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Triggers export of jobs specified by filters and returns an OperationID to track."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn export(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> export::Builder {
            export::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod export {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupJobsExport",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
}
pub mod backup_protected_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a pageable list of all items that are backed up within a vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProtectedItemResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupProtectedItems",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::ProtectedItemResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod operation {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Validate operation for specified backed up item. This is a synchronous operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: resource validate operation request"]
        pub fn validate(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::ValidateOperationRequest>,
        ) -> validate::Builder {
            validate::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod validate {
        use super::models;
        type Response = models::ValidateOperationsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::ValidateOperationRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupValidateOperation",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ValidateOperationsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod validate_operation {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Validate operation for specified backed up item in the form of an asynchronous operation. Returns tracking headers which can be tracked using GetValidateOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: resource validate operation request"]
        pub fn trigger(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::ValidateOperationRequest>,
        ) -> trigger::Builder {
            trigger::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod trigger {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::ValidateOperationRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupTriggerValidateOperation" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
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
}
pub mod validate_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the result of a triggered validate operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `operation_id`: OperationID which represents the operation whose result needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ValidateOperationsResponse),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupValidateOperationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::ValidateOperationsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod validate_operation_statuses {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the status of a triggered validate operation. The status can be in progress, completed\r\nor failed. You can refer to the OperationStatus enum for all the possible states of the operation.\r\nIf operation has completed, this method returns the list of errors obtained while validating the operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `operation_id`: OperationID represents the operation whose status needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::OperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupValidateOperationsStatuses/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_engines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Backup management servers registered to Recovery Services Vault. Returns a pageable list of servers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
        #[doc = "Returns backup management server registered to Recovery Services Vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `backup_engine_name`: Name of the backup management server."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            backup_engine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                backup_engine_name: backup_engine_name.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BackupEngineBaseResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupEngines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::BackupEngineBaseResourceList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::BackupEngineBaseResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) backup_engine_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupEngines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
                            &this.backup_engine_name
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
                        if let Some(skip_token) = &this.skip_token {
                            req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BackupEngineBaseResource = serde_json::from_slice(&rsp_body)?;
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
pub mod protection_container_refresh_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the result of the refresh operation triggered by the BeginRefresh operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the container."]
        #[doc = "* `operation_id`: Operation ID associated with the operation whose result needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . operation_id)) ? ;
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
pub mod protectable_containers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the containers that can be registered to Recovery Services Vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProtectableContainerResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectableContainers" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name)) ? ;
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
                                let rsp_value: models::ProtectableContainerResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod protection_containers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets details of the specific container registered to your Recovery Services Vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Name of the fabric where the container belongs."]
        #[doc = "* `container_name`: Name of the container whose details need to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
            }
        }
        #[doc = "Registers the container with Recovery Services vault.\r\nThis is an asynchronous operation. To track the operation status, use location header to call get latest status of\r\nthe operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the container."]
        #[doc = "* `container_name`: Name of the container to be registered."]
        #[doc = "* `parameters`: Request body for operation"]
        pub fn register(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            parameters: impl Into<models::ProtectionContainerResource>,
        ) -> register::Builder {
            register::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Unregisters the given container from your Recovery Services Vault. This is an asynchronous operation. To determine\r\nwhether the backend service has finished processing the request, call Get Container Operation Result API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Name of the fabric where the container belongs."]
        #[doc = "* `container_name`: Name of the container which needs to be unregistered from the Recovery Services Vault."]
        pub fn unregister(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
        ) -> unregister::Builder {
            unregister::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
            }
        }
        #[doc = "Inquires all the protectable items under the given container."]
        #[doc = "This is an async operation and the results should be tracked using location header or Azure-async-url."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric Name associated with the container."]
        #[doc = "* `container_name`: Name of the container in which inquiry needs to be triggered."]
        pub fn inquire(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
        ) -> inquire::Builder {
            inquire::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                filter: None,
            }
        }
        #[doc = "Discovers all the containers in the subscription that can be backed up to Recovery Services Vault. This is an\r\nasynchronous operation. To know the status of the operation, call GetRefreshOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated the container."]
        pub fn refresh(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
        ) -> refresh::Builder {
            refresh::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                filter: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProtectionContainerResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name)) ? ;
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
                                let rsp_value: models::ProtectionContainerResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod register {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectionContainerResource),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) parameters: models::ProtectionContainerResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProtectionContainerResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod unregister {
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name)) ? ;
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
    pub mod inquire {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/inquire" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name)) ? ;
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
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod refresh {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/refreshContainers" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name)) ? ;
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
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
}
pub mod backup_workload_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a pageable list of workload item of a specific container according to the query filter and the pagination\r\nparameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the container."]
        #[doc = "* `container_name`: Name of the container."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkloadItemResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/items" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name)) ? ;
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
                                let rsp_value: models::WorkloadItemResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod protection_container_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the result of any operation on the container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the container."]
        #[doc = "* `container_name`: Container name whose information should be fetched."]
        #[doc = "* `operation_id`: Operation ID which represents the operation whose result needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProtectionContainerResource),
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/operationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::ProtectionContainerResource = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
        #[doc = "Triggers backup for specified backed up item. This is an asynchronous operation. To know the status of the\r\noperation, call GetProtectedItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backup item."]
        #[doc = "* `container_name`: Container name associated with the backup item."]
        #[doc = "* `protected_item_name`: Backup item for which backup needs to be triggered."]
        #[doc = "* `parameters`: resource backup request"]
        pub fn trigger(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            parameters: impl Into<models::BackupRequestResource>,
        ) -> trigger::Builder {
            trigger::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod trigger {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) parameters: models::BackupRequestResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/backup" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
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
}
pub mod protected_item_operation_statuses {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the status of an operation such as triggering a backup, restore. The status can be in progress, completed\r\nor failed. You can refer to the OperationStatus enum for all the possible states of the operation. Some operations\r\ncreate jobs. This method returns the list of jobs associated with the operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backup item."]
        #[doc = "* `container_name`: Container name associated with the backup item."]
        #[doc = "* `protected_item_name`: Backup item name whose details are to be fetched."]
        #[doc = "* `operation_id`: OperationID represents the operation whose status needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::OperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/operationsStatus/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod item_level_recovery_connections {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provisions a script which invokes an iSCSI connection to the backup data. Executing this script opens a file\r\nexplorer displaying all the recoverable files and folders. This is an asynchronous operation. To know the status of\r\nprovisioning, call GetProtectedItemOperationResult API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up items."]
        #[doc = "* `container_name`: Container name associated with the backed up items."]
        #[doc = "* `protected_item_name`: Backed up item name whose files/folders are to be restored."]
        #[doc = "* `recovery_point_id`: Recovery point ID which represents backed up data. iSCSI connection will be provisioned\r\nfor this backed up data."]
        #[doc = "* `parameters`: resource ILR request"]
        pub fn provision(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            recovery_point_id: impl Into<String>,
            parameters: impl Into<models::IlrRequestResource>,
        ) -> provision::Builder {
            provision::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                recovery_point_id: recovery_point_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Revokes an iSCSI connection which can be used to download a script. Executing this script opens a file explorer\r\ndisplaying all recoverable files and folders. This is an asynchronous operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the backed up items."]
        #[doc = "* `container_name`: Container name associated with the backed up items."]
        #[doc = "* `protected_item_name`: Backed up item name whose files/folders are to be restored."]
        #[doc = "* `recovery_point_id`: Recovery point ID which represents backed up data. iSCSI connection will be revoked for\r\nthis backed up data."]
        pub fn revoke(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            recovery_point_id: impl Into<String>,
        ) -> revoke::Builder {
            revoke::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                recovery_point_id: recovery_point_id.into(),
            }
        }
    }
    pub mod provision {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) recovery_point_id: String,
            pub(crate) parameters: models::IlrRequestResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/provisionInstantItemRecovery" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod revoke {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) recovery_point_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/revokeInstantItemRecovery" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
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
}
pub mod backup_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the status of the delete operations such as deleting backed up item. Once the operation has started, the\r\nstatus code in the response would be Accepted. It will continue to be in this state till it reaches completion. On\r\nsuccessful completion, the status code will be OK. This method expects OperationID as an argument. OperationID is\r\npart of the Location header of the operation response."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `operation_id`: OperationID which represents the operation."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupOperationResults/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . operation_id)) ? ;
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
pub mod backup_operation_statuses {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the status of an operation such as triggering a backup, restore. The status can be in progress, completed\r\nor failed. You can refer to the OperationStatus enum for all the possible states of an operation. Some operations\r\ncreate jobs. This method returns the list of jobs when the operation is complete."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `operation_id`: OperationID which represents the operation."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::OperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupOperations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name,
                            &this.operation_id
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
                                let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod protection_policy_operation_statuses {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides the status of the asynchronous operations like backup, restore. The status can be in progress, completed\r\nor failed. You can refer to the Operation Status enum for all the possible states of an operation. Some operations\r\ncreate jobs. This method returns the list of jobs associated with operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `policy_name`: Backup policy name whose operation's status needs to be fetched."]
        #[doc = "* `operation_id`: Operation ID which represents an operation whose status needs to be fetched."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            policy_name: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                policy_name: policy_name.into(),
                operation_id: operation_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::OperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) policy_name: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupPolicies/{}/operations/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . policy_name , & this . operation_id)) ? ;
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
                                let rsp_value: models::OperationStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_protectable_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Provides a pageable list of protectable objects within your subscription according to the query filter and the\r\npagination parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkloadProtectableItemResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "OData filter options."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "skipToken Filter."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupProtectableItems",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                                let rsp_value: models::WorkloadProtectableItemResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod backup_protection_containers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the containers registered to Recovery Services Vault."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProtectionContainerResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
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
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupProtectionContainers" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                                let rsp_value: models::ProtectionContainerResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod security_pi_ns {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the security PIN."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                parameters: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TokenInformation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: Option<models::SecurityPinBase>,
        }
        impl Builder {
            #[doc = "security pin request"]
            pub fn parameters(mut self, parameters: impl Into<models::SecurityPinBase>) -> Self {
                self.parameters = Some(parameters.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupSecurityPIN",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.vault_name
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
                        let req_body = if let Some(parameters) = &this.parameters {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(parameters)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TokenInformation = serde_json::from_slice(&rsp_body)?;
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
pub mod recovery_points_recommended_for_move {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the recovery points recommended for move to another tier"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: List Recovery points Recommended for Move Request"]
        pub fn list(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            parameters: impl Into<models::ListRecoveryPointsRecommendedForMoveRequest>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                fabric_name: fabric_name.into(),
                container_name: container_name.into(),
                protected_item_name: protected_item_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::RecoveryPointResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) fabric_name: String,
            pub(crate) container_name: String,
            pub(crate) protected_item_name: String,
            pub(crate) parameters: models::ListRecoveryPointsRecommendedForMoveRequest,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPointsRecommendedForMove" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-02-01");
                                req.insert_header("content-type", "application/json");
                                let req_body = azure_core::to_json(&this.parameters)?;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecoveryPointResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod resource_guard_proxies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the ResourceGuardProxies under vault"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ResourceGuardProxyBaseResourceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupResourceGuardProxies" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
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
                                let rsp_value: models::ResourceGuardProxyBaseResourceList = serde_json::from_slice(&rsp_body)?;
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
pub mod resource_guard_proxy {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns ResourceGuardProxy under vault and with the name referenced in request"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_guard_proxy_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_guard_proxy_name: resource_guard_proxy_name.into(),
            }
        }
        #[doc = "Add or Update ResourceGuardProxy under vault\r\nSecures vault critical operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Request body for operation"]
        pub fn put(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_guard_proxy_name: impl Into<String>,
            parameters: impl Into<models::ResourceGuardProxyBaseResource>,
        ) -> put::Builder {
            put::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_guard_proxy_name: resource_guard_proxy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete ResourceGuardProxy under vault"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_guard_proxy_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_guard_proxy_name: resource_guard_proxy_name.into(),
            }
        }
        #[doc = "Secures delete ResourceGuardProxy operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Request body for operation"]
        pub fn unlock_delete(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            resource_guard_proxy_name: impl Into<String>,
            parameters: impl Into<models::UnlockDeleteRequest>,
        ) -> unlock_delete::Builder {
            unlock_delete::Builder {
                client: self.0.clone(),
                vault_name: vault_name.into(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
                resource_guard_proxy_name: resource_guard_proxy_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ResourceGuardProxyBaseResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_guard_proxy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupResourceGuardProxies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . resource_guard_proxy_name)) ? ;
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
                                let rsp_value: models::ResourceGuardProxyBaseResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod put {
        use super::models;
        type Response = models::ResourceGuardProxyBaseResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_guard_proxy_name: String,
            pub(crate) parameters: models::ResourceGuardProxyBaseResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupResourceGuardProxies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . resource_guard_proxy_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceGuardProxyBaseResource = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_guard_proxy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupResourceGuardProxies/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . resource_guard_proxy_name)) ? ;
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
    pub mod unlock_delete {
        use super::models;
        type Response = models::UnlockDeleteResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) vault_name: String,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
            pub(crate) resource_guard_proxy_name: String,
            pub(crate) parameters: models::UnlockDeleteRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupResourceGuardProxies/{}/unlockDelete" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . resource_guard_proxy_name)) ? ;
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
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UnlockDeleteResponse = serde_json::from_slice(&rsp_body)?;
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
