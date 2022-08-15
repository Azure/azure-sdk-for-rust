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
    pub fn directory_client(&self) -> directory::Client {
        directory::Client(self.clone())
    }
    pub fn file_client(&self) -> file::Client {
        file::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
    pub fn share_client(&self) -> share::Client {
        share::Client(self.clone())
    }
}
pub mod service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of a storage account's File service, including properties for Storage Analytics metrics and CORS (Cross-Origin Resource Sharing) rules."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_properties(&self, x_ms_version: impl Into<String>) -> get_properties::Builder {
            get_properties::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
            }
        }
        #[doc = "Sets properties for a storage account's File service endpoint, including properties for Storage Analytics metrics and CORS (Cross-Origin Resource Sharing) rules."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_service_properties`: The StorageService properties."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_properties(
            &self,
            storage_service_properties: impl Into<models::StorageServiceProperties>,
            x_ms_version: impl Into<String>,
        ) -> set_properties::Builder {
            set_properties::Builder {
                client: self.0.clone(),
                storage_service_properties: storage_service_properties.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
            }
        }
        #[doc = "The List Shares Segment operation returns a list of the shares and share snapshots under the specified account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn list_shares_segment(&self, x_ms_version: impl Into<String>) -> list_shares_segment::Builder {
            list_shares_segment::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        type Response = models::StorageServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StorageServiceProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod set_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) storage_service_properties: models::StorageServiceProperties,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.storage_service_properties)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod list_shares_segment {
        use super::models;
        type Response = models::ListSharesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "Filters the results to return only entries whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/?comp=list", this.client.endpoint(),))?;
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
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                req.insert_header("x-ms-version", &this.x_ms_version);
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ListSharesResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod share {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns all user-defined metadata and system properties for the specified share or share snapshot. The data returned does not include the share's list of files."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_properties(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> get_properties::Builder {
            get_properties::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                sharesnapshot: None,
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Creates a new share under the specified account. If the share with the same name already exists, the operation fails."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn create(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_share_quota: None,
                x_ms_access_tier: None,
                x_ms_enabled_protocols: None,
                x_ms_root_squash: None,
            }
        }
        #[doc = "Operation marks the specified share or share snapshot for deletion. The share or share snapshot and any files contained within it are later deleted during garbage collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn delete(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                sharesnapshot: None,
                timeout: None,
                x_ms_delete_snapshots: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn acquire_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> acquire_lease::Builder {
            acquire_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn release_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> release_lease::Builder {
            release_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn change_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> change_lease::Builder {
            change_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_proposed_lease_id: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn renew_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> renew_lease::Builder {
            renew_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn break_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> break_lease::Builder {
            break_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_break_period: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                sharesnapshot: None,
            }
        }
        #[doc = "Creates a read-only snapshot of a share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn create_snapshot(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> create_snapshot::Builder {
            create_snapshot::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_meta: None,
            }
        }
        #[doc = "Returns the permission (security descriptor) for a given key"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_file_permission_key`: Key of the permission to be set for the directory/file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_permission(
            &self,
            share_name: impl Into<String>,
            x_ms_file_permission_key: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_permission::Builder {
            get_permission::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_file_permission_key: x_ms_file_permission_key.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
            }
        }
        #[doc = "Create a permission (a security descriptor)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `share_permission`: A permission (a security descriptor) at the share level."]
        pub fn create_permission(
            &self,
            share_name: impl Into<String>,
            x_ms_version: impl Into<String>,
            share_permission: impl Into<models::SharePermission>,
        ) -> create_permission::Builder {
            create_permission::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                share_permission: share_permission.into(),
                timeout: None,
            }
        }
        #[doc = "Sets properties for the specified share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_properties(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> set_properties::Builder {
            set_properties::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_share_quota: None,
                x_ms_access_tier: None,
                x_ms_lease_id: None,
                x_ms_root_squash: None,
            }
        }
        #[doc = "Sets one or more user-defined name-value pairs for the specified share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_metadata(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> set_metadata::Builder {
            set_metadata::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Returns information about stored access policies specified on the share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_access_policy(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> get_access_policy::Builder {
            get_access_policy::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Sets a stored access policy for use with shared access signatures."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_access_policy(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> set_access_policy::Builder {
            set_access_policy::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                share_acl: None,
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Retrieves statistics related to the share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_statistics(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> get_statistics::Builder {
            get_statistics::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Restores a previously deleted Share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn restore(&self, share_name: impl Into<String>, x_ms_version: impl Into<String>) -> restore::Builder {
            restore::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_deleted_share_name: None,
                x_ms_deleted_share_version: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod create {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_share_quota: Option<i64>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_enabled_protocols: Option<String>,
            pub(crate) x_ms_root_squash: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Specifies the maximum size of the share, in gigabytes."]
            pub fn x_ms_share_quota(mut self, x_ms_share_quota: i64) -> Self {
                self.x_ms_share_quota = Some(x_ms_share_quota);
                self
            }
            #[doc = "Specifies the access tier of the share."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Protocols to enable on the share."]
            pub fn x_ms_enabled_protocols(mut self, x_ms_enabled_protocols: impl Into<String>) -> Self {
                self.x_ms_enabled_protocols = Some(x_ms_enabled_protocols.into());
                self
            }
            #[doc = "Root squash to set on the share.  Only valid for NFS shares."]
            pub fn x_ms_root_squash(mut self, x_ms_root_squash: impl Into<String>) -> Self {
                self.x_ms_root_squash = Some(x_ms_root_squash.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_share_quota) = &this.x_ms_share_quota {
                            req.insert_header("x-ms-share-quota", &x_ms_share_quota.to_string());
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_enabled_protocols) = &this.x_ms_enabled_protocols {
                            req.insert_header("x-ms-enabled-protocols", x_ms_enabled_protocols);
                        }
                        if let Some(x_ms_root_squash) = &this.x_ms_root_squash {
                            req.insert_header("x-ms-root-squash", x_ms_root_squash);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_delete_snapshots: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the option include to delete the base share and all of its snapshots."]
            pub fn x_ms_delete_snapshots(mut self, x_ms_delete_snapshots: impl Into<String>) -> Self {
                self.x_ms_delete_snapshots = Some(x_ms_delete_snapshots.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_delete_snapshots) = &this.x_ms_delete_snapshots {
                            req.insert_header("x-ms-delete-snapshots", x_ms_delete_snapshots);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod acquire_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=lease&acquire",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod release_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=lease&release",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod change_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=lease&change",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod renew_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=lease&renew",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod break_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_break_period: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "For a break operation, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60. This break period is only used if it is shorter than the time remaining on the lease. If longer, the time remaining on the lease is used. A new lease will not be available before the break period has expired, but the lease may be held for longer than the break period. If this header does not appear with a break operation, a fixed-duration lease breaks after the remaining lease period elapses, and an infinite lease breaks immediately."]
            pub fn x_ms_lease_break_period(mut self, x_ms_lease_break_period: i64) -> Self {
                self.x_ms_lease_break_period = Some(x_ms_lease_break_period);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=lease&break",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_break_period) = &this.x_ms_lease_break_period {
                            req.insert_header("x-ms-lease-break-period", &x_ms_lease_break_period.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
    pub mod create_snapshot {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=snapshot",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod get_permission {
        use super::models;
        type Response = models::SharePermission;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_file_permission_key: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=filepermission",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-file-permission-key", &this.x_ms_file_permission_key);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SharePermission = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_permission {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) share_permission: models::SharePermission,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=filepermission",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.share_permission)?;
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
    pub mod set_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_share_quota: Option<i64>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_root_squash: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the maximum size of the share, in gigabytes."]
            pub fn x_ms_share_quota(mut self, x_ms_share_quota: i64) -> Self {
                self.x_ms_share_quota = Some(x_ms_share_quota);
                self
            }
            #[doc = "Specifies the access tier of the share."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Root squash to set on the share.  Only valid for NFS shares."]
            pub fn x_ms_root_squash(mut self, x_ms_root_squash: impl Into<String>) -> Self {
                self.x_ms_root_squash = Some(x_ms_root_squash.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=properties",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_share_quota) = &this.x_ms_share_quota {
                            req.insert_header("x-ms-share-quota", &x_ms_share_quota.to_string());
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_root_squash) = &this.x_ms_root_squash {
                            req.insert_header("x-ms-root-squash", x_ms_root_squash);
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
    pub mod set_metadata {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=metadata",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod get_access_policy {
        use super::models;
        type Response = models::SignedIdentifiers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=acl", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SignedIdentifiers = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod set_access_policy {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) share_acl: Option<models::SignedIdentifiers>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The ACL for the share."]
            pub fn share_acl(mut self, share_acl: impl Into<models::SignedIdentifiers>) -> Self {
                self.share_acl = Some(share_acl.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=acl", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(share_acl) = &this.share_acl {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(share_acl)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod get_statistics {
        use super::models;
        type Response = models::ShareStats;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=stats", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ShareStats = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_deleted_share_name: Option<String>,
            pub(crate) x_ms_deleted_share_version: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the name of the previously-deleted share."]
            pub fn x_ms_deleted_share_name(mut self, x_ms_deleted_share_name: impl Into<String>) -> Self {
                self.x_ms_deleted_share_name = Some(x_ms_deleted_share_name.into());
                self
            }
            #[doc = "Specifies the version of the previously-deleted share."]
            pub fn x_ms_deleted_share_version(mut self, x_ms_deleted_share_version: impl Into<String>) -> Self {
                self.x_ms_deleted_share_version = Some(x_ms_deleted_share_version.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}?restype=share&comp=undelete",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_deleted_share_name) = &this.x_ms_deleted_share_name {
                            req.insert_header("x-ms-deleted-share-name", x_ms_deleted_share_name);
                        }
                        if let Some(x_ms_deleted_share_version) = &this.x_ms_deleted_share_version {
                            req.insert_header("x-ms-deleted-share-version", x_ms_deleted_share_version);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
pub mod directory {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns all system properties for the specified directory, and can also be used to check the existence of a directory. The data returned does not include the files in the directory or any subdirectories."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_properties(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_properties::Builder {
            get_properties::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                sharesnapshot: None,
                timeout: None,
            }
        }
        #[doc = "Creates a new directory under the specified share or parent directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn create(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
            }
        }
        #[doc = "Removes the specified empty directory. Note that the directory must be empty before it can be deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn delete(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
            }
        }
        #[doc = "Sets properties on the directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn set_properties(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> set_properties::Builder {
            set_properties::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
            }
        }
        #[doc = "Updates user defined metadata for the specified directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_metadata(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> set_metadata::Builder {
            set_metadata::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_meta: None,
            }
        }
        #[doc = "Returns a list of files or directories under the specified share or directory. It lists the contents only for a single level of the directory hierarchy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn list_files_and_directories_segment(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> list_files_and_directories_segment::Builder {
            list_files_and_directories_segment::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                prefix: None,
                sharesnapshot: None,
                marker: None,
                maxresults: None,
                timeout: None,
                include: Vec::new(),
                x_ms_file_extended_info: None,
            }
        }
        #[doc = "Lists handles for directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn list_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> list_handles::Builder {
            list_handles::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                marker: None,
                maxresults: None,
                timeout: None,
                sharesnapshot: None,
                x_ms_recursive: None,
            }
        }
        #[doc = "Closes all handles open for given directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_handle_id`: Specifies handle ID opened on the file or directory to be closed. Asterisk (‘*’) is a wildcard that specifies all handles."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn force_close_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_handle_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> force_close_handles::Builder {
            force_close_handles::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_handle_id: x_ms_handle_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                marker: None,
                sharesnapshot: None,
                x_ms_recursive: None,
            }
        }
        #[doc = "Renames a directory"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_file_rename_source`: Required. Specifies the URI-style path of the source file, up to 2 KB in length."]
        pub fn rename(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_file_rename_source: impl Into<String>,
        ) -> rename::Builder {
            rename::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_file_rename_source: x_ms_file_rename_source.into(),
                timeout: None,
                x_ms_file_rename_replace_if_exists: None,
                x_ms_file_rename_ignore_readonly: None,
                x_ms_source_lease_id: None,
                x_ms_destination_lease_id: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_meta: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod create {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod set_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory&comp=properties",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
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
    pub mod set_metadata {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory&comp=metadata",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod list_files_and_directories_segment {
        use super::models;
        type Response = models::ListFilesAndDirectoriesSegmentResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) prefix: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) x_ms_file_extended_info: Option<bool>,
        }
        impl Builder {
            #[doc = "Filters the results to return only entries whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "Include extended information."]
            pub fn x_ms_file_extended_info(mut self, x_ms_file_extended_info: bool) -> Self {
                self.x_ms_file_extended_info = Some(x_ms_file_extended_info);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory&comp=list",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
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
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(sharesnapshot) = &this.sharesnapshot {
                                    req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                req.insert_header("x-ms-version", &this.x_ms_version);
                                if let Some(x_ms_file_extended_info) = &this.x_ms_file_extended_info {
                                    req.insert_header("x-ms-file-extended-info", &x_ms_file_extended_info.to_string());
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
                                let rsp_value: models::ListFilesAndDirectoriesSegmentResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_handles {
        use super::models;
        type Response = models::ListHandlesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_recursive: Option<bool>,
        }
        impl Builder {
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Specifies operation should apply to the directory specified in the URI, its files, its subdirectories and their files."]
            pub fn x_ms_recursive(mut self, x_ms_recursive: bool) -> Self {
                self.x_ms_recursive = Some(x_ms_recursive);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=listhandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_recursive) = &this.x_ms_recursive {
                            req.insert_header("x-ms-recursive", &x_ms_recursive.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ListHandlesResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod force_close_handles {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_handle_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) marker: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_recursive: Option<bool>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Specifies operation should apply to the directory specified in the URI, its files, its subdirectories and their files."]
            pub fn x_ms_recursive(mut self, x_ms_recursive: bool) -> Self {
                self.x_ms_recursive = Some(x_ms_recursive);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?comp=forceclosehandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        req.insert_header("x-ms-handle-id", &this.x_ms_handle_id);
                        if let Some(x_ms_recursive) = &this.x_ms_recursive {
                            req.insert_header("x-ms-recursive", &x_ms_recursive.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod rename {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_file_rename_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_rename_replace_if_exists: Option<bool>,
            pub(crate) x_ms_file_rename_ignore_readonly: Option<bool>,
            pub(crate) x_ms_source_lease_id: Option<String>,
            pub(crate) x_ms_destination_lease_id: Option<String>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. A boolean value for if the destination file already exists, whether this request will overwrite the file or not. If true, the rename will succeed and will overwrite the destination file. If not provided or if false and the destination file does exist, the request will not overwrite the destination file. If provided and the destination file doesn’t exist, the rename will succeed. Note: This value does not override the x-ms-file-copy-ignore-read-only header value."]
            pub fn x_ms_file_rename_replace_if_exists(mut self, x_ms_file_rename_replace_if_exists: bool) -> Self {
                self.x_ms_file_rename_replace_if_exists = Some(x_ms_file_rename_replace_if_exists);
                self
            }
            #[doc = "Optional. A boolean value that specifies whether the ReadOnly attribute on a preexisting destination file should be respected. If true, the rename will succeed, otherwise, a previous file at the destination with the ReadOnly attribute set will cause the rename to fail."]
            pub fn x_ms_file_rename_ignore_readonly(mut self, x_ms_file_rename_ignore_readonly: bool) -> Self {
                self.x_ms_file_rename_ignore_readonly = Some(x_ms_file_rename_ignore_readonly);
                self
            }
            #[doc = "Required if the source file has an active infinite lease."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Required if the destination file has an active infinite lease. The lease ID specified for this header must match the lease ID of the destination file. If the request does not include the lease ID or it is not valid, the operation fails with status code 412 (Precondition Failed). If this header is specified and the destination file does not currently have an active lease, the operation will also fail with status code 412 (Precondition Failed)."]
            pub fn x_ms_destination_lease_id(mut self, x_ms_destination_lease_id: impl Into<String>) -> Self {
                self.x_ms_destination_lease_id = Some(x_ms_destination_lease_id.into());
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory&comp=rename",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        req.insert_header("x-ms-file-rename-source", &this.x_ms_file_rename_source);
                        if let Some(x_ms_file_rename_replace_if_exists) = &this.x_ms_file_rename_replace_if_exists {
                            req.insert_header(
                                "x-ms-file-rename-replace-if-exists",
                                &x_ms_file_rename_replace_if_exists.to_string(),
                            );
                        }
                        if let Some(x_ms_file_rename_ignore_readonly) = &this.x_ms_file_rename_ignore_readonly {
                            req.insert_header("x-ms-file-rename-ignore-readonly", &x_ms_file_rename_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        if let Some(x_ms_destination_lease_id) = &this.x_ms_destination_lease_id {
                            req.insert_header("x-ms-destination-lease-id", x_ms_destination_lease_id);
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
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
}
pub mod file {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Reads or downloads a file from the system, including its metadata and properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn download(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_range: None,
                x_ms_range_get_content_md5: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Creates a new file or replaces a file. Note it only initializes the file with no content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_content_length`: Specifies the maximum size for the file, up to 4 TB."]
        #[doc = "* `x_ms_type`: Dummy constant parameter, file type can only be file."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn create(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_content_length: i64,
            x_ms_type: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_content_length,
                x_ms_type: x_ms_type.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_content_type: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_cache_control: None,
                x_ms_content_md5: None,
                x_ms_content_disposition: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "removes the file from the storage account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn delete(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Returns all user-defined metadata, standard HTTP properties, and system properties for the file. It does not return the content of the file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_properties(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_properties::Builder {
            get_properties::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                sharesnapshot: None,
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Sets HTTP headers on the file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn set_http_headers(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> set_http_headers::Builder {
            set_http_headers::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_content_length: None,
                x_ms_content_type: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_cache_control: None,
                x_ms_content_md5: None,
                x_ms_content_disposition: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Updates user-defined metadata for the specified file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_metadata(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> set_metadata::Builder {
            set_metadata::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn acquire_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> acquire_lease::Builder {
            acquire_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn release_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> release_lease::Builder {
            release_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn change_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> change_lease::Builder {
            change_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_proposed_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn break_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> break_lease::Builder {
            break_lease::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Upload a range of bytes to a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_range`: Specifies the range of bytes to be written. Both the start and end of the range must be specified. For an update operation, the range can be up to 4 MB in size. For a clear operation, the range can be up to the value of the file's full size. The File service accepts only a single byte range for the Range and 'x-ms-range' headers, and the byte range must be specified in the following format: bytes=startByte-endByte."]
        #[doc = "* `x_ms_write`: Specify one of the following options: - Update: Writes the bytes specified by the request body into the specified range. The Range and Content-Length headers must match to perform the update. - Clear: Clears the specified range and releases the space used in storage for that range. To clear a range, set the Content-Length header to zero, and set the Range header to a value that indicates the range to clear, up to maximum file size."]
        #[doc = "* `content_length`: Specifies the number of bytes being transmitted in the request body. When the x-ms-write header is set to clear, the value of this header must be set to zero."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn upload_range(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_range: impl Into<String>,
            x_ms_write: impl Into<String>,
            content_length: i64,
            x_ms_version: impl Into<String>,
        ) -> upload_range::Builder {
            upload_range::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_range: x_ms_range.into(),
                x_ms_write: x_ms_write.into(),
                content_length,
                x_ms_version: x_ms_version.into(),
                optionalbody: None,
                timeout: None,
                content_md5: None,
                x_ms_lease_id: None,
                x_ms_file_last_write_time: None,
            }
        }
        #[doc = "Upload a range of bytes to a file where the contents are read from a URL."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_range`: Writes data to the specified byte range in the file."]
        #[doc = "* `x_ms_copy_source`: Specifies the URL of the source file or blob, up to 2 KB in length. To copy a file to another file within the same storage account, you may use Shared Key to authenticate the source file. If you are copying a file from another storage account, or if you are copying a blob from the same storage account or another storage account, then you must authenticate the source file or blob using a shared access signature. If the source is a public blob, no authentication is required to perform the copy operation. A file in a share snapshot can also be specified as a copy source."]
        #[doc = "* `x_ms_write`: Only update is supported: - Update: Writes the bytes downloaded from the source url into the specified range."]
        #[doc = "* `content_length`: Specifies the number of bytes being transmitted in the request body. When the x-ms-write header is set to clear, the value of this header must be set to zero."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn upload_range_from_url(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_range: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
            x_ms_write: impl Into<String>,
            content_length: i64,
            x_ms_version: impl Into<String>,
        ) -> upload_range_from_url::Builder {
            upload_range_from_url::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_range: x_ms_range.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                x_ms_write: x_ms_write.into(),
                content_length,
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_source_range: None,
                x_ms_source_content_crc64: None,
                x_ms_source_if_match_crc64: None,
                x_ms_source_if_none_match_crc64: None,
                x_ms_lease_id: None,
                x_ms_copy_source_authorization: None,
                x_ms_file_last_write_time: None,
            }
        }
        #[doc = "Returns the list of valid ranges for a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_range_list(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_range_list::Builder {
            get_range_list::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                sharesnapshot: None,
                prevsharesnapshot: None,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Copies a blob or file to a destination file within the storage account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_copy_source`: Specifies the URL of the source file or blob, up to 2 KB in length. To copy a file to another file within the same storage account, you may use Shared Key to authenticate the source file. If you are copying a file from another storage account, or if you are copying a blob from the same storage account or another storage account, then you must authenticate the source file or blob using a shared access signature. If the source is a public blob, no authentication is required to perform the copy operation. A file in a share snapshot can also be specified as a copy source."]
        pub fn start_copy(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
        ) -> start_copy::Builder {
            start_copy::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_permission_copy_mode: None,
                x_ms_file_copy_ignore_readonly: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_copy_set_archive: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Aborts a pending Copy File operation, and leaves a destination file with zero length and full metadata."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_copy_action`: Abort."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn abort_copy(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_copy_action: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> abort_copy::Builder {
            abort_copy::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_copy_action: x_ms_copy_action.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Lists handles for file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn list_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> list_handles::Builder {
            list_handles::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                marker: None,
                maxresults: None,
                timeout: None,
                sharesnapshot: None,
            }
        }
        #[doc = "Closes all handles open for given file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_handle_id`: Specifies handle ID opened on the file or directory to be closed. Asterisk (‘*’) is a wildcard that specifies all handles."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn force_close_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_handle_id: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> force_close_handles::Builder {
            force_close_handles::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_handle_id: x_ms_handle_id.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                marker: None,
                sharesnapshot: None,
            }
        }
        #[doc = "Renames a file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `x_ms_file_rename_source`: Required. Specifies the URI-style path of the source file, up to 2 KB in length."]
        pub fn rename(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_version: impl Into<String>,
            x_ms_file_rename_source: impl Into<String>,
        ) -> rename::Builder {
            rename::Builder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_version: x_ms_version.into(),
                x_ms_file_rename_source: x_ms_file_rename_source.into(),
                timeout: None,
                x_ms_file_rename_replace_if_exists: None,
                x_ms_file_rename_ignore_readonly: None,
                x_ms_source_lease_id: None,
                x_ms_destination_lease_id: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_meta: None,
                x_ms_content_type: None,
            }
        }
    }
    pub mod download {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(serde_json::Value),
            PartialContent206(serde_json::Value),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_range_get_content_md5: Option<bool>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return file data only from the specified byte range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "When this header is set to true and specified together with the Range header, the service returns the MD5 hash for the range, as long as the range is less than or equal to 4 MB in size."]
            pub fn x_ms_range_get_content_md5(mut self, x_ms_range_get_content_md5: bool) -> Self {
                self.x_ms_range_get_content_md5 = Some(x_ms_range_get_content_md5);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_range_get_content_md5) = &this.x_ms_range_get_content_md5 {
                            req.insert_header("x-ms-range-get-content-md5", &x_ms_range_get_content_md5.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::PartialContent => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::PartialContent206(rsp_value))
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_content_length: i64,
            pub(crate) x_ms_type: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Specifies which content encodings have been applied to the file."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Specifies the natural languages used by this resource."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Sets the file's cache control. The File service stores this value but does not use or modify it."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Sets the file's MD5 hash."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "Sets the file's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        req.insert_header("x-ms-content-length", &this.x_ms_content_length.to_string());
                        req.insert_header("x-ms-type", &this.x_ms_type);
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod get_properties {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod set_http_headers {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_content_length: Option<i64>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Resizes a file to the specified size. If the specified byte value is less than the current size of the file, then all ranges above the specified byte value are cleared."]
            pub fn x_ms_content_length(mut self, x_ms_content_length: i64) -> Self {
                self.x_ms_content_length = Some(x_ms_content_length);
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Specifies which content encodings have been applied to the file."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Specifies the natural languages used by this resource."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Sets the file's cache control. The File service stores this value but does not use or modify it."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Sets the file's MD5 hash."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "Sets the file's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=properties",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_content_length) = &this.x_ms_content_length {
                            req.insert_header("x-ms-content-length", &x_ms_content_length.to_string());
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod set_metadata {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=metadata",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod acquire_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=lease&acquire",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod release_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=lease&release",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod change_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=lease&change",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod break_lease {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=lease&break",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod upload_range {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_range: String,
            pub(crate) x_ms_write: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_version: String,
            pub(crate) optionalbody: Option<serde_json::Value>,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
        }
        impl Builder {
            #[doc = "Initial data."]
            pub fn optionalbody(mut self, optionalbody: impl Into<serde_json::Value>) -> Self {
                self.optionalbody = Some(optionalbody.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "An MD5 hash of the content. This hash is used to verify the integrity of the data during transport. When the Content-MD5 header is specified, the File service compares the hash of the content that has arrived with the header value that was sent. If the two hashes do not match, the operation will fail with error code 400 (Bad Request)."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If the file last write time should be preserved or overwritten"]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=range",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(optionalbody) = &this.optionalbody {
                            req.insert_header("content-type", "application/octet-stream");
                            azure_core::to_json(optionalbody)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-range", &this.x_ms_range);
                        req.insert_header("x-ms-write", &this.x_ms_write);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
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
    pub mod upload_range_from_url {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_range: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) x_ms_write: String,
            pub(crate) content_length: i64,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_source_range: Option<String>,
            pub(crate) x_ms_source_content_crc64: Option<String>,
            pub(crate) x_ms_source_if_match_crc64: Option<String>,
            pub(crate) x_ms_source_if_none_match_crc64: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Bytes of source data in the specified range."]
            pub fn x_ms_source_range(mut self, x_ms_source_range: impl Into<String>) -> Self {
                self.x_ms_source_range = Some(x_ms_source_range.into());
                self
            }
            #[doc = "Specify the crc64 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_crc64(mut self, x_ms_source_content_crc64: impl Into<String>) -> Self {
                self.x_ms_source_content_crc64 = Some(x_ms_source_content_crc64.into());
                self
            }
            #[doc = "Specify the crc64 value to operate only on range with a matching crc64 checksum."]
            pub fn x_ms_source_if_match_crc64(mut self, x_ms_source_if_match_crc64: impl Into<String>) -> Self {
                self.x_ms_source_if_match_crc64 = Some(x_ms_source_if_match_crc64.into());
                self
            }
            #[doc = "Specify the crc64 value to operate only on range without a matching crc64 checksum."]
            pub fn x_ms_source_if_none_match_crc64(mut self, x_ms_source_if_none_match_crc64: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match_crc64 = Some(x_ms_source_if_none_match_crc64.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "If the file last write time should be preserved or overwritten"]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=range&fromURL",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-range", &this.x_ms_range);
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_source_range) = &this.x_ms_source_range {
                            req.insert_header("x-ms-source-range", x_ms_source_range);
                        }
                        req.insert_header("x-ms-write", &this.x_ms_write);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_source_content_crc64) = &this.x_ms_source_content_crc64 {
                            req.insert_header("x-ms-source-content-crc64", x_ms_source_content_crc64);
                        }
                        if let Some(x_ms_source_if_match_crc64) = &this.x_ms_source_if_match_crc64 {
                            req.insert_header("x-ms-source-if-match-crc64", x_ms_source_if_match_crc64);
                        }
                        if let Some(x_ms_source_if_none_match_crc64) = &this.x_ms_source_if_none_match_crc64 {
                            req.insert_header("x-ms-source-if-none-match-crc64", x_ms_source_if_none_match_crc64);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod get_range_list {
        use super::models;
        type Response = models::ShareFileRangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) prevsharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The previous snapshot parameter is an opaque DateTime value that, when present, specifies the previous snapshot."]
            pub fn prevsharesnapshot(mut self, prevsharesnapshot: impl Into<String>) -> Self {
                self.prevsharesnapshot = Some(prevsharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the range of bytes over which to list ranges, inclusively."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=rangelist",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(prevsharesnapshot) = &this.prevsharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("prevsharesnapshot", prevsharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ShareFileRangeList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod start_copy {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_permission_copy_mode: Option<String>,
            pub(crate) x_ms_file_copy_ignore_readonly: Option<bool>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_copy_set_archive: Option<bool>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Specifies the option to copy file security descriptor from source file or to set it using the value which is defined by the header value of x-ms-file-permission or x-ms-file-permission-key."]
            pub fn x_ms_file_permission_copy_mode(mut self, x_ms_file_permission_copy_mode: impl Into<String>) -> Self {
                self.x_ms_file_permission_copy_mode = Some(x_ms_file_permission_copy_mode.into());
                self
            }
            #[doc = "Specifies the option to overwrite the target file if it already exists and has read-only attribute set."]
            pub fn x_ms_file_copy_ignore_readonly(mut self, x_ms_file_copy_ignore_readonly: bool) -> Self {
                self.x_ms_file_copy_ignore_readonly = Some(x_ms_file_copy_ignore_readonly);
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "Specifies the option to set archive attribute on a target file. True means archive attribute will be set on a target file despite attribute overrides or a source file state."]
            pub fn x_ms_file_copy_set_archive(mut self, x_ms_file_copy_set_archive: bool) -> Self {
                self.x_ms_file_copy_set_archive = Some(x_ms_file_copy_set_archive);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=copy",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_file_permission_copy_mode) = &this.x_ms_file_permission_copy_mode {
                            req.insert_header("x-ms-file-permission-copy-mode", x_ms_file_permission_copy_mode);
                        }
                        if let Some(x_ms_file_copy_ignore_readonly) = &this.x_ms_file_copy_ignore_readonly {
                            req.insert_header("x-ms-file-copy-ignore-readonly", &x_ms_file_copy_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_copy_set_archive) = &this.x_ms_file_copy_set_archive {
                            req.insert_header("x-ms-file-copy-set-archive", &x_ms_file_copy_set_archive.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod abort_copy {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_copy_action: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=copy&copyid",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-copy-action", &this.x_ms_copy_action);
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
    pub mod list_handles {
        use super::models;
        type Response = models::ListHandlesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
        }
        impl Builder {
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=listhandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ListHandlesResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod force_close_handles {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_handle_id: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) marker: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=forceclosehandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        req.insert_header("x-ms-handle-id", &this.x_ms_handle_id);
                        req.insert_header("x-ms-version", &this.x_ms_version);
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
    pub mod rename {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_version: String,
            pub(crate) x_ms_file_rename_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_rename_replace_if_exists: Option<bool>,
            pub(crate) x_ms_file_rename_ignore_readonly: Option<bool>,
            pub(crate) x_ms_source_lease_id: Option<String>,
            pub(crate) x_ms_destination_lease_id: Option<String>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_content_type: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. A boolean value for if the destination file already exists, whether this request will overwrite the file or not. If true, the rename will succeed and will overwrite the destination file. If not provided or if false and the destination file does exist, the request will not overwrite the destination file. If provided and the destination file doesn’t exist, the rename will succeed. Note: This value does not override the x-ms-file-copy-ignore-read-only header value."]
            pub fn x_ms_file_rename_replace_if_exists(mut self, x_ms_file_rename_replace_if_exists: bool) -> Self {
                self.x_ms_file_rename_replace_if_exists = Some(x_ms_file_rename_replace_if_exists);
                self
            }
            #[doc = "Optional. A boolean value that specifies whether the ReadOnly attribute on a preexisting destination file should be respected. If true, the rename will succeed, otherwise, a previous file at the destination with the ReadOnly attribute set will cause the rename to fail."]
            pub fn x_ms_file_rename_ignore_readonly(mut self, x_ms_file_rename_ignore_readonly: bool) -> Self {
                self.x_ms_file_rename_ignore_readonly = Some(x_ms_file_rename_ignore_readonly);
                self
            }
            #[doc = "Required if the source file has an active infinite lease."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Required if the destination file has an active infinite lease. The lease ID specified for this header must match the lease ID of the destination file. If the request does not include the lease ID or it is not valid, the operation fails with status code 412 (Precondition Failed). If this header is specified and the destination file does not currently have an active lease, the operation will also fail with status code 412 (Precondition Failed)."]
            pub fn x_ms_destination_lease_id(mut self, x_ms_destination_lease_id: impl Into<String>) -> Self {
                self.x_ms_destination_lease_id = Some(x_ms_destination_lease_id.into());
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}?comp=rename",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        req.insert_header("x-ms-file-rename-source", &this.x_ms_file_rename_source);
                        if let Some(x_ms_file_rename_replace_if_exists) = &this.x_ms_file_rename_replace_if_exists {
                            req.insert_header(
                                "x-ms-file-rename-replace-if-exists",
                                &x_ms_file_rename_replace_if_exists.to_string(),
                            );
                        }
                        if let Some(x_ms_file_rename_ignore_readonly) = &this.x_ms_file_rename_ignore_readonly {
                            req.insert_header("x-ms-file-rename-ignore-readonly", &x_ms_file_rename_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        if let Some(x_ms_destination_lease_id) = &this.x_ms_destination_lease_id {
                            req.insert_header("x-ms-destination-lease-id", x_ms_destination_lease_id);
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
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
}
