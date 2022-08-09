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
    pub fn accepted_sent_shares_client(&self) -> accepted_sent_shares::Client {
        accepted_sent_shares::Client(self.clone())
    }
    pub fn asset_mappings_client(&self) -> asset_mappings::Client {
        asset_mappings::Client(self.clone())
    }
    pub fn assets_client(&self) -> assets::Client {
        assets::Client(self.clone())
    }
    pub fn email_registration_client(&self) -> email_registration::Client {
        email_registration::Client(self.clone())
    }
    pub fn received_assets_client(&self) -> received_assets::Client {
        received_assets::Client(self.clone())
    }
    pub fn received_invitations_client(&self) -> received_invitations::Client {
        received_invitations::Client(self.clone())
    }
    pub fn received_shares_client(&self) -> received_shares::Client {
        received_shares::Client(self.clone())
    }
    pub fn sent_share_invitations_client(&self) -> sent_share_invitations::Client {
        sent_share_invitations::Client(self.clone())
    }
    pub fn sent_shares_client(&self) -> sent_shares::Client {
        sent_shares::Client(self.clone())
    }
}
pub mod assets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Assets in a share."]
        #[doc = "List assets on a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        pub fn list(&self, sent_share_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Get asset in a sentShare."]
        #[doc = "Get an asset on a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `asset_name`: The name of the asset"]
        pub fn get(&self, sent_share_name: impl Into<String>, asset_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                asset_name: asset_name.into(),
            }
        }
        #[doc = "Adds a new asset to an existing share."]
        #[doc = "Create an asset on a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `asset_name`: The name of the asset"]
        #[doc = "* `asset`: The asset payload to be created."]
        pub fn create(
            &self,
            sent_share_name: impl Into<String>,
            asset_name: impl Into<String>,
            asset: impl Into<models::Asset>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                asset_name: asset_name.into(),
                asset: asset.into(),
            }
        }
        #[doc = "Delete asset in a sentShare."]
        #[doc = "Delete an asset on a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `asset_name`: The name of the asset"]
        pub fn delete(&self, sent_share_name: impl Into<String>, asset_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                asset_name: asset_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AssetList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/sentShares/{}/assets", this.client.endpoint(), &this.sent_share_name))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::AssetList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Asset;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) asset_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/assets/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.asset_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Asset = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Asset;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) asset_name: String,
            pub(crate) asset: models::Asset,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/assets/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.asset_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.asset)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Asset = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) sent_share_name: String,
            pub(crate) asset_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/assets/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.asset_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
pub mod asset_mappings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List AssetMappings in a received share."]
        #[doc = "List asset mappings for a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        pub fn list(&self, received_share_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Get AssetMapping in a receivedShare."]
        #[doc = "Get an asset mapping for a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        #[doc = "* `asset_mapping_name`: The name of the asset mapping"]
        pub fn get(&self, received_share_name: impl Into<String>, asset_mapping_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                asset_mapping_name: asset_mapping_name.into(),
            }
        }
        #[doc = "Maps a source asset in the sent share to a destination asset in the received share."]
        #[doc = "Create an asset mapping on a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        #[doc = "* `asset_mapping_name`: The name of the asset mapping"]
        #[doc = "* `asset_mapping`: The asset mapping payload."]
        pub fn create(
            &self,
            received_share_name: impl Into<String>,
            asset_mapping_name: impl Into<String>,
            asset_mapping: impl Into<models::AssetMapping>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                asset_mapping_name: asset_mapping_name.into(),
                asset_mapping: asset_mapping.into(),
            }
        }
        #[doc = "Delete AssetMapping in a receivedShare."]
        #[doc = "Delete an asset mapping for a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        #[doc = "* `asset_mapping_name`: The name of the asset mapping"]
        pub fn delete(&self, received_share_name: impl Into<String>, asset_mapping_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                asset_mapping_name: asset_mapping_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AssetMappingList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/receivedShares/{}/assetMappings",
                            this.client.endpoint(),
                            &this.received_share_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::AssetMappingList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AssetMapping;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) asset_mapping_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/receivedShares/{}/assetMappings/{}",
                            this.client.endpoint(),
                            &this.received_share_name,
                            &this.asset_mapping_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssetMapping = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AssetMapping),
            Accepted202(models::AssetMapping),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) asset_mapping_name: String,
            pub(crate) asset_mapping: models::AssetMapping,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/receivedShares/{}/assetMappings/{}",
                            this.client.endpoint(),
                            &this.received_share_name,
                            &this.asset_mapping_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.asset_mapping)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssetMapping = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssetMapping = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) asset_mapping_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/receivedShares/{}/assetMappings/{}",
                            this.client.endpoint(),
                            &this.received_share_name,
                            &this.asset_mapping_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
