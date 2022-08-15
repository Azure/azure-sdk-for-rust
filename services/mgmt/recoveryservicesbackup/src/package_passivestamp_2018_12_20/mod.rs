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
    pub fn aad_properties_client(&self) -> aad_properties::Client {
        aad_properties::Client(self.clone())
    }
    pub fn backup_crr_job_details_client(&self) -> backup_crr_job_details::Client {
        backup_crr_job_details::Client(self.clone())
    }
    pub fn backup_crr_jobs_client(&self) -> backup_crr_jobs::Client {
        backup_crr_jobs::Client(self.clone())
    }
    pub fn backup_protected_items_crr_client(&self) -> backup_protected_items_crr::Client {
        backup_protected_items_crr::Client(self.clone())
    }
    pub fn backup_resource_storage_configs_client(&self) -> backup_resource_storage_configs::Client {
        backup_resource_storage_configs::Client(self.clone())
    }
    pub fn backup_usage_summaries_crr_client(&self) -> backup_usage_summaries_crr::Client {
        backup_usage_summaries_crr::Client(self.clone())
    }
    pub fn cross_region_restore_client(&self) -> cross_region_restore::Client {
        cross_region_restore::Client(self.clone())
    }
    pub fn crr_operation_results_client(&self) -> crr_operation_results::Client {
        crr_operation_results::Client(self.clone())
    }
    pub fn crr_operation_status_client(&self) -> crr_operation_status::Client {
        crr_operation_status::Client(self.clone())
    }
    pub fn recovery_points_client(&self) -> recovery_points::Client {
        recovery_points::Client(self.clone())
    }
    pub fn recovery_points_crr_client(&self) -> recovery_points_crr::Client {
        recovery_points_crr::Client(self.clone())
    }
}
pub mod backup_usage_summaries_crr {
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupUsageSummaries",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
pub mod aad_properties {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Fetches the AAD properties from target region BCM stamp."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(&self, azure_region: impl Into<String>, subscription_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::AadPropertiesResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
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
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupAadProperties",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                                let rsp_value: models::AadPropertiesResource = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod cross_region_restore {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Restores the specified backed up data in a different region as compared to where the data is backed up."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: resource cross region restore request"]
        pub fn trigger(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CrossRegionRestoreRequest>,
        ) -> trigger::Builder {
            trigger::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod trigger {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CrossRegionRestoreRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupCrossRegionRestore",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
pub mod backup_crr_job_details {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get CRR job details from target region."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: CRR Job request"]
        pub fn get(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CrrJobRequest>,
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
        type Response = models::JobResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CrrJobRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupCrrJob",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
pub mod backup_crr_jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of CRR jobs from the target region."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `parameters`: Backup CRR Job request"]
        pub fn list(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CrrJobRequest>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
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
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CrrJobRequest,
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
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupCrrJobs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
                                req.insert_header("content-type", "application/json");
                                let req_body = azure_core::to_json(&this.parameters)?;
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
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
pub mod crr_operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
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
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupCrrOperationResults/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
                        let req_body = azure_core::EMPTY_BODY;
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
pub mod crr_operation_status {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `azure_region`: Azure region to hit Api"]
        #[doc = "* `subscription_id`: The subscription Id."]
        pub fn get(
            &self,
            azure_region: impl Into<String>,
            subscription_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                azure_region: azure_region.into(),
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
            pub(crate) azure_region: String,
            pub(crate) subscription_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.RecoveryServices/locations/{}/backupCrrOperationsStatus/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.azure_region,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
pub mod recovery_points {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the Access token for communication between BMS and Protection service"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `vault_name`: The name of the recovery services vault."]
        #[doc = "* `resource_group_name`: The name of the resource group where the recovery services vault is present."]
        #[doc = "* `subscription_id`: The subscription Id."]
        #[doc = "* `fabric_name`: Fabric name associated with the container."]
        #[doc = "* `container_name`: Name of the container."]
        #[doc = "* `protected_item_name`: Name of the Protected Item."]
        #[doc = "* `recovery_point_id`: Recovery Point Id"]
        #[doc = "* `parameters`: Get Access Token request"]
        pub fn get_access_token(
            &self,
            vault_name: impl Into<String>,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
            fabric_name: impl Into<String>,
            container_name: impl Into<String>,
            protected_item_name: impl Into<String>,
            recovery_point_id: impl Into<String>,
            parameters: impl Into<models::AadPropertiesResource>,
        ) -> get_access_token::Builder {
            get_access_token::Builder {
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
    pub mod get_access_token {
        use super::models;
        type Response = models::CrrAccessTokenResource;
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
            pub(crate) parameters: models::AadPropertiesResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/accessToken" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name , & this . recovery_point_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CrrAccessTokenResource = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod backup_resource_storage_configs {
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupstorageconfig/vaultstorageconfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
pub mod recovery_points_crr {
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
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/{}/protectionContainers/{}/protectedItems/{}/recoveryPoints/" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . vault_name , & this . fabric_name , & this . container_name , & this . protected_item_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
}
pub mod backup_protected_items_crr {
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupProtectedItems/",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-12-20");
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
