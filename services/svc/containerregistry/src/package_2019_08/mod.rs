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
    pub fn access_tokens_client(&self) -> access_tokens::Client {
        access_tokens::Client(self.clone())
    }
    pub fn blob_client(&self) -> blob::Client {
        blob::Client(self.clone())
    }
    pub fn manifests_client(&self) -> manifests::Client {
        manifests::Client(self.clone())
    }
    pub fn refresh_tokens_client(&self) -> refresh_tokens::Client {
        refresh_tokens::Client(self.clone())
    }
    pub fn repository_client(&self) -> repository::Client {
        repository::Client(self.clone())
    }
    pub fn tag_client(&self) -> tag::Client {
        tag::Client(self.clone())
    }
    pub fn v2_support_client(&self) -> v2_support::Client {
        v2_support::Client(self.clone())
    }
}
pub mod blob {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Initiate a resumable blob upload with an empty request body."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn start_upload(&self, name: impl Into<String>) -> start_upload::Builder {
            start_upload::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Retrieve the blob from the registry identified by `digest`. This endpoint may also support RFC7233 compliant range requests. Support can be detected by issuing a HEAD request. If the header `Accept-Range: bytes` is returned, range requests can be used to fetch partial content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `digest`: Digest of a BLOB"]
        #[doc = "* `range`: Format : bytes=<start>-<end>,  HTTP Range header specifying blob chunk."]
        pub fn get_chunk(&self, name: impl Into<String>, digest: impl Into<String>, range: impl Into<String>) -> get_chunk::Builder {
            get_chunk::Builder {
                client: self.0.clone(),
                name: name.into(),
                digest: digest.into(),
                range: range.into(),
            }
        }
        #[doc = "Same as GET, except only the headers are returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `digest`: Digest of a BLOB"]
        #[doc = "* `range`: Format : bytes=<start>-<end>,  HTTP Range header specifying blob chunk."]
        pub fn check_chunk(&self, name: impl Into<String>, digest: impl Into<String>, range: impl Into<String>) -> check_chunk::Builder {
            check_chunk::Builder {
                client: self.0.clone(),
                name: name.into(),
                digest: digest.into(),
                range: range.into(),
            }
        }
        #[doc = "Retrieve the blob from the registry identified by digest."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `digest`: Digest of a BLOB"]
        pub fn get(&self, name: impl Into<String>, digest: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                name: name.into(),
                digest: digest.into(),
            }
        }
        #[doc = "Removes an already uploaded blob."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `digest`: Digest of a BLOB"]
        pub fn delete(&self, name: impl Into<String>, digest: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                name: name.into(),
                digest: digest.into(),
            }
        }
        #[doc = "Same as GET, except only the headers are returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `digest`: Digest of a BLOB"]
        pub fn check(&self, name: impl Into<String>, digest: impl Into<String>) -> check::Builder {
            check::Builder {
                client: self.0.clone(),
                name: name.into(),
                digest: digest.into(),
            }
        }
        #[doc = "Mount a blob identified by the `mount` parameter from another repository."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `from`: Name of the source repository."]
        #[doc = "* `mount`: Digest of blob to mount from the source repository."]
        pub fn mount(&self, name: impl Into<String>, from: impl Into<String>, mount: impl Into<String>) -> mount::Builder {
            mount::Builder {
                client: self.0.clone(),
                name: name.into(),
                from: from.into(),
                mount: mount.into(),
            }
        }
        #[doc = "Retrieve status of upload identified by uuid. The primary purpose of this endpoint is to resolve the current status of a resumable upload."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `next_blob_uuid_link`: Link acquired from upload start or previous chunk. Note, do not include initial / (must do substring(1) )"]
        pub fn get_status(&self, next_blob_uuid_link: impl Into<String>) -> get_status::Builder {
            get_status::Builder {
                client: self.0.clone(),
                next_blob_uuid_link: next_blob_uuid_link.into(),
            }
        }
        #[doc = "Complete the upload, providing all the data in the body, if necessary. A request without a body will just complete the upload with previously uploaded content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `digest`: Digest of a BLOB"]
        #[doc = "* `next_blob_uuid_link`: Link acquired from upload start or previous chunk. Note, do not include initial / (must do substring(1) )"]
        pub fn end_upload(&self, digest: impl Into<String>, next_blob_uuid_link: impl Into<String>) -> end_upload::Builder {
            end_upload::Builder {
                client: self.0.clone(),
                digest: digest.into(),
                next_blob_uuid_link: next_blob_uuid_link.into(),
                value: None,
            }
        }
        #[doc = "Upload a stream of data without completing the upload."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `value`: Raw data of blob"]
        #[doc = "* `next_blob_uuid_link`: Link acquired from upload start or previous chunk. Note, do not include initial / (must do substring(1) )"]
        pub fn upload(&self, value: impl Into<serde_json::Value>, next_blob_uuid_link: impl Into<String>) -> upload::Builder {
            upload::Builder {
                client: self.0.clone(),
                value: value.into(),
                next_blob_uuid_link: next_blob_uuid_link.into(),
            }
        }
        #[doc = "Cancel outstanding upload processes, releasing associated resources. If this is not called, the unfinished uploads will eventually timeout."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `next_blob_uuid_link`: Link acquired from upload start or previous chunk. Note, do not include initial / (must do substring(1) )"]
        pub fn cancel_upload(&self, next_blob_uuid_link: impl Into<String>) -> cancel_upload::Builder {
            cancel_upload::Builder {
                client: self.0.clone(),
                next_blob_uuid_link: next_blob_uuid_link.into(),
            }
        }
    }
    pub mod start_upload {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/v2/{}/blobs/uploads/?mode=resumable",
                            this.client.endpoint(),
                            &this.name
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
    pub mod get_chunk {
        use super::models;
        type Response = bytes::Bytes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) digest: String,
            pub(crate) range: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/v2/{}/blobs/{}?mode=chunk",
                            this.client.endpoint(),
                            &this.name,
                            &this.digest
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("range", &this.range);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::PartialContent => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value = rsp_body;
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
    pub mod check_chunk {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) digest: String,
            pub(crate) range: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/v2/{}/blobs/{}?mode=chunk",
                            this.client.endpoint(),
                            &this.name,
                            &this.digest
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("range", &this.range);
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
        type Response = bytes::Bytes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) digest: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/v2/{}/blobs/{}", this.client.endpoint(), &this.name, &this.digest))?;
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
                                let rsp_value = rsp_body;
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
        type Response = bytes::Bytes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) digest: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/v2/{}/blobs/{}", this.client.endpoint(), &this.name, &this.digest))?;
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
                                let rsp_value = rsp_body;
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
    pub mod check {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) digest: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/v2/{}/blobs/{}", this.client.endpoint(), &this.name, &this.digest))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
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
    pub mod mount {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) from: String,
            pub(crate) mount: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/v2/{}/blobs/uploads/", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let from = &this.from;
                        req.url_mut().query_pairs_mut().append_pair("from", from);
                        let mount = &this.mount;
                        req.url_mut().query_pairs_mut().append_pair("mount", mount);
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
    pub mod get_status {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) next_blob_uuid_link: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.next_blob_uuid_link))?;
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
    pub mod end_upload {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) digest: String,
            pub(crate) next_blob_uuid_link: String,
            pub(crate) value: Option<serde_json::Value>,
        }
        impl Builder {
            #[doc = "Optional raw data of blob"]
            pub fn value(mut self, value: impl Into<serde_json::Value>) -> Self {
                self.value = Some(value.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.next_blob_uuid_link))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let digest = &this.digest;
                        req.url_mut().query_pairs_mut().append_pair("digest", digest);
                        let req_body = if let Some(value) = &this.value {
                            req.insert_header("content-type", "application/octet-stream");
                            azure_core::to_json(value)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod upload {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) value: serde_json::Value,
            pub(crate) next_blob_uuid_link: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.next_blob_uuid_link))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.value)?;
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
    pub mod cancel_upload {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) next_blob_uuid_link: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.next_blob_uuid_link))?;
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
}
pub mod v2_support {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Tells whether this Docker Registry instance supports Docker Registry HTTP API v2"]
        pub fn check(&self) -> check::Builder {
            check::Builder { client: self.0.clone() }
        }
    }
    pub mod check {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
}
pub mod manifests {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the manifest identified by `name` and `reference` where `reference` can be a tag or digest."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
        pub fn get(&self, name: impl Into<String>, reference: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
                accept: None,
            }
        }
        #[doc = "Put the manifest identified by `name` and `reference` where `reference` can be a tag or digest."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
        #[doc = "* `payload`: Manifest body, can take v1 or v2 values depending on accept header"]
        pub fn create(
            &self,
            name: impl Into<String>,
            reference: impl Into<String>,
            payload: impl Into<models::Manifest>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
                payload: payload.into(),
            }
        }
        #[doc = "Delete the manifest identified by `name` and `reference`. Note that a manifest can _only_ be deleted by `digest`."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
        pub fn delete(&self, name: impl Into<String>, reference: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
            }
        }
        #[doc = "List manifests of a repository"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn get_list(&self, name: impl Into<String>) -> get_list::Builder {
            get_list::Builder {
                client: self.0.clone(),
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
        #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
        pub fn get_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> get_attributes::Builder {
            get_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
            }
        }
        #[doc = "Update attributes of a manifest"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: A tag or a digest, pointing to a specific image"]
        pub fn update_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> update_attributes::Builder {
            update_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
                value: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ManifestWrapper;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) reference: String,
            pub(crate) accept: Option<String>,
        }
        impl Builder {
            #[doc = "Accept header string delimited by comma. For example, application/vnd.docker.distribution.manifest.v2+json"]
            pub fn accept(mut self, accept: impl Into<String>) -> Self {
                self.accept = Some(accept.into());
                self
            }
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
                        if let Some(accept) = &this.accept {
                            req.insert_header("accept", accept);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ManifestWrapper = serde_json::from_slice(&rsp_body)?;
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
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) reference: String,
            pub(crate) payload: models::Manifest,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/vnd.docker.distribution.manifest.v2+json");
                        let req_body = azure_core::to_json(&this.payload)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
    pub mod get_list {
        use super::models;
        type Response = models::AcrManifests;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) last: Option<String>,
            pub(crate) n: Option<i64>,
            pub(crate) orderby: Option<String>,
        }
        impl Builder {
            #[doc = "Query parameter for the last item in previous query. Result set will include values lexically after last."]
            pub fn last(mut self, last: impl Into<String>) -> Self {
                self.last = Some(last.into());
                self
            }
            #[doc = "query parameter for max number of items"]
            pub fn n(mut self, n: i64) -> Self {
                self.n = Some(n);
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
                            req.url_mut().query_pairs_mut().append_pair("n", &n.to_string());
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
    pub mod get_attributes {
        use super::models;
        type Response = models::ManifestAttributes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
                                let rsp_value: models::ManifestAttributes = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_attributes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) reference: String,
            pub(crate) value: Option<models::ManifestChangeableAttributes>,
        }
        impl Builder {
            #[doc = "Manifest attribute value"]
            pub fn value(mut self, value: impl Into<models::ManifestChangeableAttributes>) -> Self {
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
pub mod repository {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List repositories"]
        pub fn get_list(&self) -> get_list::Builder {
            get_list::Builder {
                client: self.0.clone(),
                last: None,
                n: None,
            }
        }
        #[doc = "Get repository attributes"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn get_attributes(&self, name: impl Into<String>) -> get_attributes::Builder {
            get_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Update the attribute identified by `name` where `reference` is the name of the repository."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn update_attributes(&self, name: impl Into<String>) -> update_attributes::Builder {
            update_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
                value: None,
            }
        }
        #[doc = "Delete the repository identified by `name`"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn delete(&self, name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
    }
    pub mod get_list {
        use super::models;
        type Response = models::Repositories;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) last: Option<String>,
            pub(crate) n: Option<i64>,
        }
        impl Builder {
            #[doc = "Query parameter for the last item in previous query. Result set will include values lexically after last."]
            pub fn last(mut self, last: impl Into<String>) -> Self {
                self.last = Some(last.into());
                self
            }
            #[doc = "query parameter for max number of items"]
            pub fn n(mut self, n: i64) -> Self {
                self.n = Some(n);
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
                            req.url_mut().query_pairs_mut().append_pair("n", &n.to_string());
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
    pub mod get_attributes {
        use super::models;
        type Response = models::RepositoryAttributes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
    pub mod update_attributes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) value: Option<models::RepositoryChangeableAttributes>,
        }
        impl Builder {
            #[doc = "Repository attribute value"]
            pub fn value(mut self, value: impl Into<models::RepositoryChangeableAttributes>) -> Self {
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
        type Response = models::DeletedRepository;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
}
pub mod tag {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List tags of a repository"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        pub fn get_list(&self, name: impl Into<String>) -> get_list::Builder {
            get_list::Builder {
                client: self.0.clone(),
                name: name.into(),
                last: None,
                n: None,
                orderby: None,
                digest: None,
            }
        }
        #[doc = "Get tag attributes by tag"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: Tag name"]
        pub fn get_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> get_attributes::Builder {
            get_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
            }
        }
        #[doc = "Update tag attributes"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: Tag name"]
        pub fn update_attributes(&self, name: impl Into<String>, reference: impl Into<String>) -> update_attributes::Builder {
            update_attributes::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
                value: None,
            }
        }
        #[doc = "Delete tag"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: Name of the image (including the namespace)"]
        #[doc = "* `reference`: Tag name"]
        pub fn delete(&self, name: impl Into<String>, reference: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                name: name.into(),
                reference: reference.into(),
            }
        }
    }
    pub mod get_list {
        use super::models;
        type Response = models::TagList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) last: Option<String>,
            pub(crate) n: Option<i64>,
            pub(crate) orderby: Option<String>,
            pub(crate) digest: Option<String>,
        }
        impl Builder {
            #[doc = "Query parameter for the last item in previous query. Result set will include values lexically after last."]
            pub fn last(mut self, last: impl Into<String>) -> Self {
                self.last = Some(last.into());
                self
            }
            #[doc = "query parameter for max number of items"]
            pub fn n(mut self, n: i64) -> Self {
                self.n = Some(n);
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
                            req.url_mut().query_pairs_mut().append_pair("n", &n.to_string());
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
                                let rsp_value: models::TagList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_attributes {
        use super::models;
        type Response = models::TagAttributes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
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
                                let rsp_value: models::TagAttributes = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_attributes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
            pub(crate) reference: String,
            pub(crate) value: Option<models::TagChangeableAttributes>,
        }
        impl Builder {
            #[doc = "Tag attribute value"]
            pub fn value(mut self, value: impl Into<models::TagChangeableAttributes>) -> Self {
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
pub mod refresh_tokens {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Exchange AAD tokens for an ACR refresh Token"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `grant_type`: Can take a value of access_token_refresh_token, or access_token, or refresh_token"]
        #[doc = "* `service`: Indicates the name of your Azure container registry."]
        pub fn get_from_exchange(&self, grant_type: impl Into<String>, service: impl Into<String>) -> get_from_exchange::Builder {
            get_from_exchange::Builder {
                client: self.0.clone(),
                grant_type: grant_type.into(),
                service: service.into(),
                tenant: None,
                refresh_token: None,
                access_token: None,
            }
        }
    }
    pub mod get_from_exchange {
        use super::models;
        type Response = models::RefreshToken;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) grant_type: String,
            pub(crate) service: String,
            pub(crate) tenant: Option<String>,
            pub(crate) refresh_token: Option<String>,
            pub(crate) access_token: Option<String>,
        }
        impl Builder {
            #[doc = "AAD tenant associated to the AAD credentials."]
            pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
                self.tenant = Some(tenant.into());
                self
            }
            #[doc = "AAD refresh token, mandatory when grant_type is access_token_refresh_token or refresh_token"]
            pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
                self.refresh_token = Some(refresh_token.into());
                self
            }
            #[doc = "AAD access token, mandatory when grant_type is access_token_refresh_token or access_token."]
            pub fn access_token(mut self, access_token: impl Into<String>) -> Self {
                self.access_token = Some(access_token.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/oauth2/exchange", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RefreshToken = serde_json::from_slice(&rsp_body)?;
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
pub mod access_tokens {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Exchange Username, Password and Scope an ACR Access Token"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service`: Indicates the name of your Azure container registry."]
        #[doc = "* `scope`: Expected to be a valid scope, and can be specified more than once for multiple scope requests. You can obtain this from the Www-Authenticate response header from the challenge."]
        pub fn get_from_login(&self, service: impl Into<String>, scope: impl Into<String>) -> get_from_login::Builder {
            get_from_login::Builder {
                client: self.0.clone(),
                service: service.into(),
                scope: scope.into(),
            }
        }
        #[doc = "Exchange ACR Refresh token for an ACR Access Token"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `grant_type`: Grant type is expected to be refresh_token"]
        #[doc = "* `service`: Indicates the name of your Azure container registry."]
        #[doc = "* `scope`: Which is expected to be a valid scope, and can be specified more than once for multiple scope requests. You obtained this from the Www-Authenticate response header from the challenge."]
        #[doc = "* `refresh_token`: Must be a valid ACR refresh token"]
        pub fn get(
            &self,
            grant_type: impl Into<String>,
            service: impl Into<String>,
            scope: impl Into<String>,
            refresh_token: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                grant_type: grant_type.into(),
                service: service.into(),
                scope: scope.into(),
                refresh_token: refresh_token.into(),
            }
        }
    }
    pub mod get_from_login {
        use super::models;
        type Response = models::AccessToken;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) service: String,
            pub(crate) scope: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/oauth2/token", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let service = &this.service;
                        req.url_mut().query_pairs_mut().append_pair("service", service);
                        let scope = &this.scope;
                        req.url_mut().query_pairs_mut().append_pair("scope", scope);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessToken = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AccessToken;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) grant_type: String,
            pub(crate) service: String,
            pub(crate) scope: String,
            pub(crate) refresh_token: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/oauth2/token", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AccessToken = serde_json::from_slice(&rsp_body)?;
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