pub mod received_invitations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the received invitations."]
        #[doc = "List received invitations"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Gets the received invitation identified by name"]
        #[doc = "Get a received invitation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_invitation_name`: Name of the received invitation"]
        pub fn get(&self, received_invitation_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                received_invitation_name: received_invitation_name.into(),
            }
        }
        #[doc = "Rejects the received invitation identified by name"]
        #[doc = "Reject a received invitation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_invitation_name`: Name of the received invitation"]
        #[doc = "* `received_invitation`: The received invitation to reject"]
        pub fn reject(
            &self,
            received_invitation_name: impl Into<String>,
            received_invitation: impl Into<models::ReceivedInvitation>,
        ) -> reject::Builder {
            reject::Builder {
                client: self.0.clone(),
                received_invitation_name: received_invitation_name.into(),
                received_invitation: received_invitation.into(),
                repeatability_request_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ReceivedInvitationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/receivedInvitations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::ReceivedInvitationList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReceivedInvitation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_invitation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/receivedInvitations/{}",
                            this.client.endpoint(),
                            &this.received_invitation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReceivedInvitation = serde_json::from_slice(&rsp_body)?;
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
    pub mod reject {
        use super::models;
        type Response = models::ReceivedInvitation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_invitation_name: String,
            pub(crate) received_invitation: models::ReceivedInvitation,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/receivedInvitations/{}:reject",
                            this.client.endpoint(),
                            &this.received_invitation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.received_invitation)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReceivedInvitation = serde_json::from_slice(&rsp_body)?;
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
pub mod received_shares {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of received shares."]
        #[doc = "List received shares"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Get a received share by name."]
        #[doc = "Get a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        pub fn get(&self, received_share_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
            }
        }
        #[doc = "Create a received share in the given account."]
        #[doc = "Create a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        #[doc = "* `received_share`: The received share to create."]
        pub fn create(&self, received_share_name: impl Into<String>, received_share: impl Into<models::ReceivedShare>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                received_share: received_share.into(),
            }
        }
        #[doc = "Deletes a received share"]
        #[doc = "Delete a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        pub fn delete(&self, received_share_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ReceivedShareList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/receivedShares", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::ReceivedShareList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReceivedShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/receivedShares/{}", this.client.endpoint(), &this.received_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReceivedShare = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ReceivedShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) received_share: models::ReceivedShare,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/receivedShares/{}", this.client.endpoint(), &this.received_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.received_share)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReceivedShare = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) received_share_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/receivedShares/{}", this.client.endpoint(), &this.received_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
pub mod received_assets {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List source asset of a received share."]
        #[doc = "List received assets for a received share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `received_share_name`: The name of the received share"]
        pub fn list(&self, received_share_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                received_share_name: received_share_name.into(),
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ReceivedAssetList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) received_share_name: String,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/receivedShares/{}/receivedAssets",
                            this.client.endpoint(),
                            &this.received_share_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
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
                                let rsp_value: models::ReceivedAssetList = serde_json::from_slice(&rsp_body)?;
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
pub mod sent_shares {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of sent shares in the given Purview account."]
        #[doc = "Get list of sent shares"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Get a sent share in the given Purview account."]
        #[doc = "Get a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        pub fn get(&self, sent_share_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
            }
        }
        #[doc = "Create a sent share in the given Purview account."]
        #[doc = "Create a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `sent_share`: The sent share payload"]
        pub fn create_or_update(
            &self,
            sent_share_name: impl Into<String>,
            sent_share: impl Into<models::SentShare>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                sent_share: sent_share.into(),
            }
        }
        #[doc = "Deletes a sent share"]
        #[doc = "Delete a sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        pub fn delete(&self, sent_share_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SentShareList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/sentShares", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::SentShareList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SentShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sentShares/{}", this.client.endpoint(), &this.sent_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShare = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::SentShare),
            Ok200(models::SentShare),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) sent_share: models::SentShare,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sentShares/{}", this.client.endpoint(), &this.sent_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.sent_share)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShare = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShare = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
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
            pub(crate) sent_share_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sentShares/{}", this.client.endpoint(), &this.sent_share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
