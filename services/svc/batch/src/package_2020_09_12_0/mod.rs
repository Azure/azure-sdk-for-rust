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
    pub fn account_client(&self) -> account::Client {
        account::Client(self.clone())
    }
    pub fn application_client(&self) -> application::Client {
        application::Client(self.clone())
    }
    pub fn certificate_client(&self) -> certificate::Client {
        certificate::Client(self.clone())
    }
    pub fn compute_node_client(&self) -> compute_node::Client {
        compute_node::Client(self.clone())
    }
    pub fn file_client(&self) -> file::Client {
        file::Client(self.clone())
    }
    pub fn job_client(&self) -> job::Client {
        job::Client(self.clone())
    }
    pub fn job_schedule_client(&self) -> job_schedule::Client {
        job_schedule::Client(self.clone())
    }
    pub fn pool_client(&self) -> pool::Client {
        pool::Client(self.clone())
    }
    pub fn task_client(&self) -> task::Client {
        task::Client(self.clone())
    }
}
pub mod application {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the applications available in the specified Account."]
        #[doc = "This operation returns only Applications and versions that are available for use on Compute Nodes; that is, that can be used in an Package reference. For administrator information about applications and versions that are not yet available to Compute Nodes, use the Azure portal or the Azure Resource Manager API."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Application."]
        #[doc = "This operation returns only Applications and versions that are available for use on Compute Nodes; that is, that can be used in an Package reference. For administrator information about Applications and versions that are not yet available to Compute Nodes, use the Azure portal or the Azure Resource Manager API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `application_id`: The ID of the Application."]
        pub fn get(&self, application_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                application_id: application_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ApplicationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 applications can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/applications", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::ApplicationListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ApplicationSummary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) application_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/applications/{}", this.client.endpoint(), &this.application_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ApplicationSummary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod pool {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the usage metrics, aggregated by Pool across individual time intervals, for the specified Account."]
        #[doc = "If you do not specify a $filter clause including a poolId, the response includes all Pools that existed in the Account in the time range of the returned aggregation intervals. If you do not specify a $filter clause including a startTime or endTime these filters default to the start and end times of the last aggregation interval currently available; that is, only the last aggregation interval is returned."]
        pub fn list_usage_metrics(&self) -> list_usage_metrics::Builder {
            list_usage_metrics::Builder {
                client: self.0.clone(),
                starttime: None,
                endtime: None,
                filter: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets lifetime summary statistics for all of the Pools in the specified Account."]
        #[doc = "Statistics are aggregated across all Pools that have ever existed in the Account, from Account creation to the last update time of the statistics. The statistics may not be immediately available. The Batch service performs periodic roll-up of statistics. The typical delay is about 30 minutes."]
        pub fn get_all_lifetime_statistics(&self) -> get_all_lifetime_statistics::Builder {
            get_all_lifetime_statistics::Builder {
                client: self.0.clone(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Lists all of the Pools in the specified Account."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                select: None,
                expand: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a Pool to the specified Account."]
        #[doc = "When naming Pools, avoid including sensitive information such as user names or secret project names. This information may appear in telemetry logs accessible to Microsoft Support engineers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool`: The Pool to be added."]
        pub fn add(&self, pool: impl Into<models::PoolAddParameter>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                pool: pool.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to get."]
        pub fn get(&self, pool_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                select: None,
                expand: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Pool."]
        #[doc = "This only replaces the Pool properties specified in the request. For example, if the Pool has a StartTask associated with it, and a request does not specify a StartTask element, then the Pool keeps the existing StartTask."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to update."]
        #[doc = "* `pool_patch_parameter`: The parameters for the request."]
        pub fn patch(&self, pool_id: impl Into<String>, pool_patch_parameter: impl Into<models::PoolPatchParameter>) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                pool_patch_parameter: pool_patch_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes a Pool from the specified Account."]
        #[doc = "When you request that a Pool be deleted, the following actions occur: the Pool state is set to deleting; any ongoing resize operation on the Pool are stopped; the Batch service starts resizing the Pool to zero Compute Nodes; any Tasks running on existing Compute Nodes are terminated and requeued (as if a resize Pool operation had been requested with the default requeue option); finally, the Pool is removed from the system. Because running Tasks are requeued, the user can rerun these Tasks by updating their Job to target a different Pool. The Tasks can then run on the new Pool. If you want to override the requeue behavior, then you should call resize Pool explicitly to shrink the Pool to zero size before deleting the Pool. If you call an Update, Patch or Delete API on a Pool in the deleting state, it will fail with HTTP status code 409 with error code PoolBeingDeleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to delete."]
        pub fn delete(&self, pool_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Gets basic properties of a Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to get."]
        pub fn exists(&self, pool_id: impl Into<String>) -> exists::Builder {
            exists::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Disables automatic scaling for a Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool on which to disable automatic scaling."]
        pub fn disable_auto_scale(&self, pool_id: impl Into<String>) -> disable_auto_scale::Builder {
            disable_auto_scale::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Enables automatic scaling for a Pool."]
        #[doc = "You cannot enable automatic scaling on a Pool if a resize operation is in progress on the Pool. If automatic scaling of the Pool is currently disabled, you must specify a valid autoscale formula as part of the request. If automatic scaling of the Pool is already enabled, you may specify a new autoscale formula and/or a new evaluation interval. You cannot call this API for the same Pool more than once every 30 seconds."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool on which to enable automatic scaling."]
        #[doc = "* `pool_enable_auto_scale_parameter`: The parameters for the request."]
        pub fn enable_auto_scale(
            &self,
            pool_id: impl Into<String>,
            pool_enable_auto_scale_parameter: impl Into<models::PoolEnableAutoScaleParameter>,
        ) -> enable_auto_scale::Builder {
            enable_auto_scale::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                pool_enable_auto_scale_parameter: pool_enable_auto_scale_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Gets the result of evaluating an automatic scaling formula on the Pool."]
        #[doc = "This API is primarily for validating an autoscale formula, as it simply returns the result without applying the formula to the Pool. The Pool must have auto scaling enabled in order to evaluate a formula."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool on which to evaluate the automatic scaling formula."]
        #[doc = "* `pool_evaluate_auto_scale_parameter`: The parameters for the request."]
        pub fn evaluate_auto_scale(
            &self,
            pool_id: impl Into<String>,
            pool_evaluate_auto_scale_parameter: impl Into<models::PoolEvaluateAutoScaleParameter>,
        ) -> evaluate_auto_scale::Builder {
            evaluate_auto_scale::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                pool_evaluate_auto_scale_parameter: pool_evaluate_auto_scale_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Changes the number of Compute Nodes that are assigned to a Pool."]
        #[doc = "You can only resize a Pool when its allocation state is steady. If the Pool is already resizing, the request fails with status code 409. When you resize a Pool, the Pool's allocation state changes from steady to resizing. You cannot resize Pools which are configured for automatic scaling. If you try to do this, the Batch service returns an error 409. If you resize a Pool downwards, the Batch service chooses which Compute Nodes to remove. To remove specific Compute Nodes, use the Pool remove Compute Nodes API instead."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to resize."]
        #[doc = "* `pool_resize_parameter`: The parameters for the request."]
        pub fn resize(&self, pool_id: impl Into<String>, pool_resize_parameter: impl Into<models::PoolResizeParameter>) -> resize::Builder {
            resize::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                pool_resize_parameter: pool_resize_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Stops an ongoing resize operation on the Pool."]
        #[doc = "This does not restore the Pool to its previous state before the resize operation: it only stops any further changes being made, and the Pool maintains its current state. After stopping, the Pool stabilizes at the number of Compute Nodes it was at when the stop operation was done. During the stop operation, the Pool allocation state changes first to stopping and then to steady. A resize operation need not be an explicit resize Pool request; this API can also be used to halt the initial sizing of the Pool when it is created."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool whose resizing you want to stop."]
        pub fn stop_resize(&self, pool_id: impl Into<String>) -> stop_resize::Builder {
            stop_resize::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Pool."]
        #[doc = "This fully replaces all the updatable properties of the Pool. For example, if the Pool has a StartTask associated with it and if StartTask is not specified with this request, then the Batch service will remove the existing StartTask."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool to update."]
        #[doc = "* `pool_update_properties_parameter`: The parameters for the request."]
        pub fn update_properties(
            &self,
            pool_id: impl Into<String>,
            pool_update_properties_parameter: impl Into<models::PoolUpdatePropertiesParameter>,
        ) -> update_properties::Builder {
            update_properties::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                pool_update_properties_parameter: pool_update_properties_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Removes Compute Nodes from the specified Pool."]
        #[doc = "This operation can only run when the allocation state of the Pool is steady. When this operation runs, the allocation state changes from steady to resizing. Each request may remove up to 100 nodes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool from which you want to remove Compute Nodes."]
        #[doc = "* `node_remove_parameter`: The parameters for the request."]
        pub fn remove_nodes(
            &self,
            pool_id: impl Into<String>,
            node_remove_parameter: impl Into<models::NodeRemoveParameter>,
        ) -> remove_nodes::Builder {
            remove_nodes::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_remove_parameter: node_remove_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
    }
    pub mod list_usage_metrics {
        use super::models;
        type Response = models::PoolListUsageMetricsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) starttime: Option<time::OffsetDateTime>,
            pub(crate) endtime: Option<time::OffsetDateTime>,
            pub(crate) filter: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The earliest time from which to include metrics. This must be at least two and a half hours before the current time. If not specified this defaults to the start time of the last aggregation interval currently available."]
            pub fn starttime(mut self, starttime: impl Into<time::OffsetDateTime>) -> Self {
                self.starttime = Some(starttime.into());
                self
            }
            #[doc = "The latest time from which to include metrics. This must be at least two hours before the current time. If not specified this defaults to the end time of the last aggregation interval currently available."]
            pub fn endtime(mut self, endtime: impl Into<time::OffsetDateTime>) -> Self {
                self.endtime = Some(endtime.into());
                self
            }
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-account-usage-metrics."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 results will be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/poolusagemetrics", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(starttime) = &this.starttime {
                                    req.url_mut().query_pairs_mut().append_pair("starttime", &starttime.to_string());
                                }
                                if let Some(endtime) = &this.endtime {
                                    req.url_mut().query_pairs_mut().append_pair("endtime", &endtime.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::PoolListUsageMetricsResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_all_lifetime_statistics {
        use super::models;
        type Response = models::PoolStatistics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/lifetimepoolstats", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PoolStatistics = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::CloudPoolListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-pools."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Pools can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/pools", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudPoolListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool: models::PoolAddParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::CloudPool;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudPool = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) pool_id: String,
            pub(crate) pool_patch_parameter: models::PoolPatchParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool_patch_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
            pub(crate) pool_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod exists {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod disable_auto_scale {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/disableautoscale", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod enable_auto_scale {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) pool_enable_auto_scale_parameter: models::PoolEnableAutoScaleParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/enableautoscale", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool_enable_auto_scale_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod evaluate_auto_scale {
        use super::models;
        type Response = models::AutoScaleRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) pool_evaluate_auto_scale_parameter: models::PoolEvaluateAutoScaleParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/evaluateautoscale", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool_evaluate_auto_scale_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AutoScaleRun = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod resize {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) pool_resize_parameter: models::PoolResizeParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/resize", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool_resize_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod stop_resize {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/stopresize", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod update_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) pool_update_properties_parameter: models::PoolUpdatePropertiesParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/updateproperties", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.pool_update_properties_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod remove_nodes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_remove_parameter: models::NodeRemoveParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/pools/{}/removenodes", this.client.endpoint(), &this.pool_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.node_remove_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
pub mod account {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all Virtual Machine Images supported by the Azure Batch service."]
        pub fn list_supported_images(&self) -> list_supported_images::Builder {
            list_supported_images::Builder {
                client: self.0.clone(),
                filter: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the number of Compute Nodes in each state, grouped by Pool. Note that the numbers returned may not always be up to date. If you need exact node counts, use a list query."]
        pub fn list_pool_node_counts(&self) -> list_pool_node_counts::Builder {
            list_pool_node_counts::Builder {
                client: self.0.clone(),
                filter: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod list_supported_images {
        use super::models;
        type Response = models::AccountListSupportedImagesResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-support-images."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 results will be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/supportedimages", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::AccountListSupportedImagesResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_pool_node_counts {
        use super::models;
        type Response = models::PoolNodeCountsListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The maximum number of items to return in the response."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/nodecounts", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::PoolNodeCountsListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod job {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets lifetime summary statistics for all of the Jobs in the specified Account."]
        #[doc = "Statistics are aggregated across all Jobs that have ever existed in the Account, from Account creation to the last update time of the statistics. The statistics may not be immediately available. The Batch service performs periodic roll-up of statistics. The typical delay is about 30 minutes."]
        pub fn get_all_lifetime_statistics(&self) -> get_all_lifetime_statistics::Builder {
            get_all_lifetime_statistics::Builder {
                client: self.0.clone(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job."]
        pub fn get(&self, job_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                select: None,
                expand: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Job."]
        #[doc = "This fully replaces all the updatable properties of the Job. For example, if the Job has constraints associated with it and if constraints is not specified with this request, then the Batch service will remove the existing constraints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job whose properties you want to update."]
        #[doc = "* `job_update_parameter`: The parameters for the request."]
        pub fn update(&self, job_id: impl Into<String>, job_update_parameter: impl Into<models::JobUpdateParameter>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                job_update_parameter: job_update_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Job."]
        #[doc = "This replaces only the Job properties specified in the request. For example, if the Job has constraints, and a request does not specify the constraints element, then the Job keeps the existing constraints."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job whose properties you want to update."]
        #[doc = "* `job_patch_parameter`: The parameters for the request."]
        pub fn patch(&self, job_id: impl Into<String>, job_patch_parameter: impl Into<models::JobPatchParameter>) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                job_patch_parameter: job_patch_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes a Job."]
        #[doc = "Deleting a Job also deletes all Tasks that are part of that Job, and all Job statistics. This also overrides the retention period for Task data; that is, if the Job contains Tasks which are still retained on Compute Nodes, the Batch services deletes those Tasks' working directories and all their contents.  When a Delete Job request is received, the Batch service sets the Job to the deleting state. All update operations on a Job that is in deleting state will fail with status code 409 (Conflict), with additional information indicating that the Job is being deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to delete."]
        pub fn delete(&self, job_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Disables the specified Job, preventing new Tasks from running."]
        #[doc = "The Batch Service immediately moves the Job to the disabling state. Batch then uses the disableTasks parameter to determine what to do with the currently running Tasks of the Job. The Job remains in the disabling state until the disable operation is completed and all Tasks have been dealt with according to the disableTasks option; the Job then moves to the disabled state. No new Tasks are started under the Job until it moves back to active state. If you try to disable a Job that is in any state other than active, disabling, or disabled, the request fails with status code 409."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to disable."]
        #[doc = "* `job_disable_parameter`: The parameters for the request."]
        pub fn disable(
            &self,
            job_id: impl Into<String>,
            job_disable_parameter: impl Into<models::JobDisableParameter>,
        ) -> disable::Builder {
            disable::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                job_disable_parameter: job_disable_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Enables the specified Job, allowing new Tasks to run."]
        #[doc = "When you call this API, the Batch service sets a disabled Job to the enabling state. After the this operation is completed, the Job moves to the active state, and scheduling of new Tasks under the Job resumes. The Batch service does not allow a Task to remain in the active state for more than 180 days. Therefore, if you enable a Job containing active Tasks which were added more than 180 days ago, those Tasks will not run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to enable."]
        pub fn enable(&self, job_id: impl Into<String>) -> enable::Builder {
            enable::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Terminates the specified Job, marking it as completed."]
        #[doc = "When a Terminate Job request is received, the Batch service sets the Job to the terminating state. The Batch service then terminates any running Tasks associated with the Job and runs any required Job release Tasks. Then the Job moves into the completed state. If there are any Tasks in the Job in the active state, they will remain in the active state. Once a Job is terminated, new Tasks cannot be added and any remaining active Tasks will not be scheduled."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to terminate."]
        pub fn terminate(&self, job_id: impl Into<String>) -> terminate::Builder {
            terminate::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                job_terminate_parameter: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Lists all of the Jobs in the specified Account."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                select: None,
                expand: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a Job to the specified Account."]
        #[doc = "The Batch service supports two ways to control the work done as part of a Job. In the first approach, the user specifies a Job Manager Task. The Batch service launches this Task when it is ready to start the Job. The Job Manager Task controls all other Tasks that run under this Job, by using the Task APIs. In the second approach, the user directly controls the execution of Tasks under an active Job, by using the Task APIs. Also note: when naming Jobs, avoid including sensitive information such as user names or secret project names. This information may appear in telemetry logs accessible to Microsoft Support engineers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job`: The Job to be added."]
        pub fn add(&self, job: impl Into<models::JobAddParameter>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                job: job.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Lists the Jobs that have been created under the specified Job Schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule from which you want to get a list of Jobs."]
        pub fn list_from_job_schedule(&self, job_schedule_id: impl Into<String>) -> list_from_job_schedule::Builder {
            list_from_job_schedule::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                filter: None,
                select: None,
                expand: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Lists the execution status of the Job Preparation and Job Release Task for the specified Job across the Compute Nodes where the Job has run."]
        #[doc = "This API returns the Job Preparation and Job Release Task status on all Compute Nodes that have run the Job Preparation or Job Release Task. This includes Compute Nodes which have since been removed from the Pool. If this API is invoked on a Job which has no Job Preparation or Job Release Task, the Batch service returns HTTP status code 409 (Conflict) with an error code of JobPreparationTaskNotSpecified."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job."]
        pub fn list_preparation_and_release_task_status(
            &self,
            job_id: impl Into<String>,
        ) -> list_preparation_and_release_task_status::Builder {
            list_preparation_and_release_task_status::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                filter: None,
                select: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the Task counts for the specified Job."]
        #[doc = "Task counts provide a count of the Tasks by active, running or completed Task state, and a count of Tasks which succeeded or failed. Tasks in the preparing state are counted as running. Note that the numbers returned may not always be up to date. If you need exact task counts, use a list query."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job."]
        pub fn get_task_counts(&self, job_id: impl Into<String>) -> get_task_counts::Builder {
            get_task_counts::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod get_all_lifetime_statistics {
        use super::models;
        type Response = models::JobStatistics;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/lifetimejobstats", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JobStatistics = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::CloudJob;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudJob = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) job_id: String,
            pub(crate) job_update_parameter: models::JobUpdateParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job_update_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod patch {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) job_patch_parameter: models::JobPatchParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job_patch_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
            pub(crate) job_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod disable {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) job_disable_parameter: models::JobDisableParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/disable", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job_disable_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod enable {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/enable", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod terminate {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) job_terminate_parameter: Option<models::JobTerminateParameter>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The parameters for the request."]
            pub fn job_terminate_parameter(mut self, job_terminate_parameter: impl Into<models::JobTerminateParameter>) -> Self {
                self.job_terminate_parameter = Some(job_terminate_parameter.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/terminate", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        let req_body = if let Some(job_terminate_parameter) = &this.job_terminate_parameter {
                            req.insert_header("content-type", "application/json; odata=minimalmetadata");
                            azure_core::to_json(job_terminate_parameter)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod list {
        use super::models;
        type Response = models::CloudJobListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-jobs."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Jobs can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/jobs", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudJobListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job: models::JobAddParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_from_job_schedule {
        use super::models;
        type Response = models::CloudJobListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-jobs-in-a-job-schedule."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Jobs can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/jobschedules/{}/jobs", this.client.endpoint(), &this.job_schedule_id))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudJobListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_preparation_and_release_task_status {
        use super::models;
        type Response = models::CloudJobListPreparationAndReleaseTaskStatusResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-job-preparation-and-release-status."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Tasks can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/jobpreparationandreleasetaskstatus",
                            this.client.endpoint(),
                            &this.job_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudJobListPreparationAndReleaseTaskStatusResult =
                                    serde_json::from_slice(&rsp_body)?;
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
    pub mod get_task_counts {
        use super::models;
        type Response = models::TaskCountsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/taskcounts", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskCountsResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod certificate {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the Certificates that have been added to the specified Account."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                select: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a Certificate to the specified Account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `certificate`: The Certificate to be added."]
        pub fn add(&self, certificate: impl Into<models::CertificateAddParameter>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                certificate: certificate.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Cancels a failed deletion of a Certificate from the specified Account."]
        #[doc = "If you try to delete a Certificate that is being used by a Pool or Compute Node, the status of the Certificate changes to deleteFailed. If you decide that you want to continue using the Certificate, you can use this operation to set the status of the Certificate back to active. If you intend to delete the Certificate, you do not need to run this operation after the deletion failed. You must make sure that the Certificate is not being used by any resources, and then you can try again to delete the Certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `thumbprint_algorithm`: The algorithm used to derive the thumbprint parameter. This must be sha1."]
        #[doc = "* `thumbprint`: The thumbprint of the Certificate being deleted."]
        pub fn cancel_deletion(&self, thumbprint_algorithm: impl Into<String>, thumbprint: impl Into<String>) -> cancel_deletion::Builder {
            cancel_deletion::Builder {
                client: self.0.clone(),
                thumbprint_algorithm: thumbprint_algorithm.into(),
                thumbprint: thumbprint.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `thumbprint_algorithm`: The algorithm used to derive the thumbprint parameter. This must be sha1."]
        #[doc = "* `thumbprint`: The thumbprint of the Certificate to get."]
        pub fn get(&self, thumbprint_algorithm: impl Into<String>, thumbprint: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                thumbprint_algorithm: thumbprint_algorithm.into(),
                thumbprint: thumbprint.into(),
                select: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Deletes a Certificate from the specified Account."]
        #[doc = "You cannot delete a Certificate if a resource (Pool or Compute Node) is using it. Before you can delete a Certificate, you must therefore make sure that the Certificate is not associated with any existing Pools, the Certificate is not installed on any Nodes (even if you remove a Certificate from a Pool, it is not removed from existing Compute Nodes in that Pool until they restart), and no running Tasks depend on the Certificate. If you try to delete a Certificate that is in use, the deletion fails. The Certificate status changes to deleteFailed. You can use Cancel Delete Certificate to set the status back to active if you decide that you want to continue using the Certificate."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `thumbprint_algorithm`: The algorithm used to derive the thumbprint parameter. This must be sha1."]
        #[doc = "* `thumbprint`: The thumbprint of the Certificate to be deleted."]
        pub fn delete(&self, thumbprint_algorithm: impl Into<String>, thumbprint: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                thumbprint_algorithm: thumbprint_algorithm.into(),
                thumbprint: thumbprint.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CertificateListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-certificates."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Certificates can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/certificates", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CertificateListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) certificate: models::CertificateAddParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/certificates", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.certificate)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod cancel_deletion {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) thumbprint_algorithm: String,
            pub(crate) thumbprint: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/certificates(thumbprintAlgorithm={},thumbprint={})/canceldelete",
                            this.client.endpoint(),
                            &this.thumbprint_algorithm,
                            &this.thumbprint
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod get {
        use super::models;
        type Response = models::Certificate;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) thumbprint_algorithm: String,
            pub(crate) thumbprint: String,
            pub(crate) select: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/certificates(thumbprintAlgorithm={},thumbprint={})",
                            this.client.endpoint(),
                            &this.thumbprint_algorithm,
                            &this.thumbprint
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Certificate = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) thumbprint_algorithm: String,
            pub(crate) thumbprint: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/certificates(thumbprintAlgorithm={},thumbprint={})",
                            this.client.endpoint(),
                            &this.thumbprint_algorithm,
                            &this.thumbprint
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
pub mod file {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the content of the specified Task file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job that contains the Task."]
        #[doc = "* `task_id`: The ID of the Task whose file you want to retrieve."]
        #[doc = "* `file_path`: The path to the Task file that you want to get the content of."]
        pub fn get_from_task(
            &self,
            job_id: impl Into<String>,
            task_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_from_task::Builder {
            get_from_task::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                file_path: file_path.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                ocp_range: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes the specified Task file from the Compute Node where the Task ran."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job that contains the Task."]
        #[doc = "* `task_id`: The ID of the Task whose file you want to delete."]
        #[doc = "* `file_path`: The path to the Task file or directory that you want to delete."]
        pub fn delete_from_task(
            &self,
            job_id: impl Into<String>,
            task_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> delete_from_task::Builder {
            delete_from_task::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                file_path: file_path.into(),
                recursive: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the properties of the specified Task file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job that contains the Task."]
        #[doc = "* `task_id`: The ID of the Task whose file you want to get the properties of."]
        #[doc = "* `file_path`: The path to the Task file that you want to get the properties of."]
        pub fn get_properties_from_task(
            &self,
            job_id: impl Into<String>,
            task_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_properties_from_task::Builder {
            get_properties_from_task::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                file_path: file_path.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Returns the content of the specified Compute Node file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node that contains the file."]
        #[doc = "* `file_path`: The path to the Compute Node file that you want to get the content of."]
        pub fn get_from_compute_node(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_from_compute_node::Builder {
            get_from_compute_node::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                file_path: file_path.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                ocp_range: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes the specified file from the Compute Node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node from which you want to delete the file."]
        #[doc = "* `file_path`: The path to the file or directory that you want to delete."]
        pub fn delete_from_compute_node(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> delete_from_compute_node::Builder {
            delete_from_compute_node::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                file_path: file_path.into(),
                recursive: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the properties of the specified Compute Node file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node that contains the file."]
        #[doc = "* `file_path`: The path to the Compute Node file that you want to get the properties of."]
        pub fn get_properties_from_compute_node(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            file_path: impl Into<String>,
        ) -> get_properties_from_compute_node::Builder {
            get_properties_from_compute_node::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                file_path: file_path.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Lists the files in a Task's directory on its Compute Node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job that contains the Task."]
        #[doc = "* `task_id`: The ID of the Task whose files you want to list."]
        pub fn list_from_task(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> list_from_task::Builder {
            list_from_task::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                filter: None,
                recursive: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Lists all of the files in Task directories on the specified Compute Node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node whose files you want to list."]
        pub fn list_from_compute_node(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> list_from_compute_node::Builder {
            list_from_compute_node::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                filter: None,
                recursive: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod get_from_task {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) file_path: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) ocp_range: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "The byte range to be retrieved. The default is to retrieve the entire file. The format is bytes=startRange-endRange."]
            pub fn ocp_range(mut self, ocp_range: impl Into<String>) -> Self {
                self.ocp_range = Some(ocp_range.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/files/{}",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id,
                            &this.file_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(ocp_range) = &this.ocp_range {
                            req.insert_header("ocp-range", ocp_range);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_from_task {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) file_path: String,
            pub(crate) recursive: Option<bool>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "Whether to delete children of a directory. If the filePath parameter represents a directory instead of a file, you can set recursive to true to delete the directory and all of the files and subdirectories in it. If recursive is false then the directory must be empty or deletion will fail."]
            pub fn recursive(mut self, recursive: bool) -> Self {
                self.recursive = Some(recursive);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/files/{}",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id,
                            &this.file_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(recursive) = &this.recursive {
                            req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
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
    pub mod get_properties_from_task {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) file_path: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/files/{}",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id,
                            &this.file_path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod get_from_compute_node {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) file_path: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) ocp_range: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "The byte range to be retrieved. The default is to retrieve the entire file. The format is bytes=startRange-endRange."]
            pub fn ocp_range(mut self, ocp_range: impl Into<String>) -> Self {
                self.ocp_range = Some(ocp_range.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/files/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id,
                            &this.file_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(ocp_range) = &this.ocp_range {
                            req.insert_header("ocp-range", ocp_range);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_from_compute_node {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) file_path: String,
            pub(crate) recursive: Option<bool>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "Whether to delete children of a directory. If the filePath parameter represents a directory instead of a file, you can set recursive to true to delete the directory and all of the files and subdirectories in it. If recursive is false then the directory must be empty or deletion will fail."]
            pub fn recursive(mut self, recursive: bool) -> Self {
                self.recursive = Some(recursive);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/files/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id,
                            &this.file_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(recursive) = &this.recursive {
                            req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
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
    pub mod get_properties_from_compute_node {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) file_path: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/files/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id,
                            &this.file_path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod list_from_task {
        use super::models;
        type Response = models::NodeFileListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) recursive: Option<bool>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-task-files."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Whether to list children of the Task directory. This parameter can be used in combination with the filter parameter to list specific type of files."]
            pub fn recursive(mut self, recursive: bool) -> Self {
                self.recursive = Some(recursive);
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 files can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/files",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(recursive) = &this.recursive {
                                    req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::NodeFileListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_from_compute_node {
        use super::models;
        type Response = models::NodeFileListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) recursive: Option<bool>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-compute-node-files."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Whether to list children of a directory."]
            pub fn recursive(mut self, recursive: bool) -> Self {
                self.recursive = Some(recursive);
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 files can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/files",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(recursive) = &this.recursive {
                                    req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::NodeFileListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod job_schedule {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets information about the specified Job Schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to get."]
        pub fn get(&self, job_schedule_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                select: None,
                expand: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Job Schedule."]
        #[doc = "This fully replaces all the updatable properties of the Job Schedule. For example, if the schedule property is not specified with this request, then the Batch service will remove the existing schedule. Changes to a Job Schedule only impact Jobs created by the schedule after the update has taken place; currently running Jobs are unaffected."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to update."]
        #[doc = "* `job_schedule_update_parameter`: The parameters for the request."]
        pub fn update(
            &self,
            job_schedule_id: impl Into<String>,
            job_schedule_update_parameter: impl Into<models::JobScheduleUpdateParameter>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                job_schedule_update_parameter: job_schedule_update_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Job Schedule."]
        #[doc = "This replaces only the Job Schedule properties specified in the request. For example, if the schedule property is not specified with this request, then the Batch service will keep the existing schedule. Changes to a Job Schedule only impact Jobs created by the schedule after the update has taken place; currently running Jobs are unaffected."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to update."]
        #[doc = "* `job_schedule_patch_parameter`: The parameters for the request."]
        pub fn patch(
            &self,
            job_schedule_id: impl Into<String>,
            job_schedule_patch_parameter: impl Into<models::JobSchedulePatchParameter>,
        ) -> patch::Builder {
            patch::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                job_schedule_patch_parameter: job_schedule_patch_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes a Job Schedule from the specified Account."]
        #[doc = "When you delete a Job Schedule, this also deletes all Jobs and Tasks under that schedule. When Tasks are deleted, all the files in their working directories on the Compute Nodes are also deleted (the retention period is ignored). The Job Schedule statistics are no longer accessible once the Job Schedule is deleted, though they are still counted towards Account lifetime statistics."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to delete."]
        pub fn delete(&self, job_schedule_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Checks the specified Job Schedule exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule which you want to check."]
        pub fn exists(&self, job_schedule_id: impl Into<String>) -> exists::Builder {
            exists::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Disables a Job Schedule."]
        #[doc = "No new Jobs will be created until the Job Schedule is enabled again."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to disable."]
        pub fn disable(&self, job_schedule_id: impl Into<String>) -> disable::Builder {
            disable::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Enables a Job Schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to enable."]
        pub fn enable(&self, job_schedule_id: impl Into<String>) -> enable::Builder {
            enable::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Terminates a Job Schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_schedule_id`: The ID of the Job Schedule to terminates."]
        pub fn terminate(&self, job_schedule_id: impl Into<String>) -> terminate::Builder {
            terminate::Builder {
                client: self.0.clone(),
                job_schedule_id: job_schedule_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Lists all of the Job Schedules in the specified Account."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                filter: None,
                select: None,
                expand: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a Job Schedule to the specified Account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `cloud_job_schedule`: The Job Schedule to be added."]
        pub fn add(&self, cloud_job_schedule: impl Into<models::JobScheduleAddParameter>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                cloud_job_schedule: cloud_job_schedule.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::CloudJobSchedule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules/{}", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudJobSchedule = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) job_schedule_id: String,
            pub(crate) job_schedule_update_parameter: models::JobScheduleUpdateParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules/{}", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job_schedule_update_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod patch {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) job_schedule_patch_parameter: models::JobSchedulePatchParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules/{}", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.job_schedule_patch_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
            pub(crate) job_schedule_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules/{}", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod exists {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules/{}", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod disable {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobschedules/{}/disable",
                            this.client.endpoint(),
                            &this.job_schedule_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod enable {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/jobschedules/{}/enable", this.client.endpoint(), &this.job_schedule_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod terminate {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_schedule_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobschedules/{}/terminate",
                            this.client.endpoint(),
                            &this.job_schedule_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod list {
        use super::models;
        type Response = models::CloudJobScheduleListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-job-schedules."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Job Schedules can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/jobschedules", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudJobScheduleListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) cloud_job_schedule: models::JobScheduleAddParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobschedules", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.cloud_job_schedule)?;
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod task {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the Tasks that are associated with the specified Job."]
        #[doc = "For multi-instance Tasks, information such as affinityId, executionInfo and nodeInfo refer to the primary Task. Use the list subtasks API to retrieve information about subtasks."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job."]
        pub fn list(&self, job_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                filter: None,
                select: None,
                expand: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a Task to the specified Job."]
        #[doc = "The maximum lifetime of a Task from addition to completion is 180 days. If a Task has not completed within 180 days of being added it will be terminated by the Batch service and left in whatever state it was in at that time."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to which the Task is to be added."]
        #[doc = "* `task`: The Task to be added."]
        pub fn add(&self, job_id: impl Into<String>, task: impl Into<models::TaskAddParameter>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task: task.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Adds a collection of Tasks to the specified Job."]
        #[doc = "Note that each Task must have a unique ID. The Batch service may not return the results for each Task in the same order the Tasks were submitted in this request. If the server times out or the connection is closed during the request, the request may have been partially or fully processed, or not at all. In such cases, the user should re-issue the request. Note that it is up to the user to correctly handle failures when re-issuing a request. For example, you should use the same Task IDs during a retry so that if the prior operation succeeded, the retry will not create extra Tasks unexpectedly. If the response contains any Tasks which failed to add, a client can retry the request. In a retry, it is most efficient to resubmit only Tasks that failed to add, and to omit Tasks that were successfully added on the first attempt. The maximum lifetime of a Task from addition to completion is 180 days. If a Task has not completed within 180 days of being added it will be terminated by the Batch service and left in whatever state it was in at that time."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job to which the Task collection is to be added."]
        #[doc = "* `task_collection`: The Tasks to be added."]
        pub fn add_collection(
            &self,
            job_id: impl Into<String>,
            task_collection: impl Into<models::TaskAddCollectionParameter>,
        ) -> add_collection::Builder {
            add_collection::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_collection: task_collection.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Task."]
        #[doc = "For multi-instance Tasks, information such as affinityId, executionInfo and nodeInfo refer to the primary Task. Use the list subtasks API to retrieve information about subtasks."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job that contains the Task."]
        #[doc = "* `task_id`: The ID of the Task to get information about."]
        pub fn get(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                select: None,
                expand: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Updates the properties of the specified Task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job containing the Task."]
        #[doc = "* `task_id`: The ID of the Task to update."]
        #[doc = "* `task_update_parameter`: The parameters for the request."]
        pub fn update(
            &self,
            job_id: impl Into<String>,
            task_id: impl Into<String>,
            task_update_parameter: impl Into<models::TaskUpdateParameter>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                task_update_parameter: task_update_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Deletes a Task from the specified Job."]
        #[doc = "When a Task is deleted, all of the files in its directory on the Compute Node where it ran are also deleted (regardless of the retention time). For multi-instance Tasks, the delete Task operation applies synchronously to the primary task; subtasks and their files are then deleted asynchronously in the background."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job from which to delete the Task."]
        #[doc = "* `task_id`: The ID of the Task to delete."]
        pub fn delete(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Lists all of the subtasks that are associated with the specified multi-instance Task."]
        #[doc = "If the Task is not a multi-instance Task then this returns an empty collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job."]
        #[doc = "* `task_id`: The ID of the Task."]
        pub fn list_subtasks(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> list_subtasks::Builder {
            list_subtasks::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                select: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Terminates the specified Task."]
        #[doc = "When the Task has been terminated, it moves to the completed state. For multi-instance Tasks, the terminate Task operation applies synchronously to the primary task; subtasks are then terminated asynchronously in the background."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job containing the Task."]
        #[doc = "* `task_id`: The ID of the Task to terminate."]
        pub fn terminate(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> terminate::Builder {
            terminate::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Reactivates a Task, allowing it to run again even if its retry count has been exhausted."]
        #[doc = "Reactivation makes a Task eligible to be retried again up to its maximum retry count. The Task's state is changed to active. As the Task is no longer in the completed state, any previous exit code or failure information is no longer available after reactivation. Each time a Task is reactivated, its retry count is reset to 0. Reactivation will fail for Tasks that are not completed or that previously completed successfully (with an exit code of 0). Additionally, it will fail if the Job has completed (or is terminating or deleting)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `job_id`: The ID of the Job containing the Task."]
        #[doc = "* `task_id`: The ID of the Task to reactivate."]
        pub fn reactivate(&self, job_id: impl Into<String>, task_id: impl Into<String>) -> reactivate::Builder {
            reactivate::Builder {
                client: self.0.clone(),
                job_id: job_id.into(),
                task_id: task_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CloudTaskListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-tasks."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Tasks can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/jobs/{}/tasks", this.client.endpoint(), &this.job_id))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::CloudTaskListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task: models::TaskAddParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/tasks", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.task)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_collection {
        use super::models;
        type Response = models::TaskAddCollectionResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_collection: models::TaskAddCollectionParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/jobs/{}/addtaskcollection", this.client.endpoint(), &this.job_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.task_collection)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskAddCollectionResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::CloudTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) select: Option<String>,
            pub(crate) expand: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "An OData $expand clause."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/jobs/{}/tasks/{}", this.client.endpoint(), &this.job_id, &this.task_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudTask = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) task_update_parameter: models::TaskUpdateParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/jobs/{}/tasks/{}", this.client.endpoint(), &this.job_id, &this.task_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.task_update_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/jobs/{}/tasks/{}", this.client.endpoint(), &this.job_id, &this.task_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
    pub mod list_subtasks {
        use super::models;
        type Response = models::CloudTaskListSubtasksResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) select: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/subtasksinfo",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CloudTaskListSubtasksResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod terminate {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/terminate",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
    pub mod reactivate {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) job_id: String,
            pub(crate) task_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service exactly matches the value specified by the client."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "An ETag value associated with the version of the resource known to the client. The operation will be performed only if the resource's current ETag on the service does not match the value specified by the client."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has been modified since the specified time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "A timestamp indicating the last modified time of the resource known to the client. The operation will be performed only if the resource on the service has not been modified since the specified time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/jobs/{}/tasks/{}/reactivate",
                            this.client.endpoint(),
                            &this.job_id,
                            &this.task_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
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
}
pub mod compute_node {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Adds a user Account to the specified Compute Node."]
        #[doc = "You can add a user Account to a Compute Node only when it is in the idle or running state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the machine on which you want to create a user Account."]
        #[doc = "* `user`: The user Account to be created."]
        pub fn add_user(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            user: impl Into<models::ComputeNodeUser>,
        ) -> add_user::Builder {
            add_user::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                user: user.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Updates the password and expiration time of a user Account on the specified Compute Node."]
        #[doc = "This operation replaces of all the updatable properties of the Account. For example, if the expiryTime element is not specified, the current value is replaced with the default value, not left unmodified. You can update a user Account on a Compute Node only when it is in the idle or running state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the machine on which you want to update a user Account."]
        #[doc = "* `user_name`: The name of the user Account to update."]
        #[doc = "* `node_update_user_parameter`: The parameters for the request."]
        pub fn update_user(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            user_name: impl Into<String>,
            node_update_user_parameter: impl Into<models::NodeUpdateUserParameter>,
        ) -> update_user::Builder {
            update_user::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                user_name: user_name.into(),
                node_update_user_parameter: node_update_user_parameter.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Deletes a user Account from the specified Compute Node."]
        #[doc = "You can delete a user Account to a Compute Node only when it is in the idle or running state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the machine on which you want to delete a user Account."]
        #[doc = "* `user_name`: The name of the user Account to delete."]
        pub fn delete_user(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            user_name: impl Into<String>,
        ) -> delete_user::Builder {
            delete_user::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                user_name: user_name.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets information about the specified Compute Node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node that you want to get information about."]
        pub fn get(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                select: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Restarts the specified Compute Node."]
        #[doc = "You can restart a Compute Node only if it is in an idle or running state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node that you want to restart."]
        pub fn reboot(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> reboot::Builder {
            reboot::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                node_reboot_parameter: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Reinstalls the operating system on the specified Compute Node."]
        #[doc = "You can reinstall the operating system on a Compute Node only if it is in an idle or running state. This API can be invoked only on Pools created with the cloud service configuration property."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node that you want to restart."]
        pub fn reimage(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> reimage::Builder {
            reimage::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                node_reimage_parameter: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Disables Task scheduling on the specified Compute Node."]
        #[doc = "You can disable Task scheduling on a Compute Node only if its current scheduling state is enabled."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node on which you want to disable Task scheduling."]
        pub fn disable_scheduling(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> disable_scheduling::Builder {
            disable_scheduling::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                node_disable_scheduling_parameter: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Enables Task scheduling on the specified Compute Node."]
        #[doc = "You can enable Task scheduling on a Compute Node only if its current scheduling state is disabled"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node on which you want to enable Task scheduling."]
        pub fn enable_scheduling(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> enable_scheduling::Builder {
            enable_scheduling::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the settings required for remote login to a Compute Node."]
        #[doc = "Before you can remotely login to a Compute Node using the remote login settings, you must create a user Account on the Compute Node. This API can be invoked only on Pools created with the virtual machine configuration property. For Pools created with a cloud service configuration, see the GetRemoteDesktop API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node for which to obtain the remote login settings."]
        pub fn get_remote_login_settings(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
        ) -> get_remote_login_settings::Builder {
            get_remote_login_settings::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Gets the Remote Desktop Protocol file for the specified Compute Node."]
        #[doc = "Before you can access a Compute Node by using the RDP file, you must create a user Account on the Compute Node. This API can only be invoked on Pools created with a cloud service configuration. For Pools created with a virtual machine configuration, see the GetRemoteLoginSettings API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node for which you want to get the Remote Desktop Protocol file."]
        pub fn get_remote_desktop(&self, pool_id: impl Into<String>, node_id: impl Into<String>) -> get_remote_desktop::Builder {
            get_remote_desktop::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Upload Azure Batch service log files from the specified Compute Node to Azure Blob Storage."]
        #[doc = "This is for gathering Azure Batch service log files in an automated fashion from Compute Nodes if you are experiencing an error and wish to escalate to Azure support. The Azure Batch service log files should be shared with Azure support to aid in debugging issues with the Batch service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool that contains the Compute Node."]
        #[doc = "* `node_id`: The ID of the Compute Node from which you want to upload the Azure Batch service log files."]
        #[doc = "* `upload_batch_service_logs_configuration`: The Azure Batch service log files upload configuration."]
        pub fn upload_batch_service_logs(
            &self,
            pool_id: impl Into<String>,
            node_id: impl Into<String>,
            upload_batch_service_logs_configuration: impl Into<models::UploadBatchServiceLogsConfiguration>,
        ) -> upload_batch_service_logs::Builder {
            upload_batch_service_logs::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                node_id: node_id.into(),
                upload_batch_service_logs_configuration: upload_batch_service_logs_configuration.into(),
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
        #[doc = "Lists the Compute Nodes in the specified Pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `pool_id`: The ID of the Pool from which you want to list Compute Nodes."]
        pub fn list(&self, pool_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                pool_id: pool_id.into(),
                filter: None,
                select: None,
                maxresults: None,
                timeout: None,
                client_request_id: None,
                return_client_request_id: None,
                ocp_date: None,
            }
        }
    }
    pub mod add_user {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) user: models::ComputeNodeUser,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/users",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.user)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_user {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) user_name: String,
            pub(crate) node_update_user_parameter: models::NodeUpdateUserParameter,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/users/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.node_update_user_parameter)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod delete_user {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) user_name: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/users/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
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
    pub mod get {
        use super::models;
        type Response = models::ComputeNode;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) select: Option<String>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ComputeNode = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod reboot {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) node_reboot_parameter: Option<models::NodeRebootParameter>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The parameters for the request."]
            pub fn node_reboot_parameter(mut self, node_reboot_parameter: impl Into<models::NodeRebootParameter>) -> Self {
                self.node_reboot_parameter = Some(node_reboot_parameter.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/reboot",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        let req_body = if let Some(node_reboot_parameter) = &this.node_reboot_parameter {
                            req.insert_header("content-type", "application/json; odata=minimalmetadata");
                            azure_core::to_json(node_reboot_parameter)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod reimage {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) node_reimage_parameter: Option<models::NodeReimageParameter>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The parameters for the request."]
            pub fn node_reimage_parameter(mut self, node_reimage_parameter: impl Into<models::NodeReimageParameter>) -> Self {
                self.node_reimage_parameter = Some(node_reimage_parameter.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/reimage",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        let req_body = if let Some(node_reimage_parameter) = &this.node_reimage_parameter {
                            req.insert_header("content-type", "application/json; odata=minimalmetadata");
                            azure_core::to_json(node_reimage_parameter)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod disable_scheduling {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) node_disable_scheduling_parameter: Option<models::NodeDisableSchedulingParameter>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The parameters for the request."]
            pub fn node_disable_scheduling_parameter(
                mut self,
                node_disable_scheduling_parameter: impl Into<models::NodeDisableSchedulingParameter>,
            ) -> Self {
                self.node_disable_scheduling_parameter = Some(node_disable_scheduling_parameter.into());
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/disablescheduling",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        let req_body = if let Some(node_disable_scheduling_parameter) = &this.node_disable_scheduling_parameter {
                            req.insert_header("content-type", "application/json; odata=minimalmetadata");
                            azure_core::to_json(node_disable_scheduling_parameter)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
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
    pub mod enable_scheduling {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/enablescheduling",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod get_remote_login_settings {
        use super::models;
        type Response = models::ComputeNodeGetRemoteLoginSettingsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/remoteloginsettings",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ComputeNodeGetRemoteLoginSettingsResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_remote_desktop {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/rdp",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod upload_batch_service_logs {
        use super::models;
        type Response = models::UploadBatchServiceLogsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) node_id: String,
            pub(crate) upload_batch_service_logs_configuration: models::UploadBatchServiceLogsConfiguration,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/pools/{}/nodes/{}/uploadbatchservicelogs",
                            this.client.endpoint(),
                            &this.pool_id,
                            &this.node_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(client_request_id) = &this.client_request_id {
                            req.insert_header("client-request-id", client_request_id);
                        }
                        if let Some(return_client_request_id) = &this.return_client_request_id {
                            req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                        }
                        if let Some(ocp_date) = &this.ocp_date {
                            req.insert_header("ocp-date", &ocp_date.to_string());
                        }
                        req.insert_header("content-type", "application/json; odata=minimalmetadata");
                        let req_body = azure_core::to_json(&this.upload_batch_service_logs_configuration)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UploadBatchServiceLogsResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::ComputeNodeListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) pool_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) maxresults: Option<i32>,
            pub(crate) timeout: Option<i32>,
            pub(crate) client_request_id: Option<String>,
            pub(crate) return_client_request_id: Option<bool>,
            pub(crate) ocp_date: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "An OData $filter clause. For more information on constructing this filter, see https://docs.microsoft.com/en-us/rest/api/batchservice/odata-filters-in-batch#list-nodes-in-a-pool."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An OData $select clause."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The maximum number of items to return in the response. A maximum of 1000 Compute Nodes can be returned."]
            pub fn maxresults(mut self, maxresults: i32) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The maximum time that the server can spend processing the request, in seconds. The default is 30 seconds."]
            pub fn timeout(mut self, timeout: i32) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The caller-generated request identity, in the form of a GUID with no decoration such as curly braces, e.g. 9C4D50EE-2D56-4CD3-8152-34347DC9F2B0."]
            pub fn client_request_id(mut self, client_request_id: impl Into<String>) -> Self {
                self.client_request_id = Some(client_request_id.into());
                self
            }
            #[doc = "Whether the server should return the client-request-id in the response."]
            pub fn return_client_request_id(mut self, return_client_request_id: bool) -> Self {
                self.return_client_request_id = Some(return_client_request_id);
                self
            }
            #[doc = "The time the request was issued. Client libraries typically set this to the current system clock time; set it explicitly if you are calling the REST API directly."]
            pub fn ocp_date(mut self, ocp_date: impl Into<time::OffsetDateTime>) -> Self {
                self.ocp_date = Some(ocp_date.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/pools/{}/nodes", this.client.endpoint(), &this.pool_id))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-09-01.12.0");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(client_request_id) = &this.client_request_id {
                                    req.insert_header("client-request-id", client_request_id);
                                }
                                if let Some(return_client_request_id) = &this.return_client_request_id {
                                    req.insert_header("return-client-request-id", &return_client_request_id.to_string());
                                }
                                if let Some(ocp_date) = &this.ocp_date {
                                    req.insert_header("ocp-date", &ocp_date.to_string());
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
                                let rsp_value: models::ComputeNodeListResult = serde_json::from_slice(&rsp_body)?;
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
