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
    pub fn ad_domain_service_members_client(&self) -> ad_domain_service_members::Client {
        ad_domain_service_members::Client(self.clone())
    }
    pub fn adds_service_client(&self) -> adds_service::Client {
        adds_service::Client(self.clone())
    }
    pub fn adds_service_members_client(&self) -> adds_service_members::Client {
        adds_service_members::Client(self.clone())
    }
    pub fn adds_services_client(&self) -> adds_services::Client {
        adds_services::Client(self.clone())
    }
    pub fn adds_services_replication_status_client(&self) -> adds_services_replication_status::Client {
        adds_services_replication_status::Client(self.clone())
    }
    pub fn adds_services_service_members_client(&self) -> adds_services_service_members::Client {
        adds_services_service_members::Client(self.clone())
    }
    pub fn adds_services_user_preference_client(&self) -> adds_services_user_preference::Client {
        adds_services_user_preference::Client(self.clone())
    }
    pub fn alerts_client(&self) -> alerts::Client {
        alerts::Client(self.clone())
    }
    pub fn configuration_client(&self) -> configuration::Client {
        configuration::Client(self.clone())
    }
    pub fn dimensions_client(&self) -> dimensions::Client {
        dimensions::Client(self.clone())
    }
    pub fn list_client(&self) -> list::Client {
        list::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn reports_client(&self) -> reports::Client {
        reports::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
    pub fn service_members_client(&self) -> service_members::Client {
        service_members::Client(self.clone())
    }
    pub fn services_client(&self) -> services::Client {
        services::Client(self.clone())
    }
    pub fn update_client(&self) -> update::Client {
        update::Client(self.clone())
    }
}
pub mod adds_services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of Active Directory Domain Service, for a tenant, that are onboarded to Azure Active Directory Connect Health."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                service_type: None,
                skip_count: None,
                take_count: None,
            }
        }
        #[doc = "Onboards a service for a given tenant in Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service`: The service object."]
        pub fn add(&self, service: impl Into<models::ServiceProperties>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                service: service.into(),
            }
        }
        #[doc = "Gets the details of an Active Directory Domain Service for a tenant having Azure AD Premium license and is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn get(&self, service_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Updates an Active Directory Domain Service properties of an onboarded service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service which needs to be deleted."]
        #[doc = "* `service`: The service object."]
        pub fn update(&self, service_name: impl Into<String>, service: impl Into<models::ServiceProperties>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service: service.into(),
            }
        }
        #[doc = "Deletes an Active Directory Domain Service which is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service which needs to be deleted."]
        pub fn delete(&self, service_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                confirm: None,
            }
        }
        #[doc = "Gets the forest summary for a given Active Directory Domain Service, that is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn get_forest_summary(&self, service_name: impl Into<String>) -> get_forest_summary::Builder {
            get_forest_summary::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Gets the average of the metric values for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn list_metrics_average(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_metrics_average::Builder {
            list_metrics_average::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
            }
        }
        #[doc = "Gets the sum of the metric values for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn list_metrics_sum(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_metrics_sum::Builder {
            list_metrics_sum::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
            }
        }
        #[doc = "Gets the service related metrics information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_metric_metadata(&self, service_name: impl Into<String>) -> list_metric_metadata::Builder {
            list_metric_metadata::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                perf_counter: None,
            }
        }
        #[doc = "Gets the service related metric information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        pub fn get_metric_metadata(&self, service_name: impl Into<String>, metric_name: impl Into<String>) -> get_metric_metadata::Builder {
            get_metric_metadata::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
            }
        }
        #[doc = "Gets the service related metrics for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn get_metric_metadata_for_group(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> get_metric_metadata_for_group::Builder {
            get_metric_metadata_for_group::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
                group_key: None,
                from_date: None,
                to_date: None,
            }
        }
        #[doc = "Gets complete domain controller list along with replication details for a given Active Directory Domain Service, that is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_replication_details(&self, service_name: impl Into<String>) -> list_replication_details::Builder {
            list_replication_details::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                with_details: None,
            }
        }
        #[doc = "Gets complete domain controller list along with replication details for a given Active Directory Domain Service, that is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `is_groupby_site`: Indicates if the result should be grouped by site or not."]
        #[doc = "* `query`: The custom query."]
        #[doc = "* `next_partition_key`: The next partition key to query for."]
        #[doc = "* `next_row_key`: The next row key to query for."]
        pub fn list_replication_summary(
            &self,
            service_name: impl Into<String>,
            is_groupby_site: bool,
            query: impl Into<String>,
            next_partition_key: impl Into<String>,
            next_row_key: impl Into<String>,
        ) -> list_replication_summary::Builder {
            list_replication_summary::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                is_groupby_site,
                query: query.into(),
                next_partition_key: next_partition_key.into(),
                next_row_key: next_row_key.into(),
                filter: None,
                take_count: None,
            }
        }
        #[doc = "Gets the details of an alert for a given Active Directory Domain Controller service and server combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_member_id`: The server Id for which the alert details needs to be queried."]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_server_alerts(
            &self,
            service_member_id: impl Into<String>,
            service_name: impl Into<String>,
        ) -> list_server_alerts::Builder {
            list_server_alerts::Builder {
                client: self.0.clone(),
                service_member_id: service_member_id.into(),
                service_name: service_name.into(),
                filter: None,
                state: None,
                from: None,
                to: None,
            }
        }
        #[doc = "Gets the details of Active Directory Domain Services for a tenant having Azure AD Premium license and is onboarded to Azure Active Directory Connect Health."]
        pub fn list_premium_services(&self) -> list_premium_services::Builder {
            list_premium_services::Builder {
                client: self.0.clone(),
                filter: None,
                service_type: None,
                skip_count: None,
                take_count: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::Services;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) service_type: Option<String>,
            pub(crate) skip_count: Option<i64>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The service property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
            pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
                self.service_type = Some(service_type.into());
                self
            }
            #[doc = "The skip count, which specifies the number of elements that can be bypassed from a sequence and then return the remaining elements."]
            pub fn skip_count(mut self, skip_count: i64) -> Self {
                self.skip_count = Some(skip_count);
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(service_type) = &this.service_type {
                                    req.url_mut().query_pairs_mut().append_pair("serviceType", service_type);
                                }
                                if let Some(skip_count) = &this.skip_count {
                                    req.url_mut().query_pairs_mut().append_pair("skipCount", &skip_count.to_string());
                                }
                                if let Some(take_count) = &this.take_count {
                                    req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
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
                                let rsp_value: models::Services = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service: models::ServiceProperties,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service: models::ServiceProperties,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) service_name: String,
            pub(crate) confirm: Option<bool>,
        }
        impl Builder {
            #[doc = "Indicates if the service will be permanently deleted or disabled. True indicates that the service will be permanently deleted and False indicates that the service will be marked disabled and then deleted after 30 days, if it is not re-registered."]
            pub fn confirm(mut self, confirm: bool) -> Self {
                self.confirm = Some(confirm);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(confirm) = &this.confirm {
                            req.url_mut().query_pairs_mut().append_pair("confirm", &confirm.to_string());
                        }
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
    pub mod get_forest_summary {
        use super::models;
        type Response = models::ForestSummary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/forestsummary",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ForestSummary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_metrics_average {
        use super::models;
        type Response = models::Metrics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metrics/{}/groups/{}/average",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Metrics = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metrics_sum {
        use super::models;
        type Response = models::Metrics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metrics/{}/groups/{}/sum",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Metrics = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_metadata {
        use super::models;
        type Response = models::MetricMetadataList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) perf_counter: Option<bool>,
        }
        impl Builder {
            #[doc = "The metric metadata property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Indicates if only performance counter metrics are requested."]
            pub fn perf_counter(mut self, perf_counter: bool) -> Self {
                self.perf_counter = Some(perf_counter);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metricmetadata",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(perf_counter) = &this.perf_counter {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("perfCounter", &perf_counter.to_string());
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
                                let rsp_value: models::MetricMetadataList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_metric_metadata {
        use super::models;
        type Response = models::MetricMetadata;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metricmetadata/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricMetadata = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_metric_metadata_for_group {
        use super::models;
        type Response = models::MetricSets;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
            pub(crate) group_key: Option<String>,
            pub(crate) from_date: Option<time::OffsetDateTime>,
            pub(crate) to_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The group key"]
            pub fn group_key(mut self, group_key: impl Into<String>) -> Self {
                self.group_key = Some(group_key.into());
                self
            }
            #[doc = "The start date."]
            pub fn from_date(mut self, from_date: impl Into<time::OffsetDateTime>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "The end date."]
            pub fn to_date(mut self, to_date: impl Into<time::OffsetDateTime>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metricmetadata/{}/groups/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(group_key) = &this.group_key {
                            req.url_mut().query_pairs_mut().append_pair("groupKey", group_key);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut().query_pairs_mut().append_pair("fromDate", &from_date.to_string());
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut().query_pairs_mut().append_pair("toDate", &to_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricSets = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_replication_details {
        use super::models;
        type Response = models::ReplicationDetailsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) with_details: Option<bool>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Indicates if InboundReplicationNeighbor details are required or not."]
            pub fn with_details(mut self, with_details: bool) -> Self {
                self.with_details = Some(with_details);
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/replicationdetails",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(with_details) = &this.with_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("withDetails", &with_details.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationDetailsList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_replication_summary {
        use super::models;
        type Response = models::ReplicationSummaryList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) is_groupby_site: bool,
            pub(crate) query: String,
            pub(crate) next_partition_key: String,
            pub(crate) next_row_key: String,
            pub(crate) filter: Option<String>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/replicationsummary",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let is_groupby_site = &this.is_groupby_site;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("isGroupbySite", &is_groupby_site.to_string());
                        let query = &this.query;
                        req.url_mut().query_pairs_mut().append_pair("query", query);
                        let next_partition_key = &this.next_partition_key;
                        req.url_mut().query_pairs_mut().append_pair("nextPartitionKey", next_partition_key);
                        let next_row_key = &this.next_row_key;
                        req.url_mut().query_pairs_mut().append_pair("nextRowKey", next_row_key);
                        if let Some(take_count) = &this.take_count {
                            req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationSummaryList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_server_alerts {
        use super::models;
        type Response = models::Alerts;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_member_id: String,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) state: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The alert property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The alert state to query for."]
            pub fn state(mut self, state: impl Into<String>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "The start date to query for."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "The end date till when to query for."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers/{}/alerts",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(state) = &this.state {
                                    req.url_mut().query_pairs_mut().append_pair("state", state);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("to", &to.to_string());
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
                                let rsp_value: models::Alerts = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_premium_services {
        use super::models;
        type Response = models::Services;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) service_type: Option<String>,
            pub(crate) skip_count: Option<i64>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The service property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
            pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
                self.service_type = Some(service_type.into());
                self
            }
            #[doc = "The skip count, which specifies the number of elements that can be bypassed from a sequence and then return the remaining elements."]
            pub fn skip_count(mut self, skip_count: i64) -> Self {
                self.skip_count = Some(skip_count);
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/premiumCheck",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(service_type) = &this.service_type {
                                    req.url_mut().query_pairs_mut().append_pair("serviceType", service_type);
                                }
                                if let Some(skip_count) = &this.skip_count {
                                    req.url_mut().query_pairs_mut().append_pair("skipCount", &skip_count.to_string());
                                }
                                if let Some(take_count) = &this.take_count {
                                    req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
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
                                let rsp_value: models::Services = serde_json::from_slice(&rsp_body)?;
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
pub mod alerts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the alerts for a given Active Directory Domain Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_adds_alerts(&self, service_name: impl Into<String>) -> list_adds_alerts::Builder {
            list_adds_alerts::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                state: None,
                from: None,
                to: None,
            }
        }
    }
    pub mod list_adds_alerts {
        use super::models;
        type Response = models::Alerts;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) state: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The alert property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The alert state to query for."]
            pub fn state(mut self, state: impl Into<String>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "The start date to query for."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "The end date till when to query for."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/alerts",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(state) = &this.state {
                                    req.url_mut().query_pairs_mut().append_pair("state", state);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("to", &to.to_string());
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
                                let rsp_value: models::Alerts = serde_json::from_slice(&rsp_body)?;
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
pub mod configuration {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the service configurations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_adds_configurations(&self, service_name: impl Into<String>) -> list_adds_configurations::Builder {
            list_adds_configurations::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                grouping: None,
            }
        }
        #[doc = "Gets the details of a tenant onboarded to Azure Active Directory Connect Health."]
        pub fn get(&self) -> get::Builder {
            get::Builder { client: self.0.clone() }
        }
        #[doc = "Onboards a tenant in Azure Active Directory Connect Health."]
        pub fn add(&self) -> add::Builder {
            add::Builder { client: self.0.clone() }
        }
        #[doc = "Updates tenant properties for tenants onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `tenant`: The tenant object with the properties set to the updated value."]
        pub fn update(&self, tenant: impl Into<models::Tenant>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                tenant: tenant.into(),
            }
        }
    }
    pub mod list_adds_configurations {
        use super::models;
        type Response = models::AddsConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) grouping: Option<String>,
        }
        impl Builder {
            #[doc = "The grouping for configurations."]
            pub fn grouping(mut self, grouping: impl Into<String>) -> Self {
                self.grouping = Some(grouping.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/configuration",
                            this.client.endpoint(),
                            &this.service_name
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
                                if let Some(grouping) = &this.grouping {
                                    req.url_mut().query_pairs_mut().append_pair("grouping", grouping);
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
                                let rsp_value: models::AddsConfiguration = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Tenant;
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
                            "{}/providers/Microsoft.ADHybridHealthService/configuration",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Tenant = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add {
        use super::models;
        type Response = models::Tenant;
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
                            "{}/providers/Microsoft.ADHybridHealthService/configuration",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Tenant = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::Tenant;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) tenant: models::Tenant,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/configuration",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.tenant)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Tenant = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod dimensions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the dimensions for a given dimension type in a server."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `dimension`: The dimension type."]
        pub fn list_adds_dimensions(&self, service_name: impl Into<String>, dimension: impl Into<String>) -> list_adds_dimensions::Builder {
            list_adds_dimensions::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                dimension: dimension.into(),
            }
        }
    }
    pub mod list_adds_dimensions {
        use super::models;
        type Response = models::Dimensions;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) dimension: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/dimensions/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.dimension
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Dimensions = serde_json::from_slice(&rsp_body)?;
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
pub mod adds_service_members {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of the Active Directory Domain servers, for a given Active Directory Domain Service, that are onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list(&self, service_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
            }
        }
        #[doc = "Gets the details of a server, for a given Active Directory Domain Controller service, that are onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn get(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Deletes a Active Directory Domain Controller server that has been onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn delete(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
                confirm: None,
            }
        }
        #[doc = "Gets the credentials of the server which is needed by the agent to connect to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn list_credentials(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> list_credentials::Builder {
            list_credentials::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AddsServiceMembers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/addsservicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                let rsp_value: models::AddsServiceMembers = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ServiceMember;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceMember = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
            pub(crate) confirm: Option<bool>,
        }
        impl Builder {
            #[doc = "Indicates if the server will be permanently deleted or disabled. True indicates that the server will be permanently deleted and False indicates that the server will be marked disabled and then deleted after 30 days, if it is not re-registered."]
            pub fn confirm(mut self, confirm: bool) -> Self {
                self.confirm = Some(confirm);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(confirm) = &this.confirm {
                            req.url_mut().query_pairs_mut().append_pair("confirm", &confirm.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod list_credentials {
        use super::models;
        type Response = models::Credentials;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The property filter to apply."]
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
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers/{}/credentials",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                let rsp_value: models::Credentials = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod ad_domain_service_members {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of the servers, for a given Active Directory Domain Service, that are onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `is_groupby_site`: Indicates if the result should be grouped by site or not."]
        #[doc = "* `next_partition_key`: The next partition key to query for."]
        #[doc = "* `next_row_key`: The next row key to query for."]
        pub fn list(
            &self,
            service_name: impl Into<String>,
            is_groupby_site: bool,
            next_partition_key: impl Into<String>,
            next_row_key: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                is_groupby_site,
                next_partition_key: next_partition_key.into(),
                next_row_key: next_row_key.into(),
                filter: None,
                query: None,
                take_count: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AddsServiceMembers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) is_groupby_site: bool,
            pub(crate) next_partition_key: String,
            pub(crate) next_row_key: String,
            pub(crate) filter: Option<String>,
            pub(crate) query: Option<String>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The custom query."]
            pub fn query(mut self, query: impl Into<String>) -> Self {
                self.query = Some(query.into());
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/addomainservicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let is_groupby_site = &this.is_groupby_site;
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair("isGroupbySite", &is_groupby_site.to_string());
                                if let Some(query) = &this.query {
                                    req.url_mut().query_pairs_mut().append_pair("query", query);
                                }
                                let next_partition_key = &this.next_partition_key;
                                req.url_mut().query_pairs_mut().append_pair("nextPartitionKey", next_partition_key);
                                let next_row_key = &this.next_row_key;
                                req.url_mut().query_pairs_mut().append_pair("nextRowKey", next_row_key);
                                if let Some(take_count) = &this.take_count {
                                    req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
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
                                let rsp_value: models::AddsServiceMembers = serde_json::from_slice(&rsp_body)?;
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
pub mod adds_services_user_preference {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the user preferences for a given feature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `feature_name`: The name of the feature."]
        pub fn get(&self, service_name: impl Into<String>, feature_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                feature_name: feature_name.into(),
            }
        }
        #[doc = "Adds the user preferences for a given feature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `feature_name`: The name of the feature."]
        #[doc = "* `setting`: The user preference setting."]
        pub fn add(
            &self,
            service_name: impl Into<String>,
            feature_name: impl Into<String>,
            setting: impl Into<models::UserPreference>,
        ) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                feature_name: feature_name.into(),
                setting: setting.into(),
            }
        }
        #[doc = "Deletes the user preferences for a given feature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `feature_name`: The name of the feature."]
        pub fn delete(&self, service_name: impl Into<String>, feature_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                feature_name: feature_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::UserPreference;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) feature_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/features/{}/userpreference",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.feature_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UserPreference = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) feature_name: String,
            pub(crate) setting: models::UserPreference,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/features/{}/userpreference",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.feature_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.setting)?;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) feature_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/features/{}/userpreference",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.feature_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
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
pub mod adds_service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the server related metrics for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn get_metrics(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> get_metrics::Builder {
            get_metrics::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
                group_key: None,
                from_date: None,
                to_date: None,
            }
        }
    }
    pub mod get_metrics {
        use super::models;
        type Response = models::MetricSets;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
            pub(crate) group_key: Option<String>,
            pub(crate) from_date: Option<time::OffsetDateTime>,
            pub(crate) to_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The group key"]
            pub fn group_key(mut self, group_key: impl Into<String>) -> Self {
                self.group_key = Some(group_key.into());
                self
            }
            #[doc = "The start date."]
            pub fn from_date(mut self, from_date: impl Into<time::OffsetDateTime>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "The end date."]
            pub fn to_date(mut self, to_date: impl Into<time::OffsetDateTime>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/metrics/{}/groups/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(group_key) = &this.group_key {
                            req.url_mut().query_pairs_mut().append_pair("groupKey", group_key);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut().query_pairs_mut().append_pair("fromDate", &from_date.to_string());
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut().query_pairs_mut().append_pair("toDate", &to_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricSets = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod adds_services_replication_status {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets Replication status for a given Active Directory Domain Service, that is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn get(&self, service_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ReplicationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/replicationstatus",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
}
pub mod adds_services_service_members {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of the servers, for a given Active Directory Domain Controller service, that are onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list(&self, service_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                dimension_type: None,
                dimension_signature: None,
            }
        }
        #[doc = "Onboards  a server, for a given Active Directory Domain Controller service, to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service under which the server is to be onboarded."]
        #[doc = "* `service_member`: The server object."]
        pub fn add(&self, service_name: impl Into<String>, service_member: impl Into<models::ServiceMember>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member: service_member.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ServiceMembers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) dimension_type: Option<String>,
            pub(crate) dimension_signature: Option<String>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The server specific dimension."]
            pub fn dimension_type(mut self, dimension_type: impl Into<String>) -> Self {
                self.dimension_type = Some(dimension_type.into());
                self
            }
            #[doc = "The value of the dimension."]
            pub fn dimension_signature(mut self, dimension_signature: impl Into<String>) -> Self {
                self.dimension_signature = Some(dimension_signature.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(dimension_type) = &this.dimension_type {
                                    req.url_mut().query_pairs_mut().append_pair("dimensionType", dimension_type);
                                }
                                if let Some(dimension_signature) = &this.dimension_signature {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("dimensionSignature", dimension_signature);
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
                                let rsp_value: models::ServiceMembers = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = models::ServiceMember;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member: models::ServiceMember,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/addsservices/{}/servicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service_member)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceMember = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        #[doc = "Lists the available Azure Data Factory API operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationListResponse;
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
                            "{}/providers/Microsoft.ADHybridHealthService/operations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationListResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod reports {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Checks if the user is enabled for Dev Ops access."]
        pub fn get_dev_ops(&self) -> get_dev_ops::Builder {
            get_dev_ops::Builder { client: self.0.clone() }
        }
    }
    pub mod get_dev_ops {
        use super::models;
        type Response = models::Result;
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
                            "{}/providers/Microsoft.ADHybridHealthService/reports/DevOps/IsDevOps",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Result = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of services, for a tenant, that are onboarded to Azure Active Directory Connect Health."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                service_type: None,
                skip_count: None,
                take_count: None,
            }
        }
        #[doc = "Onboards a service for a given tenant in Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service`: The service object."]
        pub fn add(&self, service: impl Into<models::ServiceProperties>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                service: service.into(),
            }
        }
        #[doc = "Gets the details of services for a tenant having Azure AD Premium license and is onboarded to Azure Active Directory Connect Health."]
        pub fn list_premium(&self) -> list_premium::Builder {
            list_premium::Builder {
                client: self.0.clone(),
                filter: None,
                service_type: None,
                skip_count: None,
                take_count: None,
            }
        }
        #[doc = "Gets the details of a service for a tenant having Azure AD Premium license and is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn get(&self, service_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Updates the service properties of an onboarded service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service which needs to be deleted."]
        #[doc = "* `service`: The service object."]
        pub fn update(&self, service_name: impl Into<String>, service: impl Into<models::ServiceProperties>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service: service.into(),
            }
        }
        #[doc = "Deletes a service which is onboarded to Azure Active Directory Connect Health."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service which needs to be deleted."]
        pub fn delete(&self, service_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                confirm: None,
            }
        }
        #[doc = "Gets the alerts for a given service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_alerts(&self, service_name: impl Into<String>) -> list_alerts::Builder {
            list_alerts::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                state: None,
                from: None,
                to: None,
            }
        }
        #[doc = "Checks if the service has all the pre-requisites met to use a feature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `feature_name`: The name of the feature."]
        pub fn get_feature_availibility(
            &self,
            service_name: impl Into<String>,
            feature_name: impl Into<String>,
        ) -> get_feature_availibility::Builder {
            get_feature_availibility::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                feature_name: feature_name.into(),
            }
        }
        #[doc = "Gets the count of latest AAD export errors."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_export_errors(&self, service_name: impl Into<String>) -> list_export_errors::Builder {
            list_export_errors::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = " Gets the categorized export errors."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `error_bucket`: The error category to query for."]
        pub fn list_export_errors_v2(
            &self,
            service_name: impl Into<String>,
            error_bucket: impl Into<String>,
        ) -> list_export_errors_v2::Builder {
            list_export_errors_v2::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                error_bucket: error_bucket.into(),
            }
        }
        #[doc = "Gets the export status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_export_status(&self, service_name: impl Into<String>) -> list_export_status::Builder {
            list_export_status::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Adds an alert feedback submitted by customer."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `alert_feedback`: The alert feedback."]
        pub fn add_alert_feedback(
            &self,
            service_name: impl Into<String>,
            alert_feedback: impl Into<models::AlertFeedback>,
        ) -> add_alert_feedback::Builder {
            add_alert_feedback::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                alert_feedback: alert_feedback.into(),
            }
        }
        #[doc = "Gets a list of all alert feedback for a given tenant and alert type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `short_name`: The name of the alert."]
        pub fn list_alert_feedback(&self, service_name: impl Into<String>, short_name: impl Into<String>) -> list_alert_feedback::Builder {
            list_alert_feedback::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                short_name: short_name.into(),
            }
        }
        #[doc = "Gets the average of the metric values for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn list_metrics_average(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_metrics_average::Builder {
            list_metrics_average::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
            }
        }
        #[doc = "Gets the sum of the metric values for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn list_metrics_sum(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_metrics_sum::Builder {
            list_metrics_sum::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
            }
        }
        #[doc = "Gets the service related metrics information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_metric_metadata(&self, service_name: impl Into<String>) -> list_metric_metadata::Builder {
            list_metric_metadata::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                perf_counter: None,
            }
        }
        #[doc = "Gets the service related metrics information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        pub fn get_metric_metadata(&self, service_name: impl Into<String>, metric_name: impl Into<String>) -> get_metric_metadata::Builder {
            get_metric_metadata::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
            }
        }
        #[doc = "Gets the service related metrics for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn get_metric_metadata_for_group(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> get_metric_metadata_for_group::Builder {
            get_metric_metadata_for_group::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
                group_key: None,
                from_date: None,
                to_date: None,
            }
        }
        #[doc = "Updates the service level monitoring configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `configuration_setting`: The monitoring configuration to update"]
        pub fn update_monitoring_configuration(
            &self,
            service_name: impl Into<String>,
            configuration_setting: impl Into<models::Item>,
        ) -> update_monitoring_configuration::Builder {
            update_monitoring_configuration::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                configuration_setting: configuration_setting.into(),
            }
        }
        #[doc = "Gets the service level monitoring configurations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_monitoring_configurations(&self, service_name: impl Into<String>) -> list_monitoring_configurations::Builder {
            list_monitoring_configurations::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Gets the bad password login attempt report for an user"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_user_bad_password_report(&self, service_name: impl Into<String>) -> list_user_bad_password_report::Builder {
            list_user_bad_password_report::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                data_source: None,
            }
        }
        #[doc = "Checks if the tenant, to which a service is registered, is whitelisted to use a feature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `feature_name`: The name of the feature."]
        pub fn get_tenant_whitelisting(
            &self,
            service_name: impl Into<String>,
            feature_name: impl Into<String>,
        ) -> get_tenant_whitelisting::Builder {
            get_tenant_whitelisting::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                feature_name: feature_name.into(),
            }
        }
        #[doc = "Gets all Risky IP report URIs for the last 7 days."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_all_risky_ip_download_report(&self, service_name: impl Into<String>) -> list_all_risky_ip_download_report::Builder {
            list_all_risky_ip_download_report::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Initiate the generation of a new Risky IP report. Returns the URI for the new one."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_current_risky_ip_download_report(
            &self,
            service_name: impl Into<String>,
        ) -> list_current_risky_ip_download_report::Builder {
            list_current_risky_ip_download_report::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::Services;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) service_type: Option<String>,
            pub(crate) skip_count: Option<i64>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The service property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
            pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
                self.service_type = Some(service_type.into());
                self
            }
            #[doc = "The skip count, which specifies the number of elements that can be bypassed from a sequence and then return the remaining elements."]
            pub fn skip_count(mut self, skip_count: i64) -> Self {
                self.skip_count = Some(skip_count);
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(service_type) = &this.service_type {
                                    req.url_mut().query_pairs_mut().append_pair("serviceType", service_type);
                                }
                                if let Some(skip_count) = &this.skip_count {
                                    req.url_mut().query_pairs_mut().append_pair("skipCount", &skip_count.to_string());
                                }
                                if let Some(take_count) = &this.take_count {
                                    req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
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
                                let rsp_value: models::Services = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service: models::ServiceProperties,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_premium {
        use super::models;
        type Response = models::Services;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) service_type: Option<String>,
            pub(crate) skip_count: Option<i64>,
            pub(crate) take_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The service property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The service type for the services onboarded to Azure Active Directory Connect Health. Depending on whether the service is monitoring, ADFS, Sync or ADDS roles, the service type can either be AdFederationService or AadSyncService or AdDomainService."]
            pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
                self.service_type = Some(service_type.into());
                self
            }
            #[doc = "The skip count, which specifies the number of elements that can be bypassed from a sequence and then return the remaining elements."]
            pub fn skip_count(mut self, skip_count: i64) -> Self {
                self.skip_count = Some(skip_count);
                self
            }
            #[doc = "The take count , which specifies the number of elements that can be returned from a sequence."]
            pub fn take_count(mut self, take_count: i64) -> Self {
                self.take_count = Some(take_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/premiumCheck",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(service_type) = &this.service_type {
                                    req.url_mut().query_pairs_mut().append_pair("serviceType", service_type);
                                }
                                if let Some(skip_count) = &this.skip_count {
                                    req.url_mut().query_pairs_mut().append_pair("skipCount", &skip_count.to_string());
                                }
                                if let Some(take_count) = &this.take_count {
                                    req.url_mut().query_pairs_mut().append_pair("takeCount", &take_count.to_string());
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
                                let rsp_value: models::Services = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::ServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service: models::ServiceProperties,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) service_name: String,
            pub(crate) confirm: Option<bool>,
        }
        impl Builder {
            #[doc = "Indicates if the service will be permanently deleted or disabled. True indicates that the service will be permanently deleted and False indicates that the service will be marked disabled and then deleted after 30 days, if it is not re-registered."]
            pub fn confirm(mut self, confirm: bool) -> Self {
                self.confirm = Some(confirm);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(confirm) = &this.confirm {
                            req.url_mut().query_pairs_mut().append_pair("confirm", &confirm.to_string());
                        }
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
    pub mod list_alerts {
        use super::models;
        type Response = models::Alerts;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) state: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The alert property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The alert state to query for."]
            pub fn state(mut self, state: impl Into<String>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "The start date to query for."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "The end date till when to query for."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/alerts",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(state) = &this.state {
                                    req.url_mut().query_pairs_mut().append_pair("state", state);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("to", &to.to_string());
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
                                let rsp_value: models::Alerts = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_feature_availibility {
        use super::models;
        type Response = models::Result;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) feature_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/checkServiceFeatureAvailibility/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.feature_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Result = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_export_errors {
        use super::models;
        type Response = models::ErrorCounts;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/exporterrors/counts",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ErrorCounts = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_export_errors_v2 {
        use super::models;
        type Response = models::MergedExportErrors;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) error_bucket: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/exporterrors/listV2",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let error_bucket = &this.error_bucket;
                        req.url_mut().query_pairs_mut().append_pair("errorBucket", error_bucket);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MergedExportErrors = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_export_status {
        use super::models;
        type Response = models::ExportStatuses;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/exportstatus",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportStatuses = serde_json::from_slice(&rsp_body)?;
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
    pub mod add_alert_feedback {
        use super::models;
        type Response = models::AlertFeedback;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) alert_feedback: models::AlertFeedback,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/feedbacktype/alerts/feedback",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.alert_feedback)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertFeedback = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_alert_feedback {
        use super::models;
        type Response = models::AlertFeedbacks;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) short_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/feedbacktype/alerts/{}/alertfeedback",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.short_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertFeedbacks = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_metrics_average {
        use super::models;
        type Response = models::Metrics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metrics/{}/groups/{}/average",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Metrics = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metrics_sum {
        use super::models;
        type Response = models::Metrics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metrics/{}/groups/{}/sum",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Metrics = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_metric_metadata {
        use super::models;
        type Response = models::MetricMetadataList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) perf_counter: Option<bool>,
        }
        impl Builder {
            #[doc = "The metric metadata property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Indicates if only performance counter metrics are requested."]
            pub fn perf_counter(mut self, perf_counter: bool) -> Self {
                self.perf_counter = Some(perf_counter);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metricmetadata",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(perf_counter) = &this.perf_counter {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("perfCounter", &perf_counter.to_string());
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
                                let rsp_value: models::MetricMetadataList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_metric_metadata {
        use super::models;
        type Response = models::MetricMetadata;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metricmetadata/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricMetadata = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_metric_metadata_for_group {
        use super::models;
        type Response = models::MetricSets;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
            pub(crate) group_key: Option<String>,
            pub(crate) from_date: Option<time::OffsetDateTime>,
            pub(crate) to_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The group key"]
            pub fn group_key(mut self, group_key: impl Into<String>) -> Self {
                self.group_key = Some(group_key.into());
                self
            }
            #[doc = "The start date."]
            pub fn from_date(mut self, from_date: impl Into<time::OffsetDateTime>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "The end date."]
            pub fn to_date(mut self, to_date: impl Into<time::OffsetDateTime>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metricmetadata/{}/groups/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(group_key) = &this.group_key {
                            req.url_mut().query_pairs_mut().append_pair("groupKey", group_key);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut().query_pairs_mut().append_pair("fromDate", &from_date.to_string());
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut().query_pairs_mut().append_pair("toDate", &to_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricSets = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_monitoring_configuration {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) configuration_setting: models::Item,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/monitoringconfiguration",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.configuration_setting)?;
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
    pub mod list_monitoring_configurations {
        use super::models;
        type Response = models::Items;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/monitoringconfigurations",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Items = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_user_bad_password_report {
        use super::models;
        type Response = models::ErrorReportUsersEntries;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) data_source: Option<String>,
        }
        impl Builder {
            #[doc = "The source of data, if its test data or customer data."]
            pub fn data_source(mut self, data_source: impl Into<String>) -> Self {
                self.data_source = Some(data_source.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/reports/badpassword/details/user",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(data_source) = &this.data_source {
                            req.url_mut().query_pairs_mut().append_pair("dataSource", data_source);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ErrorReportUsersEntries = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_tenant_whitelisting {
        use super::models;
        type Response = models::Result;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) feature_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/TenantWhitelisting/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.feature_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Result = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_all_risky_ip_download_report {
        use super::models;
        type Response = models::RiskyIpBlobUris;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/reports/riskyIp/blobUris",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RiskyIpBlobUris = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_current_risky_ip_download_report {
        use super::models;
        type Response = models::RiskyIpBlobUris;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/reports/riskyIp/generateBlobUri",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RiskyIpBlobUris = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the server related metrics for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        pub fn get_metrics(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> get_metrics::Builder {
            get_metrics::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
                group_key: None,
                from_date: None,
                to_date: None,
            }
        }
    }
    pub mod get_metrics {
        use super::models;
        type Response = models::MetricSets;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
            pub(crate) group_key: Option<String>,
            pub(crate) from_date: Option<time::OffsetDateTime>,
            pub(crate) to_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The group key"]
            pub fn group_key(mut self, group_key: impl Into<String>) -> Self {
                self.group_key = Some(group_key.into());
                self
            }
            #[doc = "The start date."]
            pub fn from_date(mut self, from_date: impl Into<time::OffsetDateTime>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "The end date."]
            pub fn to_date(mut self, to_date: impl Into<time::OffsetDateTime>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/metrics/{}/groups/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.metric_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(group_key) = &this.group_key {
                            req.url_mut().query_pairs_mut().append_pair("groupKey", group_key);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut().query_pairs_mut().append_pair("fromDate", &from_date.to_string());
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut().query_pairs_mut().append_pair("toDate", &to_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricSets = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod service_members {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the details of the servers, for a given service, that are onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list(&self, service_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                filter: None,
                dimension_type: None,
                dimension_signature: None,
            }
        }
        #[doc = "Onboards  a server, for a given service, to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service under which the server is to be onboarded."]
        #[doc = "* `service_member`: The server object."]
        pub fn add(&self, service_name: impl Into<String>, service_member: impl Into<models::ServiceMember>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member: service_member.into(),
            }
        }
        #[doc = "Gets the details of a server, for a given service, that are onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn get(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Deletes a server that has been onboarded to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn delete(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
                confirm: None,
            }
        }
        #[doc = "Gets the details of an alert for a given service and server combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_member_id`: The server Id for which the alert details needs to be queried."]
        #[doc = "* `service_name`: The name of the service."]
        pub fn list_alerts(&self, service_member_id: impl Into<String>, service_name: impl Into<String>) -> list_alerts::Builder {
            list_alerts::Builder {
                client: self.0.clone(),
                service_member_id: service_member_id.into(),
                service_name: service_name.into(),
                filter: None,
                state: None,
                from: None,
                to: None,
            }
        }
        #[doc = "Gets the connector details for a service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn list_connectors(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> list_connectors::Builder {
            list_connectors::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the credentials of the server which is needed by the agent to connect to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn list_credentials(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> list_credentials::Builder {
            list_credentials::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
                filter: None,
            }
        }
        #[doc = "Deletes the data uploaded by the server to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn delete_data(&self, service_name: impl Into<String>, service_member_id: impl Into<String>) -> delete_data::Builder {
            delete_data::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the last time when the server uploaded data to Azure Active Directory Connect Health Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn list_data_freshness(
            &self,
            service_name: impl Into<String>,
            service_member_id: impl Into<String>,
        ) -> list_data_freshness::Builder {
            list_data_freshness::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the export status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn list_export_status(
            &self,
            service_name: impl Into<String>,
            service_member_id: impl Into<String>,
        ) -> list_export_status::Builder {
            list_export_status::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the global configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server id."]
        pub fn list_global_configuration(
            &self,
            service_name: impl Into<String>,
            service_member_id: impl Into<String>,
        ) -> list_global_configuration::Builder {
            list_global_configuration::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the server related metrics for a given metric and group combination."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `metric_name`: The metric name"]
        #[doc = "* `group_name`: The group name"]
        #[doc = "* `service_member_id`: The server id."]
        pub fn get_metrics(
            &self,
            service_name: impl Into<String>,
            metric_name: impl Into<String>,
            group_name: impl Into<String>,
            service_member_id: impl Into<String>,
        ) -> get_metrics::Builder {
            get_metrics::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                metric_name: metric_name.into(),
                group_name: group_name.into(),
                service_member_id: service_member_id.into(),
                group_key: None,
                from_date: None,
                to_date: None,
            }
        }
        #[doc = "Gets the service configuration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The server Id."]
        pub fn get_service_configuration(
            &self,
            service_name: impl Into<String>,
            service_member_id: impl Into<String>,
        ) -> get_service_configuration::Builder {
            get_service_configuration::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
            }
        }
        #[doc = "Gets the list of connectors and run profile names."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `service_member_id`: The service member id."]
        #[doc = "* `metric_name`: The name of the metric."]
        pub fn get_connector_metadata(
            &self,
            service_name: impl Into<String>,
            service_member_id: impl Into<String>,
            metric_name: impl Into<String>,
        ) -> get_connector_metadata::Builder {
            get_connector_metadata::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                service_member_id: service_member_id.into(),
                metric_name: metric_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ServiceMembers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) dimension_type: Option<String>,
            pub(crate) dimension_signature: Option<String>,
        }
        impl Builder {
            #[doc = "The server property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The server specific dimension."]
            pub fn dimension_type(mut self, dimension_type: impl Into<String>) -> Self {
                self.dimension_type = Some(dimension_type.into());
                self
            }
            #[doc = "The value of the dimension."]
            pub fn dimension_signature(mut self, dimension_signature: impl Into<String>) -> Self {
                self.dimension_signature = Some(dimension_signature.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(dimension_type) = &this.dimension_type {
                                    req.url_mut().query_pairs_mut().append_pair("dimensionType", dimension_type);
                                }
                                if let Some(dimension_signature) = &this.dimension_signature {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("dimensionSignature", dimension_signature);
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
                                let rsp_value: models::ServiceMembers = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = models::ServiceMember;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member: models::ServiceMember,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.service_member)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceMember = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::ServiceMember;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceMember = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
            pub(crate) confirm: Option<bool>,
        }
        impl Builder {
            #[doc = "Indicates if the server will be permanently deleted or disabled. True indicates that the server will be permanently deleted and False indicates that the server will be marked disabled and then deleted after 30 days, if it is not re-registered."]
            pub fn confirm(mut self, confirm: bool) -> Self {
                self.confirm = Some(confirm);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(confirm) = &this.confirm {
                            req.url_mut().query_pairs_mut().append_pair("confirm", &confirm.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod list_alerts {
        use super::models;
        type Response = models::Alerts;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_member_id: String,
            pub(crate) service_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) state: Option<String>,
            pub(crate) from: Option<time::OffsetDateTime>,
            pub(crate) to: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The alert property filter to apply."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The alert state to query for."]
            pub fn state(mut self, state: impl Into<String>) -> Self {
                self.state = Some(state.into());
                self
            }
            #[doc = "The start date to query for."]
            pub fn from(mut self, from: impl Into<time::OffsetDateTime>) -> Self {
                self.from = Some(from.into());
                self
            }
            #[doc = "The end date till when to query for."]
            pub fn to(mut self, to: impl Into<time::OffsetDateTime>) -> Self {
                self.to = Some(to.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/alerts",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(state) = &this.state {
                                    req.url_mut().query_pairs_mut().append_pair("state", state);
                                }
                                if let Some(from) = &this.from {
                                    req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                                }
                                if let Some(to) = &this.to {
                                    req.url_mut().query_pairs_mut().append_pair("to", &to.to_string());
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
                                let rsp_value: models::Alerts = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_connectors {
        use super::models;
        type Response = models::Connectors;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/service/{}/servicemembers/{}/connectors",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Connectors = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_credentials {
        use super::models;
        type Response = models::Credentials;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The property filter to apply."]
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
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/credentials",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                let rsp_value: models::Credentials = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_data {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/data",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod list_data_freshness {
        use super::models;
        type Response = models::DataFreshnessDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/datafreshness",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataFreshnessDetails = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_export_status {
        use super::models;
        type Response = models::ExportStatuses;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/exportstatus",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportStatuses = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_global_configuration {
        use super::models;
        type Response = models::GlobalConfigurations;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/globalconfiguration",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GlobalConfigurations = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_metrics {
        use super::models;
        type Response = models::MetricSets;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) metric_name: String,
            pub(crate) group_name: String,
            pub(crate) service_member_id: String,
            pub(crate) group_key: Option<String>,
            pub(crate) from_date: Option<time::OffsetDateTime>,
            pub(crate) to_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The group key"]
            pub fn group_key(mut self, group_key: impl Into<String>) -> Self {
                self.group_key = Some(group_key.into());
                self
            }
            #[doc = "The start date."]
            pub fn from_date(mut self, from_date: impl Into<time::OffsetDateTime>) -> Self {
                self.from_date = Some(from_date.into());
                self
            }
            #[doc = "The end date."]
            pub fn to_date(mut self, to_date: impl Into<time::OffsetDateTime>) -> Self {
                self.to_date = Some(to_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/metrics/{}/groups/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id,
                            &this.metric_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        if let Some(group_key) = &this.group_key {
                            req.url_mut().query_pairs_mut().append_pair("groupKey", group_key);
                        }
                        if let Some(from_date) = &this.from_date {
                            req.url_mut().query_pairs_mut().append_pair("fromDate", &from_date.to_string());
                        }
                        if let Some(to_date) = &this.to_date {
                            req.url_mut().query_pairs_mut().append_pair("toDate", &to_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricSets = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_service_configuration {
        use super::models;
        type Response = models::ServiceConfiguration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/serviceconfiguration",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceConfiguration = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_connector_metadata {
        use super::models;
        type Response = models::ConnectorMetadata;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) service_member_id: String,
            pub(crate) metric_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/servicemembers/{}/metrics/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.service_member_id,
                            &this.metric_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectorMetadata = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod list {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the IP address aggregates for a given service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn ip_address_aggregates_by_service(&self, service_name: impl Into<String>) -> ip_address_aggregates_by_service::Builder {
            ip_address_aggregates_by_service::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                skiptoken: None,
            }
        }
        #[doc = "Gets the IP address aggregate settings."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        pub fn ip_address_aggregate_settings(&self, service_name: impl Into<String>) -> ip_address_aggregate_settings::Builder {
            ip_address_aggregate_settings::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
    }
    pub mod ip_address_aggregates_by_service {
        use super::models;
        type Response = models::IpAddressAggregates;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "A continuationtoken value returned in paginated result to load different pages."]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/ipAddressAggregates",
                            this.client.endpoint(),
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("skiptoken", skiptoken);
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
                                let rsp_value: models::IpAddressAggregates = serde_json::from_slice(&rsp_body)?;
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
    pub mod ip_address_aggregate_settings {
        use super::models;
        type Response = models::IpAddressAggregateSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/ipAddressAggregateSettings",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IpAddressAggregateSetting = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod update {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Updates the IP address aggregate settings alert thresholds."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: The name of the service."]
        #[doc = "* `ip_address_aggregate_setting`: The IP address aggregate setting object."]
        pub fn ip_address_aggregate_settings(
            &self,
            service_name: impl Into<String>,
            ip_address_aggregate_setting: impl Into<models::IpAddressAggregateSetting>,
        ) -> ip_address_aggregate_settings::Builder {
            ip_address_aggregate_settings::Builder {
                client: self.0.clone(),
                service_name: service_name.into(),
                ip_address_aggregate_setting: ip_address_aggregate_setting.into(),
            }
        }
    }
    pub mod ip_address_aggregate_settings {
        use super::models;
        type Response = models::IpAddressAggregateSetting;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service_name: String,
            pub(crate) ip_address_aggregate_setting: models::IpAddressAggregateSetting,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ADHybridHealthService/services/{}/ipAddressAggregateSettings",
                            this.client.endpoint(),
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2014-01-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.ip_address_aggregate_setting)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IpAddressAggregateSetting = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