pub mod accepted_sent_shares {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List of accepted shares for the current sent share."]
        #[doc = "Get list of accepted sent shares"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        pub fn list(&self, sent_share_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                skip_token: None,
            }
        }
        #[doc = "Get an accepted share with acceptedSentShareName to a particular sent share."]
        #[doc = "Get an accepted sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `accepted_sent_share_name`: The name of the accepted sent share"]
        pub fn get(&self, sent_share_name: impl Into<String>, accepted_sent_share_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                accepted_sent_share_name: accepted_sent_share_name.into(),
            }
        }
        #[doc = "Reinstate a revoked accepted sent share."]
        #[doc = "Reinstate a revoked accepted sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `accepted_sent_share_name`: The name of the accepted sent share"]
        #[doc = "* `accepted_sent_share`: The accepted sent share payload"]
        pub fn reinstate(
            &self,
            sent_share_name: impl Into<String>,
            accepted_sent_share_name: impl Into<String>,
            accepted_sent_share: impl Into<models::AcceptedSentShare>,
        ) -> reinstate::Builder {
            reinstate::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                accepted_sent_share_name: accepted_sent_share_name.into(),
                accepted_sent_share: accepted_sent_share.into(),
                repeatability_request_id: None,
            }
        }
        #[doc = "Revoke an accepted sent share's access"]
        #[doc = "Revoke an accepted sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `accepted_sent_share_name`: The name of the accepted sent share"]
        pub fn revoke(&self, sent_share_name: impl Into<String>, accepted_sent_share_name: impl Into<String>) -> revoke::Builder {
            revoke::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                accepted_sent_share_name: accepted_sent_share_name.into(),
                repeatability_request_id: None,
            }
        }
        #[doc = "Update the expiration date of an active accepted sent share."]
        #[doc = "Update the expiration date of an accepted sent share"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `accepted_sent_share_name`: The name of the accepted sent share"]
        #[doc = "* `accepted_sent_share`: The accepted sent share payload"]
        pub fn update_expiration(
            &self,
            sent_share_name: impl Into<String>,
            accepted_sent_share_name: impl Into<String>,
            accepted_sent_share: impl Into<models::AcceptedSentShare>,
        ) -> update_expiration::Builder {
            update_expiration::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                accepted_sent_share_name: accepted_sent_share_name.into(),
                accepted_sent_share: accepted_sent_share.into(),
                repeatability_request_id: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AcceptedSentShareList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/acceptedSentShares",
                            this.client.endpoint(),
                            &this.sent_share_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
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
                                let rsp_value: models::AcceptedSentShareList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AcceptedSentShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) accepted_sent_share_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/acceptedSentShares/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.accepted_sent_share_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AcceptedSentShare = serde_json::from_slice(&rsp_body)?;
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
    pub mod reinstate {
        use super::models;
        type Response = models::AcceptedSentShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) accepted_sent_share_name: String,
            pub(crate) accepted_sent_share: models::AcceptedSentShare,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/acceptedSentShares/{}:reinstate",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.accepted_sent_share_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.accepted_sent_share)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AcceptedSentShare = serde_json::from_slice(&rsp_body)?;
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
    pub mod revoke {
        use super::models;
        type Response = models::AcceptedSentShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) accepted_sent_share_name: String,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/acceptedSentShares/{}:revoke",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.accepted_sent_share_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AcceptedSentShare = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_expiration {
        use super::models;
        type Response = models::AcceptedSentShare;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) accepted_sent_share_name: String,
            pub(crate) accepted_sent_share: models::AcceptedSentShare,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/acceptedSentShares/{}:update-expiration",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.accepted_sent_share_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.accepted_sent_share)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AcceptedSentShare = serde_json::from_slice(&rsp_body)?;
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
pub mod sent_share_invitations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all Invitations in a share."]
        #[doc = "List sent share invitations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        pub fn list(&self, sent_share_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                skip_token: None,
                filter: None,
                orderby: None,
            }
        }
        #[doc = "Get Invitation for a given share."]
        #[doc = "Get a sent share invitation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `sent_share_invitation_name`: Name of the sent invitation"]
        pub fn get(&self, sent_share_name: impl Into<String>, sent_share_invitation_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                sent_share_invitation_name: sent_share_invitation_name.into(),
            }
        }
        #[doc = "Create/Update a sent share invitation in the given account."]
        #[doc = "Create a sent share invitation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `sent_share_invitation_name`: Name of the sent invitation"]
        #[doc = "* `sent_share_invitation`: The sent share invitation to create"]
        pub fn create_or_update(
            &self,
            sent_share_name: impl Into<String>,
            sent_share_invitation_name: impl Into<String>,
            sent_share_invitation: impl Into<models::SentShareInvitation>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                sent_share_invitation_name: sent_share_invitation_name.into(),
                sent_share_invitation: sent_share_invitation.into(),
            }
        }
        #[doc = "Delete Invitation in a share."]
        #[doc = "Delete a sent share invitation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `sent_share_name`: The name of the sent share"]
        #[doc = "* `sent_share_invitation_name`: Name of the sent invitation"]
        pub fn delete(&self, sent_share_name: impl Into<String>, sent_share_invitation_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                sent_share_name: sent_share_name.into(),
                sent_share_invitation_name: sent_share_invitation_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SentShareInvitationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) skip_token: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token to list the next page"]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "Filters the results using OData syntax"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Sorts the results using OData syntax"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/sentShareInvitations",
                            this.client.endpoint(),
                            &this.sent_share_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("skipToken", skip_token);
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
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
                                let rsp_value: models::SentShareInvitationList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SentShareInvitation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) sent_share_invitation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/sentShareInvitations/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.sent_share_invitation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShareInvitation = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::SentShareInvitation),
            Ok200(models::SentShareInvitation),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) sent_share_name: String,
            pub(crate) sent_share_invitation_name: String,
            pub(crate) sent_share_invitation: models::SentShareInvitation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/sentShareInvitations/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.sent_share_invitation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.sent_share_invitation)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShareInvitation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SentShareInvitation = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
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
            pub(crate) sent_share_name: String,
            pub(crate) sent_share_invitation_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sentShares/{}/sentShareInvitations/{}",
                            this.client.endpoint(),
                            &this.sent_share_name,
                            &this.sent_share_invitation_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
