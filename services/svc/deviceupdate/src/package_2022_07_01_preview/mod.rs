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
    pub fn device_management_client(&self) -> device_management::Client {
        device_management::Client(self.clone())
    }
    pub fn device_update_client(&self) -> device_update::Client {
        device_update::Client(self.clone())
    }
}
pub mod device_update {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all updates that have been imported to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_updates(&self, instance_id: impl Into<String>) -> list_updates::Builder {
            list_updates::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                search: None,
                filter: None,
            }
        }
        #[doc = "Import new update version. This is a long-running-operation; use Operation-Location response header value to check for operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `update_to_import`: The update to be imported."]
        pub fn import_update(
            &self,
            instance_id: impl Into<String>,
            update_to_import: impl Into<models::ImportUpdateInput>,
        ) -> import_update::Builder {
            import_update::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                update_to_import: update_to_import.into(),
            }
        }
        #[doc = "Get a specific update version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn get_update(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> get_update::Builder {
            get_update::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
                if_none_match: None,
            }
        }
        #[doc = "Delete a specific update version. This is a long-running-operation; use Operation-Location response header value to check for operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn delete_update(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> delete_update::Builder {
            delete_update::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
            }
        }
        #[doc = "Get a list of all update providers that have been imported to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_providers(&self, instance_id: impl Into<String>) -> list_providers::Builder {
            list_providers::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Get a list of all update names that match the specified provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        pub fn list_names(&self, instance_id: impl Into<String>, provider: impl Into<String>) -> list_names::Builder {
            list_names::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
            }
        }
        #[doc = "Get a list of all update versions that match the specified provider and name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        pub fn list_versions(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
        ) -> list_versions::Builder {
            list_versions::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                filter: None,
            }
        }
        #[doc = "Get a list of all update file identifiers for the specified version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn list_files(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> list_files::Builder {
            list_files::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
            }
        }
        #[doc = "Get a specific update file from the version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        #[doc = "* `file_id`: File identifier."]
        pub fn get_file(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
            file_id: impl Into<String>,
        ) -> get_file::Builder {
            get_file::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
                file_id: file_id.into(),
                if_none_match: None,
            }
        }
        #[doc = "Get a list of all import update operations. Completed operations are kept for 7 days before auto-deleted. Delete operations are not returned by this API version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_operations(&self, instance_id: impl Into<String>) -> list_operations::Builder {
            list_operations::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
                top: None,
            }
        }
        #[doc = "Retrieve operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Operation identifier."]
        pub fn get_operation(&self, instance_id: impl Into<String>, operation_id: impl Into<String>) -> get_operation::Builder {
            get_operation::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
                if_none_match: None,
            }
        }
    }
    pub mod list_updates {
        use super::models;
        type Response = models::UpdateList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) search: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Request updates matching a free-text search expression."]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "Filter updates by its properties."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/deviceUpdate/{}/updates", this.client.endpoint(), &this.instance_id))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(search) = &this.search {
                                    req.url_mut().query_pairs_mut().append_pair("search", search);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::UpdateList = serde_json::from_slice(&rsp_body)?;
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
    pub mod import_update {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) update_to_import: models::ImportUpdateInput,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates:import",
                            this.client.endpoint(),
                            &this.instance_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_to_import)?;
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
    pub mod get_update {
        use super::models;
        type Response = models::Update;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names/{}/versions/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Update = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_update {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names/{}/versions/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod list_providers {
        use super::models;
        type Response = models::StringsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StringsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_names {
        use super::models;
        type Response = models::StringsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StringsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_versions {
        use super::models;
        type Response = models::StringsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Filter updates by its properties."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names/{}/versions",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::StringsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_files {
        use super::models;
        type Response = models::StringsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names/{}/versions/{}/files",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StringsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_file {
        use super::models;
        type Response = models::UpdateFile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
            pub(crate) file_id: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/providers/{}/names/{}/versions/{}/files/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version,
                            &this.file_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateFile = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_operations {
        use super::models;
        type Response = models::UpdateOperationsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Restricts the set of operations returned. Only one specific filter is supported: \"status eq 'NotStarted' or status eq 'Running'\""]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Specifies a non-negative integer n that limits the number of items returned from a collection. The service returns the number of available items up to but not greater than the specified value n."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/operations",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
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
                                let rsp_value: models::UpdateOperationsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_operation {
        use super::models;
        type Response = models::UpdateOperation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/updates/operations/{}",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateOperation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
pub mod device_management {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of all device classes (unique combinations of device manufacturer and model) for all devices connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_device_classes(&self, instance_id: impl Into<String>) -> list_device_classes::Builder {
            list_device_classes::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets the properties of a device class."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class(&self, instance_id: impl Into<String>, device_class_id: impl Into<String>) -> get_device_class::Builder {
            get_device_class::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Update device class details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `device_class_patch`: The device class json merge patch body. Currently only supports patching friendlyName"]
        pub fn update_device_class(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
            device_class_patch: impl Into<models::PatchBody>,
        ) -> update_device_class::Builder {
            update_device_class::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
                device_class_patch: device_class_patch.into(),
            }
        }
        #[doc = "Deletes a device class. Device classes are created automatically when Device Update-enabled devices are connected to the hub but are not automatically cleaned up since they are referenced by DeviceClassSubgroups. If the user has deleted all DeviceClassSubgroups for a device class they can also delete the device class to remove the records from the system and to stop checking the compatibility of this device class with new updates. If a device is ever reconnected for this device class it will be re-created."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn delete_device_class(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> delete_device_class::Builder {
            delete_device_class::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of installable updates for a device class."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn list_installable_updates_for_device_class(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> list_installable_updates_for_device_class::Builder {
            list_installable_updates_for_device_class::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of devices connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_devices(&self, instance_id: impl Into<String>) -> list_devices::Builder {
            list_devices::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
            }
        }
        #[doc = "Import existing devices from IoT Hub. This is a long-running-operation; use Operation-Location response header value to check for operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `import_type`: The types of devices to import."]
        pub fn import_devices(
            &self,
            instance_id: impl Into<String>,
            import_type: impl Into<models::ImportType>,
        ) -> import_devices::Builder {
            import_devices::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                import_type: import_type.into(),
            }
        }
        #[doc = "Gets the device properties and latest deployment status for a device connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_id`: Device identifier in Azure IoT Hub."]
        pub fn get_device(&self, instance_id: impl Into<String>, device_id: impl Into<String>) -> get_device::Builder {
            get_device::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_id: device_id.into(),
            }
        }
        #[doc = "Gets the device module properties and latest deployment status for a device module connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_id`: Device identifier in Azure IoT Hub."]
        #[doc = "* `module_id`: Device module identifier in Azure IoT Hub."]
        pub fn get_device_module(
            &self,
            instance_id: impl Into<String>,
            device_id: impl Into<String>,
            module_id: impl Into<String>,
        ) -> get_device_module::Builder {
            get_device_module::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_id: device_id.into(),
                module_id: module_id.into(),
            }
        }
        #[doc = "Gets the breakdown of how many devices are on their latest update, have new updates available, or are in progress receiving new updates."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_update_compliance(&self, instance_id: impl Into<String>) -> get_update_compliance::Builder {
            get_update_compliance::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets a list of all device groups.  The $default group will always be returned first."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_groups(&self, instance_id: impl Into<String>) -> list_groups::Builder {
            list_groups::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                orderby: None,
            }
        }
        #[doc = "Gets the device group properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn get_group(&self, instance_id: impl Into<String>, group_id: impl Into<String>) -> get_group::Builder {
            get_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Deletes a device group. This group is automatically created when a Device Update-enabled device is connected to the hub and reports its properties. Groups, subgroups, and deployments are not automatically cleaned up but are retained for history purposes. Users can call this method to delete a group if they do not need to retain any of the history of the group and no longer need it. If a device is ever connected again for this group after the group was deleted it will be automatically re-created but there will be no history."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn delete_group(&self, instance_id: impl Into<String>, group_id: impl Into<String>) -> delete_group::Builder {
            delete_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Get device group update compliance information such as how many devices are on their latest update, how many need new updates, and how many are in progress on receiving a new update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn get_update_compliance_for_group(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> get_update_compliance_for_group::Builder {
            get_update_compliance_for_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Get the best available updates for a device group and a count of how many devices need each update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn list_best_updates_for_group(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> list_best_updates_for_group::Builder {
            list_best_updates_for_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                filter: None,
            }
        }
        #[doc = "Gets a list of deployments for a device group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn list_deployments_for_group(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> list_deployments_for_group::Builder {
            list_deployments_for_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                orderby: None,
            }
        }
        #[doc = "Gets the deployment properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_deployment::Builder {
            get_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Creates or updates a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        #[doc = "* `deployment`: The deployment properties."]
        pub fn create_or_update_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            deployment_id: impl Into<String>,
            deployment: impl Into<models::Deployment>,
        ) -> create_or_update_deployment::Builder {
            create_or_update_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                deployment_id: deployment_id.into(),
                deployment: deployment.into(),
            }
        }
        #[doc = "Deletes a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn delete_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> delete_deployment::Builder {
            delete_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets the status of a deployment including a breakdown of how many devices in the deployment are in progress, completed, or failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment_status(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_deployment_status::Builder {
            get_deployment_status::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Get the device class subgroups for the group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        pub fn list_device_class_subgroups_for_group(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> list_device_class_subgroups_for_group::Builder {
            list_device_class_subgroups_for_group::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                filter: None,
            }
        }
        #[doc = "Gets device class subgroup details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_device_class_subgroup::Builder {
            get_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Deletes a device class subgroup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn delete_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> delete_device_class_subgroup::Builder {
            delete_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Get device class subgroup update compliance information such as how many devices are on their latest update, how many need new updates, and how many are in progress on receiving a new update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class_subgroup_update_compliance(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_device_class_subgroup_update_compliance::Builder {
            get_device_class_subgroup_update_compliance::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Get the best available update for a device class subgroup and a count of how many devices need this update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_best_updates_for_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_best_updates_for_device_class_subgroup::Builder {
            get_best_updates_for_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of deployments for a device class subgroup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn list_deployments_for_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> list_deployments_for_device_class_subgroup::Builder {
            list_deployments_for_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                orderby: None,
            }
        }
        #[doc = "Gets the deployment properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment_for_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_deployment_for_device_class_subgroup::Builder {
            get_deployment_for_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Deletes a device class subgroup deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn delete_deployment_for_device_class_subgroup(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> delete_deployment_for_device_class_subgroup::Builder {
            delete_deployment_for_device_class_subgroup::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Stops a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn stop_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> stop_deployment::Builder {
            stop_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Retries a deployment with failed devices."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn retry_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> retry_deployment::Builder {
            retry_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets the status of a deployment including a breakdown of how many devices in the deployment are in progress, completed, or failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_device_class_subgroup_deployment_status(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_device_class_subgroup_deployment_status::Builder {
            get_device_class_subgroup_deployment_status::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets a list of devices in a deployment along with their state. Useful for getting a list of failed devices."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identity."]
        #[doc = "* `device_class_id`: Device class identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn list_device_states_for_device_class_subgroup_deployment(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            device_class_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> list_device_states_for_device_class_subgroup_deployment::Builder {
            list_device_states_for_device_class_subgroup_deployment::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                device_class_id: device_class_id.into(),
                deployment_id: deployment_id.into(),
                filter: None,
            }
        }
        #[doc = "Retrieve operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Operation identifier."]
        pub fn get_operation(&self, instance_id: impl Into<String>, operation_id: impl Into<String>) -> get_operation::Builder {
            get_operation::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
                if_none_match: None,
            }
        }
        #[doc = "Get a list of all device import operations. Completed operations are kept for 7 days before auto-deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_operations(&self, instance_id: impl Into<String>) -> list_operations::Builder {
            list_operations::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
                top: None,
            }
        }
        #[doc = "Get the device diagnostics log collection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Log collection identifier."]
        pub fn get_log_collection(&self, instance_id: impl Into<String>, operation_id: impl Into<String>) -> get_log_collection::Builder {
            get_log_collection::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
            }
        }
        #[doc = "Start the device diagnostics log collection on specified devices."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Log collection identifier."]
        #[doc = "* `log_collection`: The log collection properties."]
        pub fn start_log_collection(
            &self,
            instance_id: impl Into<String>,
            operation_id: impl Into<String>,
            log_collection: impl Into<models::LogCollection>,
        ) -> start_log_collection::Builder {
            start_log_collection::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
                log_collection: log_collection.into(),
            }
        }
        #[doc = "Get all device diagnostics log collections"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn list_log_collections(&self, instance_id: impl Into<String>) -> list_log_collections::Builder {
            list_log_collections::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Get log collection with detailed status"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Operation identifier."]
        pub fn get_log_collection_detailed_status(
            &self,
            instance_id: impl Into<String>,
            operation_id: impl Into<String>,
        ) -> get_log_collection_detailed_status::Builder {
            get_log_collection_detailed_status::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
            }
        }
        #[doc = "Get list of device health"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `filter`: Filter list by specified properties."]
        pub fn list_device_health(&self, instance_id: impl Into<String>, filter: impl Into<String>) -> list_device_health::Builder {
            list_device_health::Builder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: filter.into(),
            }
        }
    }
    pub mod list_device_classes {
        use super::models;
        type Response = models::DeviceClassesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceClasses",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClassesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_device_class {
        use super::models;
        type Response = models::DeviceClass;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceClasses/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClass = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_device_class {
        use super::models;
        type Response = models::DeviceClass;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
            pub(crate) device_class_patch: models::PatchBody,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceClasses/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        req.insert_header("content-type", "application/merge-patch+json");
                        let req_body = azure_core::to_json(&this.device_class_patch)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClass = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_device_class {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceClasses/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod list_installable_updates_for_device_class {
        use super::models;
        type Response = models::UpdateInfoList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceClasses/{}/installableUpdates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateInfoList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_devices {
        use super::models;
        type Response = models::DevicesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Restricts the set of devices returned. You can filter on GroupId, DeviceClassId, or GroupId and DeploymentStatus."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/devices",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::DevicesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod import_devices {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) import_type: models::ImportType,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/devices:import",
                            this.client.endpoint(),
                            &this.instance_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.import_type)?;
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
    pub mod get_device {
        use super::models;
        type Response = models::Device;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/devices/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod get_device_module {
        use super::models;
        type Response = models::Device;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_id: String,
            pub(crate) module_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/devices/{}/modules/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_id,
                            &this.module_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod get_update_compliance {
        use super::models;
        type Response = models::UpdateCompliance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/updateCompliance",
                            this.client.endpoint(),
                            &this.instance_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateCompliance = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_groups {
        use super::models;
        type Response = models::GroupsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "Orders the set of groups returned. You can order by any combination of groupId, device count, created date, subgroupsWithNewUpdatesAvailableCount, subgroupsWithUpdatesInProgressCount, or subgroupsOnLatestUpdateCount."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
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
                                let rsp_value: models::GroupsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_group {
        use super::models;
        type Response = models::Group;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Group = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod get_update_compliance_for_group {
        use super::models;
        type Response = models::UpdateCompliance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/updateCompliance",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateCompliance = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_best_updates_for_group {
        use super::models;
        type Response = models::DeviceClassSubgroupUpdatableDevicesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Restricts the set of bestUpdates returned. You can filter on update Provider, Name and Version property. This filter is deprecated and should not be used."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/bestUpdates",
                            this.client.endpoint(),
                            &this.instance_id,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::DeviceClassSubgroupUpdatableDevicesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_deployments_for_group {
        use super::models;
        type Response = models::DeploymentsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "Orders the set of deployments returned. You can order by start date."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deployments",
                            this.client.endpoint(),
                            &this.instance_id,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
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
                                let rsp_value: models::DeploymentsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_deployment {
        use super::models;
        type Response = models::Deployment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Deployment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_or_update_deployment {
        use super::models;
        type Response = models::Deployment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) deployment_id: String,
            pub(crate) deployment: models::Deployment,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.deployment)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Deployment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_deployment {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod get_deployment_status {
        use super::models;
        type Response = models::DeploymentStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deployments/{}/status",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeploymentStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_device_class_subgroups_for_group {
        use super::models;
        type Response = models::DeviceClassSubgroupsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Restricts the set of device class subgroups returned. You can filter on compat properties by name and value."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups",
                            this.client.endpoint(),
                            &this.instance_id,
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::DeviceClassSubgroupsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_device_class_subgroup {
        use super::models;
        type Response = models::DeviceClassSubgroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClassSubgroup = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_device_class_subgroup {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod get_device_class_subgroup_update_compliance {
        use super::models;
        type Response = models::UpdateCompliance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/updateCompliance",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpdateCompliance = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_best_updates_for_device_class_subgroup {
        use super::models;
        type Response = models::DeviceClassSubgroupUpdatableDevices;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/bestUpdates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClassSubgroupUpdatableDevices = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_deployments_for_device_class_subgroup {
        use super::models;
        type Response = models::DeploymentsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "Orders the set of deployments returned. You can order by start date."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
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
                                let rsp_value: models::DeploymentsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_deployment_for_device_class_subgroup {
        use super::models;
        type Response = models::Deployment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Deployment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_deployment_for_device_class_subgroup {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
    pub mod stop_deployment {
        use super::models;
        type Response = models::Deployment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}:cancel",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Deployment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod retry_deployment {
        use super::models;
        type Response = models::Deployment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}:retry",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Deployment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_device_class_subgroup_deployment_status {
        use super::models;
        type Response = models::DeviceClassSubgroupDeploymentStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}/status",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceClassSubgroupDeploymentStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_device_states_for_device_class_subgroup_deployment {
        use super::models;
        type Response = models::DeploymentDeviceStatesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) device_class_id: String,
            pub(crate) deployment_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "Restricts the set of deployment device states returned. You can filter on deviceId and moduleId and/or deviceState."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/groups/{}/deviceClassSubgroups/{}/deployments/{}/devicestates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id,
                            &this.device_class_id,
                            &this.deployment_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
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
                                let rsp_value: models::DeploymentDeviceStatesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_operation {
        use super::models;
        type Response = models::DeviceOperation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/operations/{}",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceOperation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_operations {
        use super::models;
        type Response = models::DeviceOperationsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Restricts the set of operations returned. Only one specific filter is supported: \"status eq 'NotStarted' or status eq 'Running'\""]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Specifies a non-negative integer n that limits the number of items returned from a collection. The service returns the number of available items up to but not greater than the specified value n."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/operations",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
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
                                let rsp_value: models::DeviceOperationsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_collection {
        use super::models;
        type Response = models::LogCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceDiagnostics/logCollections/{}",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogCollection = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod start_log_collection {
        use super::models;
        type Response = models::LogCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
            pub(crate) log_collection: models::LogCollection,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceDiagnostics/logCollections/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.operation_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.log_collection)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogCollection = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_log_collections {
        use super::models;
        type Response = models::LogCollectionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceDiagnostics/logCollections",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogCollectionList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_collection_detailed_status {
        use super::models;
        type Response = models::LogCollectionOperationDetailedStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceDiagnostics/logCollections/{}/detailedStatus",
                            this.client.endpoint(),
                            &this.instance_id,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LogCollectionOperationDetailedStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_device_health {
        use super::models;
        type Response = models::DeviceHealthList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceUpdate/{}/management/deviceDiagnostics/deviceHealth",
                            this.client.endpoint(),
                            &this.instance_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2022-07-01-preview");
                                let filter = &this.filter;
                                req.url_mut().query_pairs_mut().append_pair("filter", filter);
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceHealthList = serde_json::from_slice(&rsp_body)?;
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
