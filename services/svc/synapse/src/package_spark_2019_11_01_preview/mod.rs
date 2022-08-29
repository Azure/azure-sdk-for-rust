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
    pub fn spark_batch_client(&self) -> spark_batch::Client {
        spark_batch::Client(self.clone())
    }
    pub fn spark_session_client(&self) -> spark_session::Client {
        spark_session::Client(self.clone())
    }
}
pub mod spark_batch {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all spark batch jobs which are running under a particular spark pool."]
        pub fn get_spark_batch_jobs(&self) -> get_spark_batch_jobs::RequestBuilder {
            get_spark_batch_jobs::RequestBuilder {
                client: self.0.clone(),
                from: None,
                size: None,
                detailed: None,
            }
        }
        #[doc = "Create new spark batch job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `spark_batch_job_options`: Livy compatible batch job request payload."]
        pub fn create_spark_batch_job(
            &self,
            spark_batch_job_options: impl Into<models::SparkBatchJobOptions>,
        ) -> create_spark_batch_job::RequestBuilder {
            create_spark_batch_job::RequestBuilder {
                client: self.0.clone(),
                spark_batch_job_options: spark_batch_job_options.into(),
                detailed: None,
            }
        }
        #[doc = "Gets a single spark batch job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `batch_id`: Identifier for the batch job."]
        pub fn get_spark_batch_job(&self, batch_id: i32) -> get_spark_batch_job::RequestBuilder {
            get_spark_batch_job::RequestBuilder {
                client: self.0.clone(),
                batch_id,
                detailed: None,
            }
        }
        #[doc = "Cancels a running spark batch job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `batch_id`: Identifier for the batch job."]
        pub fn cancel_spark_batch_job(&self, batch_id: i32) -> cancel_spark_batch_job::RequestBuilder {
            cancel_spark_batch_job::RequestBuilder {
                client: self.0.clone(),
                batch_id,
            }
        }
    }
    pub mod get_spark_batch_jobs {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJobCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkBatchJobCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) from: Option<i32>,
            pub(crate) size: Option<i32>,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional param specifying which index the list should begin from."]
            pub fn from(mut self, from: i32) -> Self {
                self.from = Some(from);
                self
            }
            #[doc = "Optional param specifying the size of the returned list.\r\n            By default it is 20 and that is the maximum."]
            pub fn size(mut self, size: i32) -> Self {
                self.size = Some(size);
                self
            }
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/batches", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                        }
                        if let Some(size) = &this.size {
                            req.url_mut().query_pairs_mut().append_pair("size", &size.to_string());
                        }
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJobCollection> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create_spark_batch_job {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkBatchJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) spark_batch_job_options: models::SparkBatchJobOptions,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/batches", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.spark_batch_job_options)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJob> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod get_spark_batch_job {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJob> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkBatchJob = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) batch_id: i32,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/batches/{}", this.client.endpoint(), &this.batch_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkBatchJob> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod cancel_spark_batch_job {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) batch_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/batches/{}", this.client.endpoint(), &this.batch_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod spark_session {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all spark sessions which are running under a particular spark pool."]
        pub fn get_spark_sessions(&self) -> get_spark_sessions::RequestBuilder {
            get_spark_sessions::RequestBuilder {
                client: self.0.clone(),
                from: None,
                size: None,
                detailed: None,
            }
        }
        #[doc = "Create new spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `spark_session_options`: Livy compatible batch job request payload."]
        pub fn create_spark_session(
            &self,
            spark_session_options: impl Into<models::SparkSessionOptions>,
        ) -> create_spark_session::RequestBuilder {
            create_spark_session::RequestBuilder {
                client: self.0.clone(),
                spark_session_options: spark_session_options.into(),
                detailed: None,
            }
        }
        #[doc = "Gets a single spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        pub fn get_spark_session(&self, session_id: i32) -> get_spark_session::RequestBuilder {
            get_spark_session::RequestBuilder {
                client: self.0.clone(),
                session_id,
                detailed: None,
            }
        }
        #[doc = "Cancels a running spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        pub fn cancel_spark_session(&self, session_id: i32) -> cancel_spark_session::RequestBuilder {
            cancel_spark_session::RequestBuilder {
                client: self.0.clone(),
                session_id,
            }
        }
        #[doc = "Sends a keep alive call to the current session to reset the session timeout."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        pub fn reset_spark_session_timeout(&self, session_id: i32) -> reset_spark_session_timeout::RequestBuilder {
            reset_spark_session_timeout::RequestBuilder {
                client: self.0.clone(),
                session_id,
            }
        }
        #[doc = "Gets a list of statements within a spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        pub fn get_spark_statements(&self, session_id: i32) -> get_spark_statements::RequestBuilder {
            get_spark_statements::RequestBuilder {
                client: self.0.clone(),
                session_id,
            }
        }
        #[doc = "Create statement within a spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        #[doc = "* `spark_statement_options`: Livy compatible batch job request payload."]
        pub fn create_spark_statement(
            &self,
            session_id: i32,
            spark_statement_options: impl Into<models::SparkStatementOptions>,
        ) -> create_spark_statement::RequestBuilder {
            create_spark_statement::RequestBuilder {
                client: self.0.clone(),
                session_id,
                spark_statement_options: spark_statement_options.into(),
            }
        }
        #[doc = "Gets a single statement within a spark session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        #[doc = "* `statement_id`: Identifier for the statement."]
        pub fn get_spark_statement(&self, session_id: i32, statement_id: i32) -> get_spark_statement::RequestBuilder {
            get_spark_statement::RequestBuilder {
                client: self.0.clone(),
                session_id,
                statement_id,
            }
        }
        #[doc = "Kill a statement within a session."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `session_id`: Identifier for the session."]
        #[doc = "* `statement_id`: Identifier for the statement."]
        pub fn cancel_spark_statement(&self, session_id: i32, statement_id: i32) -> cancel_spark_statement::RequestBuilder {
            cancel_spark_statement::RequestBuilder {
                client: self.0.clone(),
                session_id,
                statement_id,
            }
        }
    }
    pub mod get_spark_sessions {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkSessionCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkSessionCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) from: Option<i32>,
            pub(crate) size: Option<i32>,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional param specifying which index the list should begin from."]
            pub fn from(mut self, from: i32) -> Self {
                self.from = Some(from);
                self
            }
            #[doc = "Optional param specifying the size of the returned list.\r\n            By default it is 20 and that is the maximum."]
            pub fn size(mut self, size: i32) -> Self {
                self.size = Some(size);
                self
            }
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(from) = &this.from {
                            req.url_mut().query_pairs_mut().append_pair("from", &from.to_string());
                        }
                        if let Some(size) = &this.size {
                            req.url_mut().query_pairs_mut().append_pair("size", &size.to_string());
                        }
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkSessionCollection> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create_spark_session {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkSession> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkSession = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) spark_session_options: models::SparkSessionOptions,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.spark_session_options)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkSession> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod get_spark_session {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkSession> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkSession = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
            pub(crate) detailed: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Optional query param specifying whether detailed response is returned beyond plain livy."]
            pub fn detailed(mut self, detailed: bool) -> Self {
                self.detailed = Some(detailed);
                self
            }
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions/{}", this.client.endpoint(), &this.session_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(detailed) = &this.detailed {
                            req.url_mut().query_pairs_mut().append_pair("detailed", &detailed.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkSession> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod cancel_spark_session {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions/{}", this.client.endpoint(), &this.session_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod reset_spark_session_timeout {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/sessions/{}/reset-timeout", this.client.endpoint(), &this.session_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_spark_statements {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatementCollection> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkStatementCollection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions/{}/statements", this.client.endpoint(), &this.session_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatementCollection> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod create_spark_statement {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatement> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkStatement = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
            pub(crate) spark_statement_options: models::SparkStatementOptions,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/sessions/{}/statements", this.client.endpoint(), &this.session_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.spark_statement_options)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatement> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod get_spark_statement {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatement> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkStatement = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
            pub(crate) statement_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sessions/{}/statements/{}",
                            this.client.endpoint(),
                            &this.session_id,
                            &this.statement_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatement> {
                self.send().await?.into_body().await
            }
        }
    }
    pub mod cancel_spark_statement {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatementCancellationResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SparkStatementCancellationResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
        }
        #[derive(Clone)]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) session_id: i32,
            pub(crate) statement_id: i32,
        }
        impl RequestBuilder {
            #[doc = "Send the request and returns the response."]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/sessions/{}/statements/{}/cancel",
                            this.client.endpoint(),
                            &this.session_id,
                            &this.statement_id
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
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Send the request and return the response body."]
            pub async fn into_body(self) -> azure_core::Result<models::SparkStatementCancellationResult> {
                self.send().await?.into_body().await
            }
        }
    }
}
