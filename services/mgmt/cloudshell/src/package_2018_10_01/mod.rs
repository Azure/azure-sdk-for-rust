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
}
impl Client {
    #[doc = "Get user settings."]
    #[doc = "Get current user settings for current signed in user. This operation returns settings for the user's cloud shell preferences including preferred location, storage profile, shell type, font and size settings."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `location`: The provider location"]
    pub fn get_user_settings_with_location(
        &self,
        user_settings_name: impl Into<String>,
        location: impl Into<String>,
    ) -> get_user_settings_with_location::Builder {
        get_user_settings_with_location::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            location: location.into(),
        }
    }
    #[doc = "put user settings."]
    #[doc = "Create or update cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `location`: The provider location"]
    #[doc = "* `parameters`: The properties of the user settings to be created or updated."]
    pub fn put_user_settings_with_location(
        &self,
        user_settings_name: impl Into<String>,
        location: impl Into<String>,
        parameters: impl Into<models::CloudShellUserSettings>,
    ) -> put_user_settings_with_location::Builder {
        put_user_settings_with_location::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            location: location.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "patch user settings."]
    #[doc = "Patch cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `location`: The provider location"]
    #[doc = "* `parameters`: The properties of the user settings to be updated."]
    pub fn patch_user_settings_with_location(
        &self,
        user_settings_name: impl Into<String>,
        location: impl Into<String>,
        parameters: impl Into<models::CloudShellPatchUserSettings>,
    ) -> patch_user_settings_with_location::Builder {
        patch_user_settings_with_location::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            location: location.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "delete user settings."]
    #[doc = "Delete cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `location`: The provider location"]
    pub fn delete_user_settings_with_location(
        &self,
        user_settings_name: impl Into<String>,
        location: impl Into<String>,
    ) -> delete_user_settings_with_location::Builder {
        delete_user_settings_with_location::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Get console"]
    #[doc = "Gets the console for the user."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    #[doc = "* `location`: The provider location"]
    pub fn get_console_with_location(
        &self,
        console_name: impl Into<String>,
        location: impl Into<String>,
    ) -> get_console_with_location::Builder {
        get_console_with_location::Builder {
            client: self.clone(),
            console_name: console_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Put console"]
    #[doc = "Puts a request for a console"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    #[doc = "* `location`: The provider location"]
    pub fn put_console_with_location(
        &self,
        console_name: impl Into<String>,
        location: impl Into<String>,
    ) -> put_console_with_location::Builder {
        put_console_with_location::Builder {
            client: self.clone(),
            console_name: console_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Delete console"]
    #[doc = "Deletes the console"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    #[doc = "* `location`: The provider location"]
    pub fn delete_console_with_location(
        &self,
        console_name: impl Into<String>,
        location: impl Into<String>,
    ) -> delete_console_with_location::Builder {
        delete_console_with_location::Builder {
            client: self.clone(),
            console_name: console_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Keep alive"]
    #[doc = "Keep console alive"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    #[doc = "* `location`: The provider location"]
    pub fn keep_alive_with_location(
        &self,
        console_name: impl Into<String>,
        location: impl Into<String>,
    ) -> keep_alive_with_location::Builder {
        keep_alive_with_location::Builder {
            client: self.clone(),
            console_name: console_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Get user settings."]
    #[doc = "Get current user settings for current signed in user. This operation returns settings for the user's cloud shell preferences including preferred location, storage profile, shell type, font and size settings."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    pub fn get_user_settings(&self, user_settings_name: impl Into<String>) -> get_user_settings::Builder {
        get_user_settings::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
        }
    }
    #[doc = "put user settings."]
    #[doc = "Create or update cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `parameters`: The properties of the user settings to be created or updated."]
    pub fn put_user_settings(
        &self,
        user_settings_name: impl Into<String>,
        parameters: impl Into<models::CloudShellUserSettings>,
    ) -> put_user_settings::Builder {
        put_user_settings::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "patch user settings."]
    #[doc = "Patch cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    #[doc = "* `parameters`: The properties of the user settings to be updated."]
    pub fn patch_user_settings(
        &self,
        user_settings_name: impl Into<String>,
        parameters: impl Into<models::CloudShellPatchUserSettings>,
    ) -> patch_user_settings::Builder {
        patch_user_settings::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "delete user settings."]
    #[doc = "Delete cloud shell settings for current signed in user"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `user_settings_name`: The name of the user settings"]
    pub fn delete_user_settings(&self, user_settings_name: impl Into<String>) -> delete_user_settings::Builder {
        delete_user_settings::Builder {
            client: self.clone(),
            user_settings_name: user_settings_name.into(),
        }
    }
    #[doc = "Get console"]
    #[doc = "Gets the console for the user."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    pub fn get_console(&self, console_name: impl Into<String>) -> get_console::Builder {
        get_console::Builder {
            client: self.clone(),
            console_name: console_name.into(),
        }
    }
    #[doc = "Put console"]
    #[doc = "Puts a request for a console"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    #[doc = "* `parameters`: The console definition."]
    pub fn put_console(&self, console_name: impl Into<String>, parameters: impl Into<models::ConsoleDefinition>) -> put_console::Builder {
        put_console::Builder {
            client: self.clone(),
            console_name: console_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Delete console"]
    #[doc = "Deletes the console"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    pub fn delete_console(&self, console_name: impl Into<String>) -> delete_console::Builder {
        delete_console::Builder {
            client: self.clone(),
            console_name: console_name.into(),
        }
    }
    #[doc = "Keep alive"]
    #[doc = "Keep console alive"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `console_name`: The name of the console"]
    pub fn keep_alive(&self, console_name: impl Into<String>) -> keep_alive::Builder {
        keep_alive::Builder {
            client: self.clone(),
            console_name: console_name.into(),
        }
    }
}
pub mod get_user_settings_with_location {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod put_user_settings_with_location {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) location: String,
        pub(crate) parameters: models::CloudShellUserSettings,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod patch_user_settings_with_location {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) location: String,
        pub(crate) parameters: models::CloudShellPatchUserSettings,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod delete_user_settings_with_location {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
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
pub mod get_console_with_location {
    use super::models;
    type Response = models::CloudShellConsole;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
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
pub mod put_console_with_location {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200(models::CloudShellConsole),
        Created201(models::CloudShellConsole),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
                            Ok(Response::Ok200(rsp_value))
                        }
                        azure_core::StatusCode::Created => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
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
pub mod delete_console_with_location {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
                        this.client.endpoint(),
                        &this.location,
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
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
pub mod keep_alive_with_location {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
        pub(crate) location: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}/keepAlive",
                        this.client.endpoint(),
                        &this.location,
                        &this.console_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
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
pub mod get_user_settings {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/userSettings/{}",
                        this.client.endpoint(),
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod put_user_settings {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) parameters: models::CloudShellUserSettings,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/userSettings/{}",
                        this.client.endpoint(),
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod patch_user_settings {
    use super::models;
    type Response = models::UserSettingsResponse;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
        pub(crate) parameters: models::CloudShellPatchUserSettings,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/userSettings/{}",
                        this.client.endpoint(),
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::UserSettingsResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod delete_user_settings {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) user_settings_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/userSettings/{}",
                        this.client.endpoint(),
                        &this.user_settings_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
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
pub mod get_console {
    use super::models;
    type Response = models::CloudShellConsole;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/consoles/{}",
                        this.client.endpoint(),
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
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
pub mod put_console {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200(models::CloudShellConsole),
        Created201(models::CloudShellConsole),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
        pub(crate) parameters: models::ConsoleDefinition,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/consoles/{}",
                        this.client.endpoint(),
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
                            Ok(Response::Ok200(rsp_value))
                        }
                        azure_core::StatusCode::Created => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CloudShellConsole = serde_json::from_slice(&rsp_body)?;
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
pub mod delete_console {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/consoles/{}",
                        this.client.endpoint(),
                        &this.console_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-10-01");
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
pub mod keep_alive {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) console_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Portal/consoles/{}/keepAlive",
                        this.client.endpoint(),
                        &this.console_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
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
