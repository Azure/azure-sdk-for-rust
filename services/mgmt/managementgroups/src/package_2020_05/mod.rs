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
    pub fn entities_client(&self) -> entities::Client {
        entities::Client(self.clone())
    }
    pub fn hierarchy_settings_client(&self) -> hierarchy_settings::Client {
        hierarchy_settings::Client(self.clone())
    }
    pub fn management_group_subscriptions_client(&self) -> management_group_subscriptions::Client {
        management_group_subscriptions::Client(self.clone())
    }
    pub fn management_groups_client(&self) -> management_groups::Client {
        management_groups::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
pub mod management_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List management groups for the authenticated user.\n"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                cache_control: None,
                skiptoken: None,
            }
        }
        #[doc = "Get the details of the management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn get(&self, group_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                expand: None,
                recurse: None,
                filter: None,
                cache_control: None,
            }
        }
        #[doc = "Create or update a management group.\nIf a management group is already created and a subsequent create request is issued with different properties, the management group properties will be updated.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `create_management_group_request`: Management group creation parameters."]
        pub fn create_or_update(
            &self,
            group_id: impl Into<String>,
            create_management_group_request: impl Into<models::CreateManagementGroupRequest>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                create_management_group_request: create_management_group_request.into(),
                cache_control: None,
            }
        }
        #[doc = "Update a management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `patch_group_request`: Management group patch parameters."]
        pub fn update(
            &self,
            group_id: impl Into<String>,
            patch_group_request: impl Into<models::PatchManagementGroupRequest>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                patch_group_request: patch_group_request.into(),
                cache_control: None,
            }
        }
        #[doc = "Delete management group.\nIf a management group contains child resources, the request will fail.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn delete(&self, group_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                cache_control: None,
            }
        }
        #[doc = "List all entities that descend from a management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn get_descendants(&self, group_id: impl Into<String>) -> get_descendants::Builder {
            get_descendants::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                skiptoken: None,
                top: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ManagementGroupListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) cache_control: Option<String>,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            #[doc = "Page continuation token is only used if a previous operation returned a partial result. \nIf a previous response contains a nextLink element, the value of the nextLink element will include a token parameter that specifies a starting point to use for subsequent calls.\n"]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                                if let Some(cache_control) = &this.cache_control {
                                    req.insert_header("cache-control", cache_control);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::ManagementGroupListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ManagementGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) expand: Option<String>,
            pub(crate) recurse: Option<bool>,
            pub(crate) filter: Option<String>,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "The $expand=children query string parameter allows clients to request inclusion of children in the response payload.  $expand=path includes the path from the root group to the current group."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The $recurse=true query string parameter allows clients to request inclusion of entire hierarchy in the response payload. Note that  $expand=children must be passed up if $recurse is set to true."]
            pub fn recurse(mut self, recurse: bool) -> Self {
                self.recurse = Some(recurse);
                self
            }
            #[doc = "A filter which allows the exclusion of subscriptions from results (i.e. '$filter=children.childType ne Subscription')"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        if let Some(recurse) = &this.recurse {
                            req.url_mut().query_pairs_mut().append_pair("$recurse", &recurse.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagementGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            Ok200(models::ManagementGroup),
            Accepted202(models::AzureAsyncOperationResults),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) create_management_group_request: models::CreateManagementGroupRequest,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_management_group_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagementGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureAsyncOperationResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = models::ManagementGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) patch_group_request: models::PatchManagementGroupRequest,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.patch_group_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManagementGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            Accepted202(models::AzureAsyncOperationResults),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AzureAsyncOperationResults = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
                            }
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
    pub mod get_descendants {
        use super::models;
        type Response = models::DescendantListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) skiptoken: Option<String>,
            pub(crate) top: Option<i64>,
        }
        impl Builder {
            #[doc = "Page continuation token is only used if a previous operation returned a partial result. \nIf a previous response contains a nextLink element, the value of the nextLink element will include a token parameter that specifies a starting point to use for subsequent calls.\n"]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            #[doc = "Number of elements to return when retrieving results. Passing this in will override $skipToken."]
            pub fn top(mut self, top: i64) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/descendants",
                            this.client.endpoint(),
                            &this.group_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::DescendantListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod management_group_subscriptions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves details about given subscription which is associated with the management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `subscription_id`: Subscription ID."]
        pub fn get_subscription(&self, group_id: impl Into<String>, subscription_id: impl Into<String>) -> get_subscription::Builder {
            get_subscription::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                subscription_id: subscription_id.into(),
                cache_control: None,
            }
        }
        #[doc = "Associates existing subscription with the management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `subscription_id`: Subscription ID."]
        pub fn create(&self, group_id: impl Into<String>, subscription_id: impl Into<String>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                subscription_id: subscription_id.into(),
                cache_control: None,
            }
        }
        #[doc = "De-associates subscription from the management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `subscription_id`: Subscription ID."]
        pub fn delete(&self, group_id: impl Into<String>, subscription_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                subscription_id: subscription_id.into(),
                cache_control: None,
            }
        }
        #[doc = "Retrieves details about all subscriptions which are associated with the management group.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn get_subscriptions_under_management_group(
            &self,
            group_id: impl Into<String>,
        ) -> get_subscriptions_under_management_group::Builder {
            get_subscriptions_under_management_group::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                skiptoken: None,
            }
        }
    }
    pub mod get_subscription {
        use super::models;
        type Response = models::SubscriptionUnderManagementGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) subscription_id: String,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/subscriptions/{}",
                            this.client.endpoint(),
                            &this.group_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionUnderManagementGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::SubscriptionUnderManagementGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) subscription_id: String,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/subscriptions/{}",
                            this.client.endpoint(),
                            &this.group_id,
                            &this.subscription_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionUnderManagementGroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) group_id: String,
            pub(crate) subscription_id: String,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/subscriptions/{}",
                            this.client.endpoint(),
                            &this.group_id,
                            &this.subscription_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        if let Some(cache_control) = &this.cache_control {
                            req.insert_header("cache-control", cache_control);
                        }
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
    pub mod get_subscriptions_under_management_group {
        use super::models;
        type Response = models::ListSubscriptionUnderManagementGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) skiptoken: Option<String>,
        }
        impl Builder {
            #[doc = "Page continuation token is only used if a previous operation returned a partial result. \nIf a previous response contains a nextLink element, the value of the nextLink element will include a token parameter that specifies a starting point to use for subsequent calls.\n"]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/subscriptions",
                            this.client.endpoint(),
                            &this.group_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
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
                                let rsp_value: models::ListSubscriptionUnderManagementGroup = serde_json::from_slice(&rsp_body)?;
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
pub mod hierarchy_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the hierarchy settings defined at the Management Group level. Settings can only be set on the root Management Group of the hierarchy.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn list(&self, group_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Gets the hierarchy settings defined at the Management Group level. Settings can only be set on the root Management Group of the hierarchy.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn get(&self, group_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Creates or updates the hierarchy settings defined at the Management Group level.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `create_tenant_settings_request`: Tenant level settings request parameter."]
        pub fn create_or_update(
            &self,
            group_id: impl Into<String>,
            create_tenant_settings_request: impl Into<models::CreateOrUpdateSettingsRequest>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                create_tenant_settings_request: create_tenant_settings_request.into(),
            }
        }
        #[doc = "Updates the hierarchy settings defined at the Management Group level.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        #[doc = "* `create_tenant_settings_request`: Tenant level settings request parameter."]
        pub fn update(
            &self,
            group_id: impl Into<String>,
            create_tenant_settings_request: impl Into<models::CreateOrUpdateSettingsRequest>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
                create_tenant_settings_request: create_tenant_settings_request.into(),
            }
        }
        #[doc = "Deletes the hierarchy settings defined at the Management Group level.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_id`: Management Group ID."]
        pub fn delete(&self, group_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                group_id: group_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::HierarchySettingsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/settings",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HierarchySettingsList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::HierarchySettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/settings/default",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HierarchySettings = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::HierarchySettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) create_tenant_settings_request: models::CreateOrUpdateSettingsRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/settings/default",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_tenant_settings_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HierarchySettings = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = models::HierarchySettings;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) group_id: String,
            pub(crate) create_tenant_settings_request: models::CreateOrUpdateSettingsRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/settings/default",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_tenant_settings_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::HierarchySettings = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Management/managementGroups/{}/settings/default",
                            this.client.endpoint(),
                            &this.group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available Management REST API operations."]
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
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Management/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
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
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
impl Client {
    #[doc = "Checks if the specified management group name is valid and unique"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `check_name_availability_request`: Management group name availability check parameters."]
    pub fn check_name_availability(
        &self,
        check_name_availability_request: impl Into<models::CheckNameAvailabilityRequest>,
    ) -> check_name_availability::Builder {
        check_name_availability::Builder {
            client: self.clone(),
            check_name_availability_request: check_name_availability_request.into(),
        }
    }
    #[doc = "Starts backfilling subscriptions for the Tenant."]
    pub fn start_tenant_backfill(&self) -> start_tenant_backfill::Builder {
        start_tenant_backfill::Builder { client: self.clone() }
    }
    #[doc = "Gets tenant backfill status"]
    pub fn tenant_backfill_status(&self) -> tenant_backfill_status::Builder {
        tenant_backfill_status::Builder { client: self.clone() }
    }
}
pub mod check_name_availability {
    use super::models;
    type Response = models::CheckNameAvailabilityResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) check_name_availability_request: models::CheckNameAvailabilityRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Management/checkNameAvailability",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.check_name_availability_request)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CheckNameAvailabilityResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod start_tenant_backfill {
    use super::models;
    type Response = models::TenantBackfillStatusResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Management/startTenantBackfill",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::TenantBackfillStatusResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod tenant_backfill_status {
    use super::models;
    type Response = models::TenantBackfillStatusResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Management/tenantBackfillStatus",
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
                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::TenantBackfillStatusResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod entities {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all entities (Management Groups, Subscriptions, etc.) for the authenticated user.\n"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skiptoken: None,
                skip: None,
                top: None,
                select: None,
                search: None,
                filter: None,
                view: None,
                group_name: None,
                cache_control: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::EntityListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skiptoken: Option<String>,
            pub(crate) skip: Option<i64>,
            pub(crate) top: Option<i64>,
            pub(crate) select: Option<String>,
            pub(crate) search: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) view: Option<String>,
            pub(crate) group_name: Option<String>,
            pub(crate) cache_control: Option<String>,
        }
        impl Builder {
            #[doc = "Page continuation token is only used if a previous operation returned a partial result. \nIf a previous response contains a nextLink element, the value of the nextLink element will include a token parameter that specifies a starting point to use for subsequent calls.\n"]
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            #[doc = "Number of entities to skip over when retrieving results. Passing this in will override $skipToken."]
            pub fn skip(mut self, skip: i64) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "Number of elements to return when retrieving results. Passing this in will override $skipToken."]
            pub fn top(mut self, top: i64) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "This parameter specifies the fields to include in the response. Can include any combination of Name,DisplayName,Type,ParentDisplayNameChain,ParentChain, e.g. '$select=Name,DisplayName,Type,ParentDisplayNameChain,ParentNameChain'. When specified the $select parameter can override select in $skipToken."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The $search parameter is used in conjunction with the $filter parameter to return three different outputs depending on the parameter passed in. \nWith $search=AllowedParents the API will return the entity info of all groups that the requested entity will be able to reparent to as determined by the user's permissions.\nWith $search=AllowedChildren the API will return the entity info of all entities that can be added as children of the requested entity.\nWith $search=ParentAndFirstLevelChildren the API will return the parent and  first level of children that the user has either direct access to or indirect access via one of their descendants.\nWith $search=ParentOnly the API will return only the group if the user has access to at least one of the descendants of the group.\nWith $search=ChildrenOnly the API will return only the first level of children of the group entity info specified in $filter.  The user must have direct access to the children entities or one of it's descendants for it to show up in the results."]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "The filter parameter allows you to filter on the the name or display name fields. You can check for equality on the name field (e.g. name eq '{entityName}')  and you can check for substrings on either the name or display name fields(e.g. contains(name, '{substringToSearch}'), contains(displayName, '{substringToSearch')). Note that the '{entityName}' and '{substringToSearch}' fields are checked case insensitively."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The view parameter allows clients to filter the type of data that is returned by the getEntities call."]
            pub fn view(mut self, view: impl Into<String>) -> Self {
                self.view = Some(view.into());
                self
            }
            #[doc = "A filter which allows the get entities call to focus on a particular group (i.e. \"$filter=name eq 'groupName'\")"]
            pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
                self.group_name = Some(group_name.into());
                self
            }
            #[doc = "Indicates that the request shouldn't utilize any caches."]
            pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
                self.cache_control = Some(cache_control.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Management/getEntities", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-01");
                                if let Some(skiptoken) = &this.skiptoken {
                                    req.url_mut().query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                if let Some(skip) = &this.skip {
                                    req.url_mut().query_pairs_mut().append_pair("$skip", &skip.to_string());
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
                                }
                                if let Some(search) = &this.search {
                                    req.url_mut().query_pairs_mut().append_pair("$search", search);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(view) = &this.view {
                                    req.url_mut().query_pairs_mut().append_pair("$view", view);
                                }
                                if let Some(group_name) = &this.group_name {
                                    req.url_mut().query_pairs_mut().append_pair("groupName", group_name);
                                }
                                if let Some(cache_control) = &this.cache_control {
                                    req.insert_header("cache-control", cache_control);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityListResult = serde_json::from_slice(&rsp_body)?;
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
