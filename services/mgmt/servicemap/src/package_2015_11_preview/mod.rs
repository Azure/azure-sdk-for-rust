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
    pub fn client_groups_client(&self) -> client_groups::Client {
        client_groups::Client(self.clone())
    }
    pub fn machine_groups_client(&self) -> machine_groups::Client {
        machine_groups::Client(self.clone())
    }
    pub fn machines_client(&self) -> machines::Client {
        machines::Client(self.clone())
    }
    pub fn maps_client(&self) -> maps::Client {
        maps::Client(self.clone())
    }
    pub fn ports_client(&self) -> ports::Client {
        ports::Client(self.clone())
    }
    pub fn processes_client(&self) -> processes::Client {
        processes::Client(self.clone())
    }
    pub fn summaries_client(&self) -> summaries::Client {
        summaries::Client(self.clone())
    }
}
pub mod machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a collection of machines matching the specified conditions.  The returned collection represents either machines that are active/live during the specified interval  of time (`live=true` and `startTime`/`endTime` are specified) or that are known to have existed at or  some time prior to the specified point in time (`live=false` and `timestamp` is specified)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        pub fn list_by_workspace(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> list_by_workspace::Builder {
            list_by_workspace::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                live: None,
                start_time: None,
                end_time: None,
                timestamp: None,
                top: None,
            }
        }
        #[doc = "Returns the specified machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                timestamp: None,
            }
        }
        #[doc = "Obtains the liveness status of the machine during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn get_liveness(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get_liveness::Builder {
            get_liveness::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of connections terminating or originating at the specified machine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn list_connections(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> list_connections::Builder {
            list_connections::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of processes on the specified machine matching the specified conditions. The returned collection represents either processes that are active/live during the specified interval  of time (`live=true` and `startTime`/`endTime` are specified) or that are known to have existed at or  some time prior to the specified point in time (`live=false` and `timestamp` is specified).        "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn list_processes(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> list_processes::Builder {
            list_processes::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                live: None,
                start_time: None,
                end_time: None,
                timestamp: None,
            }
        }
        #[doc = "Returns a collection of live ports on the specified machine during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn list_ports(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> list_ports::Builder {
            list_ports::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of machine groups this machine belongs to during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        pub fn list_machine_group_membership(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> list_machine_group_membership::Builder {
            list_machine_group_membership::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                start_time: None,
                end_time: None,
            }
        }
    }
    pub mod list_by_workspace {
        use super::models;
        type Response = models::MachineCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) live: Option<bool>,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
            pub(crate) timestamp: Option<time::OffsetDateTime>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Specifies whether to return live resources (true) or inventory resources (false). Defaults to **true**. When retrieving live resources, the start time (`startTime`) and end time (`endTime`) of the desired interval should be included. When retrieving inventory resources, an optional timestamp (`timestamp`) parameter can be specified to return the version of each resource closest (not-after) that timestamp."]
            pub fn live(mut self, live: bool) -> Self {
                self.live = Some(live);
                self
            }
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            #[doc = "UTC date and time specifying a time instance relative to which to evaluate each machine resource. Only applies when `live=false`. When not specified, the service uses DateTime.UtcNow."]
            pub fn timestamp(mut self, timestamp: impl Into<time::OffsetDateTime>) -> Self {
                self.timestamp = Some(timestamp.into());
                self
            }
            #[doc = "Page size to use. When not specified, the default page size is 100 records."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(live) = &this.live {
                                    req.url_mut().query_pairs_mut().append_pair("live", &live.to_string());
                                }
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                                }
                                if let Some(timestamp) = &this.timestamp {
                                    req.url_mut().query_pairs_mut().append_pair("timestamp", &timestamp.to_string());
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
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
                                let rsp_value: models::MachineCollection = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Machine;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) timestamp: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying a time instance relative to which to evaluate the machine resource. When not specified, the service uses DateTime.UtcNow."]
            pub fn timestamp(mut self, timestamp: impl Into<time::OffsetDateTime>) -> Self {
                self.timestamp = Some(timestamp.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(timestamp) = &this.timestamp {
                            req.url_mut().query_pairs_mut().append_pair("timestamp", &timestamp.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Machine = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_liveness {
        use super::models;
        type Response = models::Liveness;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/liveness" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Liveness = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_connections {
        use super::models;
        type Response = models::ConnectionCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/connections" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::ConnectionCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_processes {
        use super::models;
        type Response = models::ProcessCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) live: Option<bool>,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
            pub(crate) timestamp: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "Specifies whether to return live resources (true) or inventory resources (false). Defaults to **true**. When retrieving live resources, the start time (`startTime`) and end time (`endTime`) of the desired interval should be included. When retrieving inventory resources, an optional timestamp (`timestamp`) parameter can be specified to return the version of each resource closest (not-after) that timestamp."]
            pub fn live(mut self, live: bool) -> Self {
                self.live = Some(live);
                self
            }
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            #[doc = "UTC date and time specifying a time instance relative to which to evaluate all process resource. Only applies when `live=false`. When not specified, the service uses DateTime.UtcNow."]
            pub fn timestamp(mut self, timestamp: impl Into<time::OffsetDateTime>) -> Self {
                self.timestamp = Some(timestamp.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/processes" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(live) = &this.live {
                                    req.url_mut().query_pairs_mut().append_pair("live", &live.to_string());
                                }
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                                }
                                if let Some(timestamp) = &this.timestamp {
                                    req.url_mut().query_pairs_mut().append_pair("timestamp", &timestamp.to_string());
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
                                let rsp_value: models::ProcessCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_ports {
        use super::models;
        type Response = models::PortCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/ports" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::PortCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_machine_group_membership {
        use super::models;
        type Response = models::MachineGroupCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/machineGroups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::MachineGroupCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod processes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the specified process."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `process_name`: Process resource name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            process_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                process_name: process_name.into(),
                timestamp: None,
            }
        }
        #[doc = "Obtains the liveness status of the process during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `process_name`: Process resource name."]
        pub fn get_liveness(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            process_name: impl Into<String>,
        ) -> get_liveness::Builder {
            get_liveness::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                process_name: process_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of ports on which this process is accepting"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `process_name`: Process resource name."]
        pub fn list_accepting_ports(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            process_name: impl Into<String>,
        ) -> list_accepting_ports::Builder {
            list_accepting_ports::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                process_name: process_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of connections terminating or originating at the specified process"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `process_name`: Process resource name."]
        pub fn list_connections(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            process_name: impl Into<String>,
        ) -> list_connections::Builder {
            list_connections::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                process_name: process_name.into(),
                start_time: None,
                end_time: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Process;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) process_name: String,
            pub(crate) timestamp: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying a time instance relative to which to evaluate a resource. When not specified, the service uses DateTime.UtcNow."]
            pub fn timestamp(mut self, timestamp: impl Into<time::OffsetDateTime>) -> Self {
                self.timestamp = Some(timestamp.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/processes/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . process_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(timestamp) = &this.timestamp {
                            req.url_mut().query_pairs_mut().append_pair("timestamp", &timestamp.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Process = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_liveness {
        use super::models;
        type Response = models::Liveness;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) process_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/processes/{}/liveness" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . process_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Liveness = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_accepting_ports {
        use super::models;
        type Response = models::PortCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) process_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/processes/{}/acceptingPorts" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . process_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::PortCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_connections {
        use super::models;
        type Response = models::ConnectionCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) process_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/processes/{}/connections" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . process_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::ConnectionCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod ports {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the specified port. The port must be live during the specified time interval. If the port is not live during the interval, status 404 (Not Found) is returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `port_name`: Port resource name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            port_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                port_name: port_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Obtains the liveness status of the port during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `port_name`: Port resource name."]
        pub fn get_liveness(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            port_name: impl Into<String>,
        ) -> get_liveness::Builder {
            get_liveness::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                port_name: port_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of processes accepting on the specified port"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `port_name`: Port resource name."]
        pub fn list_accepting_processes(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            port_name: impl Into<String>,
        ) -> list_accepting_processes::Builder {
            list_accepting_processes::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                port_name: port_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns a collection of connections established via the specified port."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_name`: Machine resource name."]
        #[doc = "* `port_name`: Port resource name."]
        pub fn list_connections(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_name: impl Into<String>,
            port_name: impl Into<String>,
        ) -> list_connections::Builder {
            list_connections::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_name: machine_name.into(),
                port_name: port_name.into(),
                start_time: None,
                end_time: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Port;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) port_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/ports/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . port_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Port = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_liveness {
        use super::models;
        type Response = models::Liveness;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) port_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/ports/{}/liveness" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . port_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Liveness = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_accepting_processes {
        use super::models;
        type Response = models::ProcessCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) port_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/ports/{}/acceptingProcesses" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . port_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::ProcessCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_connections {
        use super::models;
        type Response = models::ConnectionCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_name: String,
            pub(crate) port_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machines/{}/ports/{}/connections" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_name , & this . port_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::ConnectionCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod client_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves the specified client group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `client_group_name`: Client Group resource name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            client_group_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                client_group_name: client_group_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns the approximate number of members in the client group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `client_group_name`: Client Group resource name."]
        pub fn get_members_count(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            client_group_name: impl Into<String>,
        ) -> get_members_count::Builder {
            get_members_count::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                client_group_name: client_group_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Returns the members of the client group during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `client_group_name`: Client Group resource name."]
        pub fn list_members(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            client_group_name: impl Into<String>,
        ) -> list_members::Builder {
            list_members::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                client_group_name: client_group_name.into(),
                start_time: None,
                end_time: None,
                top: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ClientGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) client_group_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/clientGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . client_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ClientGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_members_count {
        use super::models;
        type Response = models::ClientGroupMembersCount;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) client_group_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/clientGroups/{}/membersCount" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . client_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ClientGroupMembersCount = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_members {
        use super::models;
        type Response = models::ClientGroupMembersCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) client_group_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            #[doc = "Page size to use. When not specified, the default page size is 100 records."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/clientGroups/{}/members" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . client_group_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
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
                                let rsp_value: models::ClientGroupMembersCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod maps {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Generates the specified map."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `request`: Request options."]
        pub fn generate(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            request: impl Into<models::MapRequest>,
        ) -> generate::Builder {
            generate::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                request: request.into(),
            }
        }
    }
    pub mod generate {
        use super::models;
        type Response = models::MapResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) request: models::MapRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/generateMap" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MapResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod summaries {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns summary information about the machines in the workspace."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        pub fn get_machines(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> get_machines::Builder {
            get_machines::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                start_time: None,
                end_time: None,
            }
        }
    }
    pub mod get_machines {
        use super::models;
        type Response = models::MachinesSummary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/summaries/machines" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachinesSummary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod machine_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns all machine groups during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        pub fn list_by_workspace(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> list_by_workspace::Builder {
            list_by_workspace::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Creates a new machine group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_group`: Machine Group resource to create."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_group: impl Into<models::MachineGroup>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_group: machine_group.into(),
            }
        }
        #[doc = "Returns the specified machine group as it existed during the specified time interval."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_group_name`: Machine Group resource name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_group_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_group_name: machine_group_name.into(),
                start_time: None,
                end_time: None,
            }
        }
        #[doc = "Updates a machine group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_group_name`: Machine Group resource name."]
        #[doc = "* `machine_group`: Machine Group resource to update."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_group_name: impl Into<String>,
            machine_group: impl Into<models::MachineGroup>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_group_name: machine_group_name.into(),
                machine_group: machine_group.into(),
            }
        }
        #[doc = "Deletes the specified Machine Group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription identifier."]
        #[doc = "* `resource_group_name`: Resource group name within the specified subscriptionId."]
        #[doc = "* `workspace_name`: OMS workspace containing the resources of interest."]
        #[doc = "* `machine_group_name`: Machine Group resource name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            machine_group_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                machine_group_name: machine_group_name.into(),
            }
        }
    }
    pub mod list_by_workspace {
        use super::models;
        type Response = models::MachineGroupCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machineGroups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                                if let Some(start_time) = &this.start_time {
                                    req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                                }
                                if let Some(end_time) = &this.end_time {
                                    req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
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
                                let rsp_value: models::MachineGroupCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        type Response = models::MachineGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_group: models::MachineGroup,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machineGroups" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.machine_group)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachineGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::MachineGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_group_name: String,
            pub(crate) start_time: Option<time::OffsetDateTime>,
            pub(crate) end_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "UTC date and time specifying the start time of an interval. When not specified the service uses DateTime.UtcNow - 10m"]
            pub fn start_time(mut self, start_time: impl Into<time::OffsetDateTime>) -> Self {
                self.start_time = Some(start_time.into());
                self
            }
            #[doc = "UTC date and time specifying the end time of an interval. When not specified the service uses DateTime.UtcNow"]
            pub fn end_time(mut self, end_time: impl Into<time::OffsetDateTime>) -> Self {
                self.end_time = Some(end_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machineGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        if let Some(start_time) = &this.start_time {
                            req.url_mut().query_pairs_mut().append_pair("startTime", &start_time.to_string());
                        }
                        if let Some(end_time) = &this.end_time {
                            req.url_mut().query_pairs_mut().append_pair("endTime", &end_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachineGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::MachineGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_group_name: String,
            pub(crate) machine_group: models::MachineGroup,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machineGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.machine_group)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachineGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) machine_group_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/features/serviceMap/machineGroups/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . workspace_name , & this . machine_group_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2015-11-01-preview");
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
