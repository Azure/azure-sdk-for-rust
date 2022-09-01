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
    pub fn available_ground_stations_client(&self) -> available_ground_stations::Client {
        available_ground_stations::Client(self.clone())
    }
    pub fn contact_profiles_client(&self) -> contact_profiles::Client {
        contact_profiles::Client(self.clone())
    }
    pub fn contacts_client(&self) -> contacts::Client {
        contacts::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn spacecrafts_client(&self) -> spacecrafts::Client {
        spacecrafts::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available Orbital Rest API operations."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Orbital/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::OperationListResult> {
                self.send().await?.into_body().await
            }
        }
    }
}
pub mod spacecrafts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return list of spacecrafts"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Return list of spacecrafts"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        pub fn list(&self, subscription_id: impl Into<String>, resource_group_name: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Gets the specified spacecraft in a specified resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
            }
        }
        #[doc = "Creates or updates a spacecraft resource"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `parameters`: The parameters to provide for the created spacecraft."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            parameters: impl Into<models::Spacecraft>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Updates the specified spacecraft tags."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `parameters`: Parameters supplied to update spacecraft tags."]
        pub fn update_tags(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            parameters: impl Into<models::TagsObject>,
        ) -> update_tags::RequestBuilder {
            update_tags::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes a specified spacecraft resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
            }
        }
        #[doc = "Return list of available contacts"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `parameters`: The parameters to provide for the contacts."]
        pub fn list_available_contacts(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            parameters: impl Into<models::ContactParameters>,
        ) -> list_available_contacts::RequestBuilder {
            list_available_contacts::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SpacecraftListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SpacecraftListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Orbital/spacecrafts",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SpacecraftListResult> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SpacecraftListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SpacecraftListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SpacecraftListResult> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Spacecraft = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Spacecraft = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) parameters: models::Spacecraft,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update_tags {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Spacecraft = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) parameters: models::TagsObject,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Spacecraft> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_available_contacts {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AvailableContactsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AvailableContactsListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) parameters: models::ContactParameters,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}/listAvailableContacts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::AvailableContactsListResult> {
                self.send().await?.into_body().await
            }
        }
    }
}
pub mod contacts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns list of contacts by spacecraftName"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
            }
        }
        #[doc = "Gets the specified contact in a specified resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `contact_name`: Contact Name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            contact_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                contact_name: contact_name.into(),
            }
        }
        #[doc = "Creates a contact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `contact_name`: Contact Name"]
        #[doc = "* `parameters`: The parameters to provide for the created contact."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            contact_name: impl Into<String>,
            parameters: impl Into<models::Contact>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                contact_name: contact_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes a specified contact"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `spacecraft_name`: Spacecraft ID"]
        #[doc = "* `contact_name`: Contact Name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            spacecraft_name: impl Into<String>,
            contact_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                spacecraft_name: spacecraft_name.into(),
                contact_name: contact_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}/contacts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactListResult> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Contact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Contact = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) contact_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}/contacts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name,
                            &this.contact_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Contact> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Contact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Contact = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) contact_name: String,
            pub(crate) parameters: models::Contact,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}/contacts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name,
                            &this.contact_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::Contact> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) spacecraft_name: String,
            pub(crate) contact_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/spacecrafts/{}/contacts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.spacecraft_name,
                            &this.contact_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod contact_profiles {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the specified contact Profile in a specified resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `contact_profile_name`: Contact Profile Name"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            contact_profile_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                contact_profile_name: contact_profile_name.into(),
            }
        }
        #[doc = "Creates or updates a contact profile"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `contact_profile_name`: Contact Profile Name"]
        #[doc = "* `parameters`: The parameters to provide for the created Contact Profile."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            contact_profile_name: impl Into<String>,
            parameters: impl Into<models::ContactProfile>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                contact_profile_name: contact_profile_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Updates the specified contact profile tags."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `parameters`: Parameters supplied to update contact profile tags."]
        #[doc = "* `contact_profile_name`: Contact Profile Name"]
        pub fn update_tags(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            parameters: impl Into<models::TagsObject>,
            contact_profile_name: impl Into<String>,
        ) -> update_tags::RequestBuilder {
            update_tags::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                parameters: parameters.into(),
                contact_profile_name: contact_profile_name.into(),
            }
        }
        #[doc = "Deletes a specified contact profile resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `contact_profile_name`: Contact Profile Name"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            contact_profile_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                contact_profile_name: contact_profile_name.into(),
            }
        }
        #[doc = "Returns list of contact profiles"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::RequestBuilder {
            list_by_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Returns list of contact profiles"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        pub fn list(&self, subscription_id: impl Into<String>, resource_group_name: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactProfile = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) contact_profile_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/contactProfiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.contact_profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactProfile = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) contact_profile_name: String,
            pub(crate) parameters: models::ContactProfile,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/contactProfiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.contact_profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod update_tags {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactProfile = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) parameters: models::TagsObject,
            pub(crate) contact_profile_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/contactProfiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.contact_profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfile> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod delete {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) contact_profile_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/contactProfiles/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.contact_profile_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfileListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactProfileListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Orbital/contactProfiles",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfileListResult> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod list {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfileListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ContactProfileListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Orbital/contactProfiles",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::ContactProfileListResult> {
                self.send().await?.into_body().await
            }
        }
    }
}
pub mod available_ground_stations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns list of available ground stations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `capability`: Ground Station Capability"]
        pub fn list_by_capability(
            &self,
            subscription_id: impl Into<String>,
            capability: impl Into<String>,
        ) -> list_by_capability::RequestBuilder {
            list_by_capability::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                capability: capability.into(),
            }
        }
        #[doc = "Gets the specified  available ground station"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `ground_station_name`: Ground Station name"]
        pub fn get(&self, subscription_id: impl Into<String>, ground_station_name: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                ground_station_name: ground_station_name.into(),
            }
        }
    }
    pub mod list_by_capability {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AvailableGroundStationListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AvailableGroundStationListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) capability: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::AvailableGroundStationListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Orbital/availableGroundStations",
                            this.client.endpoint(),
                            &this.subscription_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                                let capability = &this.capability;
                                req.url_mut().query_pairs_mut().append_pair("capability", capability);
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AvailableGroundStation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AvailableGroundStation = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) ground_station_name: String,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Orbital/availableGroundStations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.ground_station_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-04-04-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::AvailableGroundStation> {
                self.send().await?.into_body().await
            }
        }
    }
}