pub mod email_registration {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Activates the tenant and email combination using the activation code received."]
        #[doc = "Activates the email registration for current tenant"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `tenant_email_registration`: The tenant email registration payload"]
        pub fn activate(&self, tenant_email_registration: impl Into<models::TenantEmailRegistration>) -> activate::Builder {
            activate::Builder {
                client: self.0.clone(),
                tenant_email_registration: tenant_email_registration.into(),
                repeatability_request_id: None,
            }
        }
        #[doc = "Registers the tenant and email combination for activation."]
        #[doc = "Register an email for the current tenant"]
        pub fn register(&self) -> register::Builder {
            register::Builder {
                client: self.0.clone(),
                repeatability_request_id: None,
            }
        }
    }
    pub mod activate {
        use super::models;
        type Response = models::TenantEmailRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) tenant_email_registration: models::TenantEmailRegistration,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/activateEmail", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.tenant_email_registration)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TenantEmailRegistration = serde_json::from_slice(&rsp_body)?;
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
    pub mod register {
        use super::models;
        type Response = models::TenantEmailRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) repeatability_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "If specified, the client directs that the request is repeatable; that is, that the client can make the request multiple times with the same Repeatability-Request-Id and get back an appropriate response without the server executing the request multiple times. The value of the Repeatability-Request-Id is an opaque string representing a client-generated, globally unique for all time, identifier for the request. It is recommended to use version 4 (random) UUIDs."]
            pub fn repeatability_request_id(mut self, repeatability_request_id: impl Into<String>) -> Self {
                self.repeatability_request_id = Some(repeatability_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/registerEmail", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        if let Some(repeatability_request_id) = &this.repeatability_request_id {
                            req.insert_header("repeatability-request-id", repeatability_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TenantEmailRegistration = serde_json::from_slice(&rsp_body)?;
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
