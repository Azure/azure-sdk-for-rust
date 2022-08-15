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
pub const DEFAULT_ENDPOINT: &str = "https://acrapi.azurecr-test.io";
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
    #[doc = "Tells whether this Docker Registry instance supports Docker Registry HTTP API v2"]
    pub fn get_docker_registry_v2_support(&self) -> get_docker_registry_v2_support::Builder {
        get_docker_registry_v2_support::Builder { client: self.clone() }
    }
    #[doc = "Fetch the tags under the repository identified by 'name'"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn get_tag_list(&self, name: impl Into<String>) -> get_tag_list::Builder {
        get_tag_list::Builder {
            client: self.clone(),
            name: name.into(),
        }
    }
    #[doc = "Pulls the image manifest file associated with the specified name and reference. Reference may be a tag or a digest"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
    pub fn get_manifest(&self, name: impl Into<String>, reference: impl Into<String>) -> get_manifest::Builder {
        get_manifest::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
        }
    }
    #[doc = "List repositories"]
    pub fn get_repositories(&self) -> get_repositories::Builder {
        get_repositories::Builder {
            client: self.clone(),
            last: None,
            n: None,
        }
    }
    #[doc = "List repositories"]
    pub fn get_acr_repositories(&self) -> get_acr_repositories::Builder {
        get_acr_repositories::Builder {
            client: self.clone(),
            last: None,
            n: None,
        }
    }
    #[doc = "Get repository attributes"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn get_acr_repository_attributes(&self, name: impl Into<String>) -> get_acr_repository_attributes::Builder {
        get_acr_repository_attributes::Builder {
            client: self.clone(),
            name: name.into(),
        }
    }
    #[doc = "Update attributes of a repository"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn update_acr_repository_attributes(&self, name: impl Into<String>) -> update_acr_repository_attributes::Builder {
        update_acr_repository_attributes::Builder {
            client: self.clone(),
            name: name.into(),
            value: None,
        }
    }
    #[doc = "Delete a repository"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn delete_acr_repository(&self, name: impl Into<String>) -> delete_acr_repository::Builder {
        delete_acr_repository::Builder {
            client: self.clone(),
            name: name.into(),
        }
    }
    #[doc = "List tags of a repository"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn get_acr_tags(&self, name: impl Into<String>) -> get_acr_tags::Builder {
        get_acr_tags::Builder {
            client: self.clone(),
            name: name.into(),
            last: None,
            n: None,
            orderby: None,
            digest: None,
        }
    }
    #[doc = "Get manifest attributes by tag"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A tag name of the image"]
    pub fn get_acr_tag_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> get_acr_tag_attributes::Builder {
        get_acr_tag_attributes::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
        }
    }
    #[doc = "Update tag attributes"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A tag name of the image"]
    pub fn update_acr_tag_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> update_acr_tag_attributes::Builder {
        update_acr_tag_attributes::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
            value: None,
        }
    }
    #[doc = "Delete tag"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A tag name of the image"]
    pub fn delete_acr_tag(&self, name: impl Into<String>, reference: impl Into<String>) -> delete_acr_tag::Builder {
        delete_acr_tag::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
        }
    }
    #[doc = "List manifests of a repository"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    pub fn get_acr_manifests(&self, name: impl Into<String>) -> get_acr_manifests::Builder {
        get_acr_manifests::Builder {
            client: self.clone(),
            name: name.into(),
            last: None,
            n: None,
            orderby: None,
        }
    }
    #[doc = "Get manifest attributes"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A digest pointing to a specific image"]
    pub fn get_acr_manifest_attributes(
        &self,
        name: impl Into<String>,
        reference: impl Into<String>,
    ) -> get_acr_manifest_attributes::Builder {
        get_acr_manifest_attributes::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
        }
    }
    #[doc = "Update attributes of a manifest"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `name`: Name of the image (including the namespace)"]
    #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
    pub fn update_acr_manifest_attributes(
        &self,
        name: impl Into<String>,
        reference: impl Into<String>,
    ) -> update_acr_manifest_attributes::Builder {
        update_acr_manifest_attributes::Builder {
            client: self.clone(),
            name: name.into(),
            reference: reference.into(),
            value: None,
        }
    }
}
pub mod get_docker_registry_v2_support {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/v2/", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
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
pub mod get_tag_list {
    use super::models;
    type Response = models::RepositoryTags;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/v2/{}/tags/list", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::RepositoryTags = serde_json::from_slice(&rsp_body)?;
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
pub mod get_manifest {
    use super::models;
    type Response = models::Manifest;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/v2/{}/manifests/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Manifest = serde_json::from_slice(&rsp_body)?;
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
pub mod get_repositories {
    use super::models;
    type Response = models::Repositories;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) last: Option<String>,
        pub(crate) n: Option<String>,
    }
    impl Builder {
        #[doc = "query parameter for the last item in previous query"]
        pub fn last(mut self, last: impl Into<String>) -> Self {
            self.last = Some(last.into());
            self
        }
        #[doc = "query parameter for max number of items"]
        pub fn n(mut self, n: impl Into<String>) -> Self {
            self.n = Some(n.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/v2/_catalog", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    if let Some(last) = &this.last {
                        req.url_mut().query_pairs_mut().append_pair("last", last);
                    }
                    if let Some(n) = &this.n {
                        req.url_mut().query_pairs_mut().append_pair("n", n);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Repositories = serde_json::from_slice(&rsp_body)?;
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
pub mod get_acr_repositories {
    use super::models;
    type Response = models::Repositories;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) last: Option<String>,
        pub(crate) n: Option<String>,
    }
    impl Builder {
        #[doc = "query parameter for the last item in previous query"]
        pub fn last(mut self, last: impl Into<String>) -> Self {
            self.last = Some(last.into());
            self
        }
        #[doc = "query parameter for max number of items"]
        pub fn n(mut self, n: impl Into<String>) -> Self {
            self.n = Some(n.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/_catalog", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    if let Some(last) = &this.last {
                        req.url_mut().query_pairs_mut().append_pair("last", last);
                    }
                    if let Some(n) = &this.n {
                        req.url_mut().query_pairs_mut().append_pair("n", n);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Repositories = serde_json::from_slice(&rsp_body)?;
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
pub mod get_acr_repository_attributes {
    use super::models;
    type Response = models::RepositoryAttributes;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/{}", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::RepositoryAttributes = serde_json::from_slice(&rsp_body)?;
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
pub mod update_acr_repository_attributes {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) value: Option<models::ChangeableAttributes>,
    }
    impl Builder {
        #[doc = "Repository attribute value"]
        pub fn value(mut self, value: impl Into<models::ChangeableAttributes>) -> Self {
            self.value = Some(value.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/{}", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = if let Some(value) = &this.value {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(value)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
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
pub mod delete_acr_repository {
    use super::models;
    type Response = models::DeletedRepository;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/{}", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Accepted => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedRepository = serde_json::from_slice(&rsp_body)?;
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
pub mod get_acr_tags {
    use super::models;
    type Response = models::AcrRepositoryTags;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) last: Option<String>,
        pub(crate) n: Option<String>,
        pub(crate) orderby: Option<String>,
        pub(crate) digest: Option<String>,
    }
    impl Builder {
        #[doc = "query parameter for the last item in previous query"]
        pub fn last(mut self, last: impl Into<String>) -> Self {
            self.last = Some(last.into());
            self
        }
        #[doc = "query parameter for max number of items"]
        pub fn n(mut self, n: impl Into<String>) -> Self {
            self.n = Some(n.into());
            self
        }
        #[doc = "orderby query parameter"]
        pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
            self.orderby = Some(orderby.into());
            self
        }
        #[doc = "filter by digest"]
        pub fn digest(mut self, digest: impl Into<String>) -> Self {
            self.digest = Some(digest.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/{}/_tags", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    if let Some(last) = &this.last {
                        req.url_mut().query_pairs_mut().append_pair("last", last);
                    }
                    if let Some(n) = &this.n {
                        req.url_mut().query_pairs_mut().append_pair("n", n);
                    }
                    if let Some(orderby) = &this.orderby {
                        req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                    }
                    if let Some(digest) = &this.digest {
                        req.url_mut().query_pairs_mut().append_pair("digest", digest);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::AcrRepositoryTags = serde_json::from_slice(&rsp_body)?;
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
pub mod get_acr_tag_attributes {
    use super::models;
    type Response = models::AcrTagAttributes;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/acr/v1/{}/_tags/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::AcrTagAttributes = serde_json::from_slice(&rsp_body)?;
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
pub mod update_acr_tag_attributes {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
        pub(crate) value: Option<models::ChangeableAttributes>,
    }
    impl Builder {
        #[doc = "Changeable attribute value"]
        pub fn value(mut self, value: impl Into<models::ChangeableAttributes>) -> Self {
            self.value = Some(value.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/acr/v1/{}/_tags/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = if let Some(value) = &this.value {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(value)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
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
pub mod delete_acr_tag {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/acr/v1/{}/_tags/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
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
pub mod get_acr_manifests {
    use super::models;
    type Response = models::AcrManifests;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) last: Option<String>,
        pub(crate) n: Option<String>,
        pub(crate) orderby: Option<String>,
    }
    impl Builder {
        #[doc = "query parameter for the last item in previous query"]
        pub fn last(mut self, last: impl Into<String>) -> Self {
            self.last = Some(last.into());
            self
        }
        #[doc = "query parameter for max number of items"]
        pub fn n(mut self, n: impl Into<String>) -> Self {
            self.n = Some(n.into());
            self
        }
        #[doc = "orderby query parameter"]
        pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
            self.orderby = Some(orderby.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/acr/v1/{}/_manifests", this.client.endpoint(), &this.name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    if let Some(last) = &this.last {
                        req.url_mut().query_pairs_mut().append_pair("last", last);
                    }
                    if let Some(n) = &this.n {
                        req.url_mut().query_pairs_mut().append_pair("n", n);
                    }
                    if let Some(orderby) = &this.orderby {
                        req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::AcrManifests = serde_json::from_slice(&rsp_body)?;
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
pub mod get_acr_manifest_attributes {
    use super::models;
    type Response = models::AcrManifestAttributes;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/acr/v1/{}/_manifests/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::AcrManifestAttributes = serde_json::from_slice(&rsp_body)?;
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
pub mod update_acr_manifest_attributes {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) name: String,
        pub(crate) reference: String,
        pub(crate) value: Option<models::ChangeableAttributes>,
    }
    impl Builder {
        #[doc = "Changeable attribute value"]
        pub fn value(mut self, value: impl Into<models::ChangeableAttributes>) -> Self {
            self.value = Some(value.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/acr/v1/{}/_manifests/{}",
                        this.client.endpoint(),
                        &this.name,
                        &this.reference
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = if let Some(value) = &this.value {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(value)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
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
