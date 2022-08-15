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
    pub fn android_client(&self) -> android::Client {
        android::Client(self.clone())
    }
    pub fn ios_client(&self) -> ios::Client {
        ios::Client(self.clone())
    }
}
impl Client {
    #[doc = "Returns location for user tenant."]
    pub fn get_locations(&self) -> get_locations::Builder {
        get_locations::Builder { client: self.clone() }
    }
    #[doc = "Returns location for given tenant."]
    pub fn get_location_by_host_name(&self) -> get_location_by_host_name::Builder {
        get_location_by_host_name::Builder { client: self.clone() }
    }
    #[doc = "Returns Intune Manageable apps."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    pub fn get_apps(&self, host_name: impl Into<String>) -> get_apps::Builder {
        get_apps::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            filter: None,
            top: None,
            select: None,
        }
    }
    #[doc = "Get devices for a user."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    #[doc = "* `user_name`: user unique Name"]
    pub fn get_mam_user_devices(&self, host_name: impl Into<String>, user_name: impl Into<String>) -> get_mam_user_devices::Builder {
        get_mam_user_devices::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            user_name: user_name.into(),
            filter: None,
            top: None,
            select: None,
        }
    }
    #[doc = "Get a unique device for a user."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    #[doc = "* `user_name`: unique user name"]
    #[doc = "* `device_name`: device name"]
    pub fn get_mam_user_device_by_device_name(
        &self,
        host_name: impl Into<String>,
        user_name: impl Into<String>,
        device_name: impl Into<String>,
    ) -> get_mam_user_device_by_device_name::Builder {
        get_mam_user_device_by_device_name::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            user_name: user_name.into(),
            device_name: device_name.into(),
            select: None,
        }
    }
    #[doc = "Wipe a device for a user."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    #[doc = "* `user_name`: unique user name"]
    #[doc = "* `device_name`: device name"]
    pub fn wipe_mam_user_device(
        &self,
        host_name: impl Into<String>,
        user_name: impl Into<String>,
        device_name: impl Into<String>,
    ) -> wipe_mam_user_device::Builder {
        wipe_mam_user_device::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            user_name: user_name.into(),
            device_name: device_name.into(),
        }
    }
    #[doc = "Returns operationResults."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    pub fn get_operation_results(&self, host_name: impl Into<String>) -> get_operation_results::Builder {
        get_operation_results::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            filter: None,
            top: None,
            select: None,
        }
    }
    #[doc = "Returns Intune Tenant level statuses."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    pub fn get_mam_statuses(&self, host_name: impl Into<String>) -> get_mam_statuses::Builder {
        get_mam_statuses::Builder {
            client: self.clone(),
            host_name: host_name.into(),
        }
    }
    #[doc = "Returns Intune flagged user collection"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    pub fn get_mam_flagged_users(&self, host_name: impl Into<String>) -> get_mam_flagged_users::Builder {
        get_mam_flagged_users::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            filter: None,
            top: None,
            select: None,
        }
    }
    #[doc = "Returns Intune flagged user details"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    #[doc = "* `user_name`: Flagged userName"]
    pub fn get_mam_flagged_user_by_name(
        &self,
        host_name: impl Into<String>,
        user_name: impl Into<String>,
    ) -> get_mam_flagged_user_by_name::Builder {
        get_mam_flagged_user_by_name::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            user_name: user_name.into(),
            select: None,
        }
    }
    #[doc = "Returns Intune flagged enrolled app collection for the User"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `host_name`: Location hostName for the tenant"]
    #[doc = "* `user_name`: User name for the tenant"]
    pub fn get_mam_user_flagged_enrolled_apps(
        &self,
        host_name: impl Into<String>,
        user_name: impl Into<String>,
    ) -> get_mam_user_flagged_enrolled_apps::Builder {
        get_mam_user_flagged_enrolled_apps::Builder {
            client: self.clone(),
            host_name: host_name.into(),
            user_name: user_name.into(),
            filter: None,
            top: None,
            select: None,
        }
    }
}
pub mod get_locations {
    use super::models;
    type Response = models::LocationCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Intune/locations", this.client.endpoint(),))?;
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::LocationCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod get_location_by_host_name {
    use super::models;
    type Response = models::Location;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url =
                        azure_core::Url::parse(&format!("{}/providers/Microsoft.Intune/locations/hostName", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Location = serde_json::from_slice(&rsp_body)?;
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
pub mod get_apps {
    use super::models;
    type Response = models::ApplicationCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) top: Option<i32>,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "The filter to apply on the operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        pub fn top(mut self, top: i32) -> Self {
            self.top = Some(top);
            self
        }
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/apps",
                        this.client.endpoint(),
                        &this.host_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(top) = &this.top {
                                req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                            }
                            if let Some(select) = &this.select {
                                req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                            let rsp_value: models::ApplicationCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_user_devices {
    use super::models;
    type Response = models::DeviceCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) user_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) top: Option<i32>,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "The filter to apply on the operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        pub fn top(mut self, top: i32) -> Self {
            self.top = Some(top);
            self
        }
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/users/{}/devices",
                        this.client.endpoint(),
                        &this.host_name,
                        &this.user_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(top) = &this.top {
                                req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                            }
                            if let Some(select) = &this.select {
                                req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                            let rsp_value: models::DeviceCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_user_device_by_device_name {
    use super::models;
    type Response = models::Device;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) user_name: String,
        pub(crate) device_name: String,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/users/{}/devices/{}",
                        this.client.endpoint(),
                        &this.host_name,
                        &this.user_name,
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
                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                    if let Some(select) = &this.select {
                        req.url_mut().query_pairs_mut().append_pair("$select", select);
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
pub mod wipe_mam_user_device {
    use super::models;
    type Response = models::WipeDeviceOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) user_name: String,
        pub(crate) device_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/users/{}/devices/{}/wipe",
                        this.client.endpoint(),
                        &this.host_name,
                        &this.user_name,
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
                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::WipeDeviceOperationResult = serde_json::from_slice(&rsp_body)?;
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
pub mod get_operation_results {
    use super::models;
    type Response = models::OperationResultCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) top: Option<i32>,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "The filter to apply on the operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        pub fn top(mut self, top: i32) -> Self {
            self.top = Some(top);
            self
        }
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/operationResults",
                        this.client.endpoint(),
                        &this.host_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(top) = &this.top {
                                req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                            }
                            if let Some(select) = &this.select {
                                req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                            let rsp_value: models::OperationResultCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_statuses {
    use super::models;
    type Response = models::StatusesDefault;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
    }
    impl Builder {
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/statuses/default",
                        this.client.endpoint(),
                        &this.host_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StatusesDefault = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_flagged_users {
    use super::models;
    type Response = models::FlaggedUserCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) top: Option<i32>,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "The filter to apply on the operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        pub fn top(mut self, top: i32) -> Self {
            self.top = Some(top);
            self
        }
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/flaggedUsers",
                        this.client.endpoint(),
                        &this.host_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(top) = &this.top {
                                req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                            }
                            if let Some(select) = &this.select {
                                req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                            let rsp_value: models::FlaggedUserCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_flagged_user_by_name {
    use super::models;
    type Response = models::FlaggedUser;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) user_name: String,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/flaggedUsers/{}",
                        this.client.endpoint(),
                        &this.host_name,
                        &this.user_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                    if let Some(select) = &this.select {
                        req.url_mut().query_pairs_mut().append_pair("$select", select);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::FlaggedUser = serde_json::from_slice(&rsp_body)?;
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
pub mod get_mam_user_flagged_enrolled_apps {
    use super::models;
    type Response = models::FlaggedEnrolledAppCollection;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) host_name: String,
        pub(crate) user_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) top: Option<i32>,
        pub(crate) select: Option<String>,
    }
    impl Builder {
        #[doc = "The filter to apply on the operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        pub fn top(mut self, top: i32) -> Self {
            self.top = Some(top);
            self
        }
        #[doc = "select specific fields in entity."]
        pub fn select(mut self, select: impl Into<String>) -> Self {
            self.select = Some(select.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Intune/locations/{}/flaggedUsers/{}/flaggedEnrolledApps",
                        this.client.endpoint(),
                        &this.host_name,
                        &this.user_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(top) = &this.top {
                                req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                            }
                            if let Some(select) = &this.select {
                                req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                            let rsp_value: models::FlaggedEnrolledAppCollection = serde_json::from_slice(&rsp_body)?;
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
pub mod ios {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns Intune iOSPolicies."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        pub fn get_mam_policies(&self, host_name: impl Into<String>) -> get_mam_policies::Builder {
            get_mam_policies::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                filter: None,
                top: None,
                select: None,
            }
        }
        #[doc = "Returns Intune iOS policies."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn get_mam_policy_by_name(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_mam_policy_by_name::Builder {
            get_mam_policy_by_name::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                select: None,
            }
        }
        #[doc = "Creates or updates iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update an android policy operation."]
        pub fn create_or_update_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            parameters: impl Into<models::IOsmamPolicy>,
        ) -> create_or_update_mam_policy::Builder {
            create_or_update_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = " patch an iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update an android policy operation."]
        pub fn patch_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            parameters: impl Into<models::IOsmamPolicy>,
        ) -> patch_mam_policy::Builder {
            patch_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete Ios Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn delete_mam_policy(&self, host_name: impl Into<String>, policy_name: impl Into<String>) -> delete_mam_policy::Builder {
            delete_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Get apps for an iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn get_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_app_for_mam_policy::Builder {
            get_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                filter: None,
                top: None,
                select: None,
            }
        }
        #[doc = "Add app to an iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `app_name`: application unique Name"]
        #[doc = "* `parameters`: Parameters supplied to add an app to an ios policy."]
        pub fn add_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            app_name: impl Into<String>,
            parameters: impl Into<models::MamPolicyAppIdOrGroupIdPayload>,
        ) -> add_app_for_mam_policy::Builder {
            add_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                app_name: app_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete App for Ios Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `app_name`: application unique Name"]
        pub fn delete_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            app_name: impl Into<String>,
        ) -> delete_app_for_mam_policy::Builder {
            delete_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                app_name: app_name.into(),
            }
        }
        #[doc = "Returns groups for a given iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: policy name for the tenant"]
        pub fn get_groups_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_groups_for_mam_policy::Builder {
            get_groups_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Add group to an iOSMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `group_id`: group Id"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update app to an android policy operation."]
        pub fn add_group_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            group_id: impl Into<String>,
            parameters: impl Into<models::MamPolicyAppIdOrGroupIdPayload>,
        ) -> add_group_for_mam_policy::Builder {
            add_group_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                group_id: group_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete Group for iOS Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `group_id`: application unique Name"]
        pub fn delete_group_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            group_id: impl Into<String>,
        ) -> delete_group_for_mam_policy::Builder {
            delete_group_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                group_id: group_id.into(),
            }
        }
    }
    pub mod get_mam_policies {
        use super::models;
        type Response = models::IosmamPolicyCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies",
                            this.client.endpoint(),
                            &this.host_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                                let rsp_value: models::IosmamPolicyCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_mam_policy_by_name {
        use super::models;
        type Response = models::IOsmamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IOsmamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_mam_policy {
        use super::models;
        type Response = models::IOsmamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) parameters: models::IOsmamPolicy,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IOsmamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod patch_mam_policy {
        use super::models;
        type Response = models::IOsmamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) parameters: models::IOsmamPolicy,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IOsmamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
    pub mod get_app_for_mam_policy {
        use super::models;
        type Response = models::ApplicationCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/apps",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                                let rsp_value: models::ApplicationCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod add_app_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) app_name: String,
            pub(crate) parameters: models::MamPolicyAppIdOrGroupIdPayload,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/apps/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
                            &this.app_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete_app_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) app_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/apps/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
                            &this.app_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
    pub mod get_groups_for_mam_policy {
        use super::models;
        type Response = models::GroupsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/groups",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GroupsCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod add_group_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) group_id: String,
            pub(crate) parameters: models::MamPolicyAppIdOrGroupIdPayload,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/groups/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete_group_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/iosPolicies/{}/groups/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
pub mod android {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns Intune Android policies."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        pub fn get_mam_policies(&self, host_name: impl Into<String>) -> get_mam_policies::Builder {
            get_mam_policies::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                filter: None,
                top: None,
                select: None,
            }
        }
        #[doc = "Returns AndroidMAMPolicy with given name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn get_mam_policy_by_name(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_mam_policy_by_name::Builder {
            get_mam_policy_by_name::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                select: None,
            }
        }
        #[doc = "Creates or updates AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update an android policy operation."]
        pub fn create_or_update_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            parameters: impl Into<models::AndroidMamPolicy>,
        ) -> create_or_update_mam_policy::Builder {
            create_or_update_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Patch AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update an android policy operation."]
        pub fn patch_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            parameters: impl Into<models::AndroidMamPolicy>,
        ) -> patch_mam_policy::Builder {
            patch_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete Android Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn delete_mam_policy(&self, host_name: impl Into<String>, policy_name: impl Into<String>) -> delete_mam_policy::Builder {
            delete_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Get apps for an AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        pub fn get_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_app_for_mam_policy::Builder {
            get_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                filter: None,
                top: None,
                select: None,
            }
        }
        #[doc = "Add app to an AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `app_name`: application unique Name"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update app to an android policy operation."]
        pub fn add_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            app_name: impl Into<String>,
            parameters: impl Into<models::MamPolicyAppIdOrGroupIdPayload>,
        ) -> add_app_for_mam_policy::Builder {
            add_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                app_name: app_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete App for Android Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `app_name`: application unique Name"]
        pub fn delete_app_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            app_name: impl Into<String>,
        ) -> delete_app_for_mam_policy::Builder {
            delete_app_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                app_name: app_name.into(),
            }
        }
        #[doc = "Returns groups for a given AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: policy name for the tenant"]
        pub fn get_groups_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
        ) -> get_groups_for_mam_policy::Builder {
            get_groups_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
            }
        }
        #[doc = "Add group to an AndroidMAMPolicy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `group_id`: group Id"]
        #[doc = "* `parameters`: Parameters supplied to the Create or update app to an android policy operation."]
        pub fn add_group_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            group_id: impl Into<String>,
            parameters: impl Into<models::MamPolicyAppIdOrGroupIdPayload>,
        ) -> add_group_for_mam_policy::Builder {
            add_group_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                group_id: group_id.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete Group for Android Policy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `host_name`: Location hostName for the tenant"]
        #[doc = "* `policy_name`: Unique name for the policy"]
        #[doc = "* `group_id`: application unique Name"]
        pub fn delete_group_for_mam_policy(
            &self,
            host_name: impl Into<String>,
            policy_name: impl Into<String>,
            group_id: impl Into<String>,
        ) -> delete_group_for_mam_policy::Builder {
            delete_group_for_mam_policy::Builder {
                client: self.0.clone(),
                host_name: host_name.into(),
                policy_name: policy_name.into(),
                group_id: group_id.into(),
            }
        }
    }
    pub mod get_mam_policies {
        use super::models;
        type Response = models::AndroidMamPolicyCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies",
                            this.client.endpoint(),
                            &this.host_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                                let rsp_value: models::AndroidMamPolicyCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_mam_policy_by_name {
        use super::models;
        type Response = models::AndroidMamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AndroidMamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_mam_policy {
        use super::models;
        type Response = models::AndroidMamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) parameters: models::AndroidMamPolicy,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AndroidMamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod patch_mam_policy {
        use super::models;
        type Response = models::AndroidMamPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) parameters: models::AndroidMamPolicy,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AndroidMamPolicy = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}",
                            this.client.endpoint(),
                            &this.host_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
    pub mod get_app_for_mam_policy {
        use super::models;
        type Response = models::ApplicationCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "select specific fields in entity."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/AndroidPolicies/{}/apps",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("$select", select);
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
                                let rsp_value: models::ApplicationCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod add_app_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) app_name: String,
            pub(crate) parameters: models::MamPolicyAppIdOrGroupIdPayload,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}/apps/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
                            &this.app_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete_app_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) app_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}/apps/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
                            &this.app_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
    pub mod get_groups_for_mam_policy {
        use super::models;
        type Response = models::GroupsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}/groups",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GroupsCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod add_group_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) group_id: String,
            pub(crate) parameters: models::MamPolicyAppIdOrGroupIdPayload,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}/groups/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod delete_group_for_mam_policy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) host_name: String,
            pub(crate) policy_name: String,
            pub(crate) group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Intune/locations/{}/androidPolicies/{}/groups/{}",
                            this.client.endpoint(),
                            &this.host_name,
                            &this.policy_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2015-01-14-preview");
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
