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
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{endpoint}/")]);
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
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
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
    pub fn artifacts_client(&self) -> artifacts::Client {
        artifacts::Client(self.clone())
    }
    pub fn assets_client(&self) -> assets::Client {
        assets::Client(self.clone())
    }
    pub fn events_client(&self) -> events::Client {
        events::Client(self.clone())
    }
    pub fn execution_client(&self) -> execution::Client {
        execution::Client(self.clone())
    }
    pub fn experiments_client(&self) -> experiments::Client {
        experiments::Client(self.clone())
    }
    pub fn hyper_drive_client(&self) -> hyper_drive::Client {
        hyper_drive::Client(self.clone())
    }
    pub fn ml_models_client(&self) -> ml_models::Client {
        ml_models::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn profiles_client(&self) -> profiles::Client {
        profiles::Client(self.clone())
    }
    pub fn run_artifacts_client(&self) -> run_artifacts::Client {
        run_artifacts::Client(self.clone())
    }
    pub fn run_metrics_client(&self) -> run_metrics::Client {
        run_metrics::Client(self.clone())
    }
    pub fn runs_client(&self) -> runs::Client {
        runs::Client(self.clone())
    }
    pub fn services_client(&self) -> services::Client {
        services::Client(self.clone())
    }
}
pub mod execution {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Start a run on a local machine."]
        #[doc = "Starts an experiment run using the provided definition.json file to define the run.\r\n            The source code and configuration is defined in a zip archive in project.zip."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `definition`: A JSON run definition structure."]
        pub fn start_local_run(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            definition: impl Into<models::RunDefinition>,
        ) -> start_local_run::RequestBuilder {
            start_local_run::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                definition: definition.into(),
                run_id: None,
            }
        }
        #[doc = "Start a run on a remote compute target."]
        #[doc = "Starts an experiment run using the provided definition.json file to define the run.\r\n            The source code and configuration is defined in a zip archive in project.zip."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_definition_file`: The JSON file containing the RunDefinition"]
        #[doc = "* `project_zip_file`: The zip archive of the project folder containing the source code to use for the run."]
        pub fn start_run(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_definition_file: impl Into<bytes::Bytes>,
            project_zip_file: impl Into<bytes::Bytes>,
        ) -> start_run::RequestBuilder {
            start_run::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_definition_file: run_definition_file.into(),
                project_zip_file: project_zip_file.into(),
                run_id: None,
            }
        }
        #[doc = "Start a run from a snapshot on a remote compute target."]
        #[doc = "Starts an experiment run on the remote compute target using the provided definition.json file to define the run.\r\n            The code for the run is retrieved using the snapshotId in definition.json."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `definition`: A JSON run definition structure."]
        pub fn start_snapshot_run(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            definition: impl Into<models::RunDefinition>,
        ) -> start_snapshot_run::RequestBuilder {
            start_snapshot_run::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                definition: definition.into(),
                run_id: None,
            }
        }
        #[doc = "Cancel a run."]
        #[doc = "Cancels a run within an experiment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The id of the run to cancel."]
        pub fn cancel_run_with_uri(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> cancel_run_with_uri::RequestBuilder {
            cancel_run_with_uri::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
            }
        }
    }
    pub mod start_local_run {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) definition: models::RunDefinition,
            pub(crate) run_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A run id. If not supplied a run id will be created automatically."]
            pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
                self.run_id = Some(run_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.definition)?;
                        if let Some(run_id) = &this.run_id {
                            req.url_mut().query_pairs_mut().append_pair("runId", run_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/execution/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/startlocalrun" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<bytes::Bytes>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<bytes::Bytes>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod start_run {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StartRunResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StartRunResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_definition_file: bytes::Bytes,
            pub(crate) project_zip_file: bytes::Bytes,
            pub(crate) run_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A run id. If not supplied a run id will be created automatically."]
            pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
                self.run_id = Some(run_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        unimplemented!("form data not yet supported");
                        unimplemented!("form data not yet supported");
                        if let Some(run_id) = &this.run_id {
                            req.url_mut().query_pairs_mut().append_pair("runId", run_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/execution/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/startrun" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::StartRunResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::StartRunResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod start_snapshot_run {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StartRunResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StartRunResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) definition: models::RunDefinition,
            pub(crate) run_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A run id. If not supplied a run id will be created automatically."]
            pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
                self.run_id = Some(run_id.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.definition)?;
                        if let Some(run_id) = &this.run_id {
                            req.url_mut().query_pairs_mut().append_pair("runId", run_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/execution/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/snapshotrun" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::StartRunResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::StartRunResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod cancel_run_with_uri {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StartRunResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StartRunResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/execution/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runId/{}/cancel" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::StartRunResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::StartRunResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod assets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Query the list of Assets in a workspace."]
        #[doc = "If no filter is passed, the query lists all the Assets in the given workspace. The returned list is paginated and the count of items in each page is an optional parameter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        pub fn list_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
        ) -> list_query::RequestBuilder {
            list_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                run_id: None,
                name: None,
                count: None,
                skip_token: None,
                tags: None,
                properties: None,
                orderby: None,
            }
        }
        #[doc = "Create an Asset."]
        #[doc = "Create an Asset from the provided payload."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                asset: None,
            }
        }
        #[doc = "Get an Asset."]
        #[doc = "Get an Asset by Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Asset Id."]
        pub fn query_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> query_by_id::RequestBuilder {
            query_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
        #[doc = "Update an Asset."]
        #[doc = "Patch a specific Asset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Id of the Asset to patch."]
        #[doc = "* `patch`: The payload that is used to patch an Asset."]
        pub fn patch(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
            patch: Vec<models::JsonPatchOperation>,
        ) -> patch::RequestBuilder {
            patch::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                patch,
            }
        }
        #[doc = "Delete an Asset."]
        #[doc = "Delete the specified Asset."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Id of the Asset to delete."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
    }
    pub mod list_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedAssetList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedAssetList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) run_id: Option<String>,
            pub(crate) name: Option<String>,
            pub(crate) count: Option<i32>,
            pub(crate) skip_token: Option<String>,
            pub(crate) tags: Option<String>,
            pub(crate) properties: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The run Id associated with the Assets."]
            pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
                self.run_id = Some(run_id.into());
                self
            }
            #[doc = "The object name."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "The number of items to retrieve in a page."]
            pub fn count(mut self, count: i32) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "The continuation token to retrieve the next page."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "A set of tags with which to filter the returned models.\r\n            It is a comma separated string of tags key or tags key=value\r\n            Example: tagKey1,tagKey2,tagKey3=value3"]
            pub fn tags(mut self, tags: impl Into<String>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "A set of properties with which to filter the returned models.\r\n            It is a comma separated string of properties key and/or properties key=value\r\n            Example: propKey1,propKey2,propKey3=value3"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "An option for specifying how to order the list."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedAssetList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(run_id) = &this.run_id {
                                    req.url_mut().query_pairs_mut().append_pair("runId", run_id);
                                }
                                if let Some(name) = &this.name {
                                    req.url_mut().query_pairs_mut().append_pair("name", name);
                                }
                                if let Some(count) = &this.count {
                                    req.url_mut().query_pairs_mut().append_pair("count", &count.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                if let Some(tags) = &this.tags {
                                    req.url_mut().query_pairs_mut().append_pair("tags", tags);
                                }
                                if let Some(properties) = &this.properties {
                                    req.url_mut().query_pairs_mut().append_pair("properties", properties);
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/assets" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Asset> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Asset = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) asset: Option<models::Asset>,
        }
        impl RequestBuilder {
            #[doc = "The Asset to be created."]
            pub fn asset(mut self, asset: impl Into<models::Asset>) -> Self {
                self.asset = Some(asset.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(asset) = &this.asset {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(asset)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/assets" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Asset>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Asset>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod query_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Asset> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Asset = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/assets/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Asset>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Asset>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Asset> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Asset = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) patch: Vec<models::JsonPatchOperation>,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/assets/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Asset>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Asset>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/assets/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
    }
}
pub mod ml_models {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a model."]
        #[doc = "Gets a model by model id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The model id."]
        pub fn query_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> query_by_id::RequestBuilder {
            query_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
        #[doc = "Patch a specific model."]
        #[doc = "Updates an existing model with the specified patch."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The model id."]
        #[doc = "* `patch`: The payload that is used to patch the model."]
        pub fn patch(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
            patch: Vec<models::JsonPatchOperation>,
        ) -> patch::RequestBuilder {
            patch::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                patch,
            }
        }
        #[doc = "Delete the specified Model."]
        #[doc = "Deletes a model if it exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The model id."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
        #[doc = "Query the list of Models in a workspace."]
        #[doc = "The result list can be filtered using tag and name. If no filter is passed, the query lists all the Models in the given workspace. The returned list is paginated and the count of items in each page is an optional parameter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        pub fn list_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
        ) -> list_query::RequestBuilder {
            list_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                name: None,
                framework: None,
                description: None,
                count: None,
                skip_token: None,
                tags: None,
                properties: None,
                run_id: None,
                order_by: None,
            }
        }
        #[doc = "Register a model."]
        #[doc = "Register the model provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `model`: The payload that is used to register the model."]
        pub fn register(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            model: impl Into<models::Model>,
        ) -> register::RequestBuilder {
            register::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                model: model.into(),
            }
        }
        #[doc = "Retrieve the metrics for a Model."]
        #[doc = "The operational events collected for the Model are returned."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Model Id."]
        pub fn get_metrics(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> get_metrics::RequestBuilder {
            get_metrics::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                start_date: None,
                end_date: None,
            }
        }
    }
    pub mod query_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Model> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Model = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Model>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Model>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Model> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Model = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) patch: Vec<models::JsonPatchOperation>,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Model>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Model>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedModelList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedModelList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) name: Option<String>,
            pub(crate) framework: Option<String>,
            pub(crate) description: Option<String>,
            pub(crate) count: Option<i32>,
            pub(crate) skip_token: Option<String>,
            pub(crate) tags: Option<String>,
            pub(crate) properties: Option<String>,
            pub(crate) run_id: Option<String>,
            pub(crate) order_by: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The object name."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "The framework."]
            pub fn framework(mut self, framework: impl Into<String>) -> Self {
                self.framework = Some(framework.into());
                self
            }
            #[doc = "The object description."]
            pub fn description(mut self, description: impl Into<String>) -> Self {
                self.description = Some(description.into());
                self
            }
            #[doc = "The number of items to retrieve in a page."]
            pub fn count(mut self, count: i32) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "The continuation token to retrieve the next page."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "A set of tags with which to filter the returned models.\r\n            It is a comma separated string of tags key or tags key=value\r\n            Example: tagKey1,tagKey2,tagKey3=value3"]
            pub fn tags(mut self, tags: impl Into<String>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "A set of properties with which to filter the returned models.\r\n            It is a comma separated string of properties key and/or properties key=value\r\n            Example: propKey1,propKey2,propKey3=value3"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "The runId which created the model."]
            pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
                self.run_id = Some(run_id.into());
                self
            }
            #[doc = "An option to specify how the models are ordered in the response."]
            pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
                self.order_by = Some(order_by.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedModelList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(name) = &this.name {
                                    req.url_mut().query_pairs_mut().append_pair("name", name);
                                }
                                if let Some(framework) = &this.framework {
                                    req.url_mut().query_pairs_mut().append_pair("framework", framework);
                                }
                                if let Some(description) = &this.description {
                                    req.url_mut().query_pairs_mut().append_pair("description", description);
                                }
                                if let Some(count) = &this.count {
                                    req.url_mut().query_pairs_mut().append_pair("count", &count.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                if let Some(tags) = &this.tags {
                                    req.url_mut().query_pairs_mut().append_pair("tags", tags);
                                }
                                if let Some(properties) = &this.properties {
                                    req.url_mut().query_pairs_mut().append_pair("properties", properties);
                                }
                                if let Some(run_id) = &this.run_id {
                                    req.url_mut().query_pairs_mut().append_pair("runId", run_id);
                                }
                                if let Some(order_by) = &this.order_by {
                                    req.url_mut().query_pairs_mut().append_pair("orderBy", order_by);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
    }
    pub mod register {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Model> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Model = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) model: models::Model,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.model)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Model>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Model>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_metrics {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ModelOperationalState> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ModelOperationalState = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) start_date: Option<String>,
            pub(crate) end_date: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The start date from which to retrieve metrics, ISO 8601 literal format."]
            pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
                self.start_date = Some(start_date.into());
                self
            }
            #[doc = "The end date from which to retrieve metrics, ISO 8601 literal format."]
            pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
                self.end_date = Some(end_date.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(start_date) = &this.start_date {
                            req.url_mut().query_pairs_mut().append_pair("startDate", start_date);
                        }
                        if let Some(end_date) = &this.end_date {
                            req.url_mut().query_pairs_mut().append_pair("endDate", end_date);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/models/{}/metrics" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ModelOperationalState>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ModelOperationalState>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the status of an async operation."]
        #[doc = "Get the status of an async operation by operation id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The operation id."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AsyncOperationStatus> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AsyncOperationStatus = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/operations/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AsyncOperationStatus>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AsyncOperationStatus>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod profiles {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a Profile."]
        #[doc = "Get the Profile for an Image."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `image_id`: The Image Id."]
        #[doc = "* `id`: The Profile Id."]
        pub fn query_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            image_id: impl Into<String>,
            id: impl Into<String>,
        ) -> query_by_id::RequestBuilder {
            query_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                image_id: image_id.into(),
                id: id.into(),
            }
        }
        #[doc = "Get a list of Image Profiles."]
        #[doc = "If no filter is passed, the query lists all Profiles for the Image. The returned list is paginated and the count of items in each page is an optional parameter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `image_id`: The Image Id."]
        pub fn list_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            image_id: impl Into<String>,
        ) -> list_query::RequestBuilder {
            list_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                image_id: image_id.into(),
                name: None,
                description: None,
                tags: None,
                properties: None,
                count: None,
                skip_token: None,
                order_by: None,
            }
        }
        #[doc = "Create a Profile."]
        #[doc = "Create a Profile for an Image."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `image_id`: The Image Id."]
        #[doc = "* `input_request`: The payload that is used to create the Profile."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            image_id: impl Into<String>,
            input_request: impl Into<models::ProfileRequestBase>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                image_id: image_id.into(),
                input_request: input_request.into(),
            }
        }
    }
    pub mod query_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProfileResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProfileResponse = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) image_id: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/images/{}/profiles/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . image_id , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ProfileResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ProfileResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedProfileResponseList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedProfileResponseList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) image_id: String,
            pub(crate) name: Option<String>,
            pub(crate) description: Option<String>,
            pub(crate) tags: Option<String>,
            pub(crate) properties: Option<String>,
            pub(crate) count: Option<i32>,
            pub(crate) skip_token: Option<String>,
            pub(crate) order_by: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Profile name."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "The Profile description."]
            pub fn description(mut self, description: impl Into<String>) -> Self {
                self.description = Some(description.into());
                self
            }
            #[doc = "A set of tags with which to filter the returned models.\r\n            It is a comma separated string of tags key or tags key=value\r\n            Example: tagKey1,tagKey2,tagKey3=value3"]
            pub fn tags(mut self, tags: impl Into<String>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "A set of properties with which to filter the returned models.\r\n            It is a comma separated string of properties key and/or properties key=value\r\n            Example: propKey1,propKey2,propKey3=value3"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "The number of items to retrieve in a page."]
            pub fn count(mut self, count: i32) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "The continuation token to retrieve the next page."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "The option to order the response."]
            pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
                self.order_by = Some(order_by.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedProfileResponseList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(name) = &this.name {
                                    req.url_mut().query_pairs_mut().append_pair("name", name);
                                }
                                if let Some(description) = &this.description {
                                    req.url_mut().query_pairs_mut().append_pair("description", description);
                                }
                                if let Some(tags) = &this.tags {
                                    req.url_mut().query_pairs_mut().append_pair("tags", tags);
                                }
                                if let Some(properties) = &this.properties {
                                    req.url_mut().query_pairs_mut().append_pair("properties", properties);
                                }
                                if let Some(count) = &this.count {
                                    req.url_mut().query_pairs_mut().append_pair("count", &count.to_string());
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                if let Some(order_by) = &this.order_by {
                                    req.url_mut().query_pairs_mut().append_pair("orderBy", order_by);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/images/{}/profiles" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . image_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) image_id: String,
            pub(crate) input_request: models::ProfileRequestBase,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input_request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/images/{}/profiles" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . image_id)) ? ;
                Ok(url)
            }
        }
    }
}
pub mod services {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a Service."]
        #[doc = "Get a Service by Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        pub fn query_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> query_by_id::RequestBuilder {
            query_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                expand: None,
            }
        }
        #[doc = "Patch a Service."]
        #[doc = "Patch a specific Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        #[doc = "* `patch`: The payload that is used to patch the Service."]
        pub fn patch(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
            patch: Vec<models::JsonPatchOperation>,
        ) -> patch::RequestBuilder {
            patch::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                patch,
            }
        }
        #[doc = "Delete a Service."]
        #[doc = "Delete a specific Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
        #[doc = "Query the list of Services in a Workspace."]
        #[doc = "If no filter is passed, the query lists all Services in the Workspace. The returned list is paginated and the count of item in each page is an optional parameter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        pub fn list_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
        ) -> list_query::RequestBuilder {
            list_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                image_id: None,
                image_name: None,
                model_id: None,
                model_name: None,
                name: None,
                count: None,
                compute_type: None,
                skip_token: None,
                tags: None,
                properties: None,
                expand: None,
                orderby: None,
            }
        }
        #[doc = "Create a Service."]
        #[doc = "Create a Service with the specified payload."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `request`: The payload that is used to create the Service."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            request: impl Into<models::CreateServiceRequestUnion>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                request: request.into(),
            }
        }
        #[doc = "Lists Service keys."]
        #[doc = "Gets a list of Service keys."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        pub fn list_service_keys(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> list_service_keys::RequestBuilder {
            list_service_keys::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
        #[doc = "Regenerate Service Keys."]
        #[doc = "Regenerate and return the Service keys."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        #[doc = "* `request`: The payload that is used to regenerate keys."]
        pub fn regenerate_service_keys(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
            request: impl Into<models::RegenerateServiceKeysRequest>,
        ) -> regenerate_service_keys::RequestBuilder {
            regenerate_service_keys::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
                request: request.into(),
            }
        }
        #[doc = "Generate Service Access Token."]
        #[doc = "Gets access token that can be used for calling service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace`: The name of the workspace."]
        #[doc = "* `id`: The Service Id."]
        pub fn get_service_token(
            &self,
            subscription_id: impl Into<String>,
            resource_group: impl Into<String>,
            workspace: impl Into<String>,
            id: impl Into<String>,
        ) -> get_service_token::RequestBuilder {
            get_service_token::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group: resource_group.into(),
                workspace: workspace.into(),
                id: id.into(),
            }
        }
    }
    pub mod query_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ServiceResponseBaseUnion> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ServiceResponseBaseUnion = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) expand: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Set to True to include Model details."]
            pub fn expand(mut self, expand: bool) -> Self {
                self.expand = Some(expand);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("expand", &expand.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ServiceResponseBaseUnion>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ServiceResponseBaseUnion>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) patch: Vec<models::JsonPatchOperation>,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedServiceList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedServiceList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) image_id: Option<String>,
            pub(crate) image_name: Option<String>,
            pub(crate) model_id: Option<String>,
            pub(crate) model_name: Option<String>,
            pub(crate) name: Option<String>,
            pub(crate) count: Option<i32>,
            pub(crate) compute_type: Option<String>,
            pub(crate) skip_token: Option<String>,
            pub(crate) tags: Option<String>,
            pub(crate) properties: Option<String>,
            pub(crate) expand: Option<bool>,
            pub(crate) orderby: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Image Id."]
            pub fn image_id(mut self, image_id: impl Into<String>) -> Self {
                self.image_id = Some(image_id.into());
                self
            }
            #[doc = "The Image name."]
            pub fn image_name(mut self, image_name: impl Into<String>) -> Self {
                self.image_name = Some(image_name.into());
                self
            }
            #[doc = "The Model Id."]
            pub fn model_id(mut self, model_id: impl Into<String>) -> Self {
                self.model_id = Some(model_id.into());
                self
            }
            #[doc = "The Model name."]
            pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
                self.model_name = Some(model_name.into());
                self
            }
            #[doc = "The object name."]
            pub fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            #[doc = "The number of items to retrieve in a page."]
            pub fn count(mut self, count: i32) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "The compute environment type."]
            pub fn compute_type(mut self, compute_type: impl Into<String>) -> Self {
                self.compute_type = Some(compute_type.into());
                self
            }
            #[doc = "The continuation token to retrieve the next page."]
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            #[doc = "A set of tags with which to filter the returned models.\r\n            It is a comma separated string of tags key or tags key=value\r\n            Example: tagKey1,tagKey2,tagKey3=value3"]
            pub fn tags(mut self, tags: impl Into<String>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "A set of properties with which to filter the returned models.\r\n            It is a comma separated string of properties key and/or properties key=value\r\n            Example: propKey1,propKey2,propKey3=value3"]
            pub fn properties(mut self, properties: impl Into<String>) -> Self {
                self.properties = Some(properties.into());
                self
            }
            #[doc = "Set to True to include Model details."]
            pub fn expand(mut self, expand: bool) -> Self {
                self.expand = Some(expand);
                self
            }
            #[doc = "The option to order the response."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedServiceList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(image_id) = &this.image_id {
                                    req.url_mut().query_pairs_mut().append_pair("imageId", image_id);
                                }
                                if let Some(image_name) = &this.image_name {
                                    req.url_mut().query_pairs_mut().append_pair("imageName", image_name);
                                }
                                if let Some(model_id) = &this.model_id {
                                    req.url_mut().query_pairs_mut().append_pair("modelId", model_id);
                                }
                                if let Some(model_name) = &this.model_name {
                                    req.url_mut().query_pairs_mut().append_pair("modelName", model_name);
                                }
                                if let Some(name) = &this.name {
                                    req.url_mut().query_pairs_mut().append_pair("name", name);
                                }
                                if let Some(count) = &this.count {
                                    req.url_mut().query_pairs_mut().append_pair("count", &count.to_string());
                                }
                                if let Some(compute_type) = &this.compute_type {
                                    req.url_mut().query_pairs_mut().append_pair("computeType", compute_type);
                                }
                                if let Some(skip_token) = &this.skip_token {
                                    req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                if let Some(tags) = &this.tags {
                                    req.url_mut().query_pairs_mut().append_pair("tags", tags);
                                }
                                if let Some(properties) = &this.properties {
                                    req.url_mut().query_pairs_mut().append_pair("properties", properties);
                                }
                                if let Some(expand) = &this.expand {
                                    req.url_mut().query_pairs_mut().append_pair("expand", &expand.to_string());
                                }
                                if let Some(orderby) = &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) request: models::CreateServiceRequestUnion,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_service_keys {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AuthKeys> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AuthKeys = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}/listkeys" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AuthKeys>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AuthKeys>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod regenerate_service_keys {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AuthKeys> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AuthKeys = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
            pub(crate) request: models::RegenerateServiceKeysRequest,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}/regenerateKeys" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AuthKeys>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AuthKeys>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_service_token {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AuthToken> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::AuthToken = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group: String,
            pub(crate) workspace: String,
            pub(crate) id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/modelmanagement/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/services/{}/token" , self . client . endpoint () , & self . subscription_id , & self . resource_group , & self . workspace , & self . id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::AuthToken>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::AuthToken>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod events {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Batch post event data."]
        #[doc = "Post event data to a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn batch_post(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> batch_post::RequestBuilder {
            batch_post::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                batch_event_command: None,
            }
        }
        #[doc = "Post event data."]
        #[doc = "Post event data to a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn post(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> post::RequestBuilder {
            post::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                event_message: None,
            }
        }
    }
    pub mod batch_post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchEventCommandResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchEventCommandResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) batch_event_command: Option<models::BatchEventCommand>,
        }
        impl RequestBuilder {
            #[doc = "The batch of Event details."]
            pub fn batch_event_command(mut self, batch_event_command: impl Into<models::BatchEventCommand>) -> Self {
                self.batch_event_command = Some(batch_event_command.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(batch_event_command) = &this.batch_event_command {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(batch_event_command)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/batch/events" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchEventCommandResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchEventCommandResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) event_message: Option<models::BaseEvent>,
        }
        impl RequestBuilder {
            #[doc = "The Event details."]
            pub fn event_message(mut self, event_message: impl Into<models::BaseEvent>) -> Self {
                self.event_message = Some(event_message.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(event_message) = &this.event_message {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(event_message)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/events" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
}
pub mod experiments {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get details of an Experiment."]
        #[doc = "Get details of an Experiment with specific Experiment name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
            }
        }
        #[doc = "Create an Experiment."]
        #[doc = "Create a new Experiment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
            }
        }
        #[doc = "Get details of an Experiment."]
        #[doc = "Get details of an Experiment with specific Experiment Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_id`: The identifier of the experiment."]
        pub fn get_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_id: impl Into<String>,
        ) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_id: experiment_id.into(),
            }
        }
        #[doc = "Update details of an Experiment."]
        #[doc = "Update details of an Experiment with specific Experiment Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_id`: The identifier of the experiment."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_id: experiment_id.into(),
                modify_experiment_dto: None,
            }
        }
        #[doc = "Get all Experiments in a specific workspace."]
        #[doc = "Get all experiments in a specific workspace with the specified query filters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        pub fn get_by_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> get_by_query::RequestBuilder {
            get_by_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                query_params: None,
            }
        }
        #[doc = "Delete list of Tags in an Experiment."]
        #[doc = "Delete list of Tags from a specific Experiment Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_id`: The identifier of the experiment."]
        pub fn delete_tags(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_id: impl Into<String>,
        ) -> delete_tags::RequestBuilder {
            delete_tags::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_id: experiment_id.into(),
                tags: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Experiment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Experiment = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Experiment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Experiment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Experiment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Experiment = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Experiment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Experiment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Experiment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Experiment = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experimentids/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Experiment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Experiment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Experiment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Experiment = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_id: String,
            pub(crate) modify_experiment_dto: Option<models::ModifyExperiment>,
        }
        impl RequestBuilder {
            #[doc = "Experiment details which needs to be updated."]
            pub fn modify_experiment_dto(mut self, modify_experiment_dto: impl Into<models::ModifyExperiment>) -> Self {
                self.modify_experiment_dto = Some(modify_experiment_dto.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(modify_experiment_dto) = &this.modify_experiment_dto {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(modify_experiment_dto)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experimentids/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Experiment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Experiment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_by_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedExperimentList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedExperimentList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) query_params: Option<models::QueryParams>,
        }
        impl RequestBuilder {
            #[doc = "Query parameters for data sorting and filtering."]
            pub fn query_params(mut self, query_params: impl Into<models::QueryParams>) -> Self {
                self.query_params = Some(query_params.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedExperimentList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                let req_body = if let Some(query_params) = &this.query_params {
                                    req.insert_header("content-type", "application/json");
                                    azure_core::to_json(query_params)?
                                } else {
                                    azure_core::EMPTY_BODY
                                };
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments:query" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name)) ? ;
                Ok(url)
            }
        }
    }
    pub mod delete_tags {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Experiment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Experiment = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_id: String,
            pub(crate) tags: Option<models::DeleteTagsCommand>,
        }
        impl RequestBuilder {
            #[doc = "The requested tags list to be deleted."]
            pub fn tags(mut self, tags: impl Into<models::DeleteTagsCommand>) -> Self {
                self.tags = Some(tags.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(tags) = &this.tags {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(tags)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experimentids/{}/tags" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Experiment>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Experiment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod runs {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get details of all child runs."]
        #[doc = "Get details of all child runs for the specified Run Id with the specified filters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get_child(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_child::RequestBuilder {
            get_child::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                filter: None,
                continuationtoken: None,
                orderby: Vec::new(),
                sortorder: None,
                top: None,
                count: None,
            }
        }
        #[doc = "Get Run Details."]
        #[doc = "Get Run Details for a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get_details(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_details::RequestBuilder {
            get_details::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
            }
        }
        #[doc = "Add or Modify a batch of Runs."]
        #[doc = "Add or Modify a batch of Runs for a given experiment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn batch_add_or_modify(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> batch_add_or_modify::RequestBuilder {
            batch_add_or_modify::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                request_dto: None,
            }
        }
        #[doc = "Get Run details."]
        #[doc = "Get Run details of a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
            }
        }
        #[doc = "Add or Modify a Run."]
        #[doc = "Add a new Run or Modify an existing Run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn patch(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> patch::RequestBuilder {
            patch::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                create_run_dto: None,
            }
        }
        #[doc = "Delete list of Tags in a Run."]
        #[doc = "Delete list of Tags from a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn delete_tags(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> delete_tags::RequestBuilder {
            delete_tags::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                tags: Vec::new(),
            }
        }
        #[doc = "Get all Runs for a specific Experiment."]
        #[doc = "Get all Runs for a specific Experiment with the specified query filters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn get_by_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> get_by_query::RequestBuilder {
            get_by_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                query_params: None,
            }
        }
    }
    pub mod get_child {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedRunList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedRunList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) continuationtoken: Option<String>,
            pub(crate) orderby: Vec<String>,
            pub(crate) sortorder: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) count: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Allows for filtering the collection of resources.\r\nThe expression specified is evaluated for each resource in the collection, and only items where the expression evaluates to true are included in the response."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "The continuation token to use for getting the next set of resources."]
            pub fn continuationtoken(mut self, continuationtoken: impl Into<String>) -> Self {
                self.continuationtoken = Some(continuationtoken.into());
                self
            }
            #[doc = "The list of resource properties to use for sorting the requested resources."]
            pub fn orderby(mut self, orderby: Vec<String>) -> Self {
                self.orderby = orderby;
                self
            }
            #[doc = "The sort order of the returned resources. Not used, specify asc or desc after each property name in the OrderBy parameter."]
            pub fn sortorder(mut self, sortorder: impl Into<String>) -> Self {
                self.sortorder = Some(sortorder.into());
                self
            }
            #[doc = "The maximum number of items in the resource collection to be included in the result.\r\nIf not specified, all items are returned."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Whether to include a count of the matching resources along with the resources returned in the response."]
            pub fn count(mut self, count: bool) -> Self {
                self.count = Some(count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedRunList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(continuationtoken) = &this.continuationtoken {
                                    req.url_mut().query_pairs_mut().append_pair("$continuationtoken", continuationtoken);
                                }
                                let orderby = &this.orderby;
                                for value in &this.orderby {
                                    req.url_mut().query_pairs_mut().append_pair("$orderby", &value.to_string());
                                }
                                if let Some(sortorder) = &this.sortorder {
                                    req.url_mut().query_pairs_mut().append_pair("$sortorder", sortorder);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(count) = &this.count {
                                    req.url_mut().query_pairs_mut().append_pair("$count", &count.to_string());
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/children" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod get_details {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RunDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RunDetails = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/details" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::RunDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::RunDetails>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod batch_add_or_modify {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchAddOrModifyRunResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchAddOrModifyRunResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) request_dto: Option<models::BatchAddOrModifyRunRequest>,
        }
        impl RequestBuilder {
            #[doc = "The list of requested Run Additions/modifications in an Experiment."]
            pub fn request_dto(mut self, request_dto: impl Into<models::BatchAddOrModifyRunRequest>) -> Self {
                self.request_dto = Some(request_dto.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(request_dto) = &this.request_dto {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(request_dto)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/batch/runs" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchAddOrModifyRunResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchAddOrModifyRunResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Run> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Run = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Run>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Run>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod patch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Run> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Run = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) create_run_dto: Option<models::CreateRun>,
        }
        impl RequestBuilder {
            #[doc = "The requested Run parameter Additions/modifications."]
            pub fn create_run_dto(mut self, create_run_dto: impl Into<models::CreateRun>) -> Self {
                self.create_run_dto = Some(create_run_dto.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(create_run_dto) = &this.create_run_dto {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(create_run_dto)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Run>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Run>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_tags {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Run> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Run = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) tags: Vec<String>,
        }
        impl RequestBuilder {
            #[doc = "The requested tags list to be deleted."]
            pub fn tags(mut self, tags: Vec<String>) -> Self {
                self.tags = tags;
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.tags)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/tags" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Run>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Run>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_by_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedRunList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedRunList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) query_params: Option<models::QueryParams>,
        }
        impl RequestBuilder {
            #[doc = "Query parameters for data sorting and filtering."]
            pub fn query_params(mut self, query_params: impl Into<models::QueryParams>) -> Self {
                self.query_params = Some(query_params.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedRunList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                let req_body = if let Some(query_params) = &this.query_params {
                                    req.insert_header("content-type", "application/json");
                                    azure_core::to_json(query_params)?
                                } else {
                                    azure_core::EMPTY_BODY
                                };
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs:query" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
    }
}
pub mod run_artifacts {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get Artifacts in a container."]
        #[doc = "Get Artifacts in container for a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn list_in_container(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> list_in_container::RequestBuilder {
            list_in_container::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                continuation_token: None,
            }
        }
        #[doc = "Get Artifacts in the provided path."]
        #[doc = "Get Artifacts in the provided path for a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn list_in_path(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> list_in_path::RequestBuilder {
            list_in_path::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                path: None,
                continuation_token: None,
            }
        }
        #[doc = "Get Artifact by Id."]
        #[doc = "Get Artifact for a specific Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                path: None,
            }
        }
        #[doc = "Get Artifact content information."]
        #[doc = "Get Artifact content information for give Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get_content_information(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_content_information::RequestBuilder {
            get_content_information::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                path: None,
            }
        }
        #[doc = "Get URI of an Artifact."]
        #[doc = "Get URI of an Artifact for a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn get_sas_uri(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_sas_uri::RequestBuilder {
            get_sas_uri::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                path: None,
            }
        }
        #[doc = "Get SAS of an Artifact."]
        #[doc = "Get SAS of an Artifact in the specified path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn list_sas_by_prefix(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> list_sas_by_prefix::RequestBuilder {
            list_sas_by_prefix::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                path: None,
                continuation_token: None,
            }
        }
        #[doc = "Create a batch of empty Artifacts."]
        #[doc = "Create a batch of empty Artifacts in a specific Run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier of the Run."]
        pub fn batch_create_empty_artifacts(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> batch_create_empty_artifacts::RequestBuilder {
            batch_create_empty_artifacts::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                artifact_paths: None,
            }
        }
    }
    pub mod list_in_container {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Continuation Token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_in_path {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) path: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The Continuation Token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(path) = &this.path {
                                    req.url_mut().query_pairs_mut().append_pair("path", path);
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/path" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Artifact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Artifact = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Artifact>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Artifact>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_content_information {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ArtifactContentInformation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ArtifactContentInformation = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/contentinfo" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ArtifactContentInformation>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ArtifactContentInformation>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_sas_uri {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<String> {
                let bytes = self.0.into_body().collect().await?;
                let body: String = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/artifacturi" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<String>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<String>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_sas_by_prefix {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactContentInformationList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactContentInformationList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) path: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The Continuation Token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactContentInformationList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(path) = &this.path {
                                    req.url_mut().query_pairs_mut().append_pair("path", path);
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/prefix/contentinfo" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod batch_create_empty_artifacts {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchArtifactContentInformationResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchArtifactContentInformationResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) artifact_paths: Option<models::ArtifactPathList>,
        }
        impl RequestBuilder {
            #[doc = "The list of artifact paths."]
            pub fn artifact_paths(mut self, artifact_paths: impl Into<models::ArtifactPathList>) -> Self {
                self.artifact_paths = Some(artifact_paths.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(artifact_paths) = &this.artifact_paths {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(artifact_paths)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/artifacts/batch/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchArtifactContentInformationResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchArtifactContentInformationResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod run_metrics {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Post Metric to a Run."]
        #[doc = "Post a Metric to a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier for a run."]
        pub fn post(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> post::RequestBuilder {
            post::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                metric_dto: None,
            }
        }
        #[doc = "Post Metrics to a Run."]
        #[doc = "Post Metrics to a specific Run Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `run_id`: The identifier for a run."]
        pub fn batch_post(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> batch_post::RequestBuilder {
            batch_post::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                run_id: run_id.into(),
                batch_metric_dto: None,
            }
        }
        #[doc = "Get Metric details."]
        #[doc = "Get Metric details for a specific Metric Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        #[doc = "* `metric_id`: The identifier for a Metric."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
            metric_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                metric_id: metric_id.into(),
            }
        }
        #[doc = "Get all Run Metrics for the specific Experiment."]
        #[doc = "Get all Run Metrics for the specific Experiment with the specified query filters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `experiment_name`: The experiment name."]
        pub fn get_by_query(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            experiment_name: impl Into<String>,
        ) -> get_by_query::RequestBuilder {
            get_by_query::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                experiment_name: experiment_name.into(),
                query_params: None,
                merge_strategy_type: None,
                merge_strategy_options: None,
                merge_strategy_settings_version: None,
                merge_strategy_settings_select_metrics: None,
            }
        }
    }
    pub mod post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) metric_dto: Option<models::Metric>,
        }
        impl RequestBuilder {
            #[doc = "Details of the metric which will be added to the Run Id."]
            pub fn metric_dto(mut self, metric_dto: impl Into<models::Metric>) -> Self {
                self.metric_dto = Some(metric_dto.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(metric_dto) = &this.metric_dto {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(metric_dto)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/metrics" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod batch_post {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) run_id: String,
            pub(crate) batch_metric_dto: Option<models::BatchMetric>,
        }
        impl RequestBuilder {
            #[doc = "Details of the Metrics which will be added to the Run Id."]
            pub fn batch_metric_dto(mut self, batch_metric_dto: impl Into<models::BatchMetric>) -> Self {
                self.batch_metric_dto = Some(batch_metric_dto.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(batch_metric_dto) = &this.batch_metric_dto {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(batch_metric_dto)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/runs/{}/batch/metrics" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . run_id)) ? ;
                Ok(url)
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::RunMetric> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::RunMetric = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) metric_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/metrics/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name , & self . metric_id)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::RunMetric>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::RunMetric>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_by_query {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedRunMetricList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedRunMetricList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) experiment_name: String,
            pub(crate) query_params: Option<models::QueryParams>,
            pub(crate) merge_strategy_type: Option<String>,
            pub(crate) merge_strategy_options: Option<String>,
            pub(crate) merge_strategy_settings_version: Option<String>,
            pub(crate) merge_strategy_settings_select_metrics: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Query Parameters for data sorting and filtering."]
            pub fn query_params(mut self, query_params: impl Into<models::QueryParams>) -> Self {
                self.query_params = Some(query_params.into());
                self
            }
            #[doc = "The type of merge strategy. Currently supported strategies are:\r\nNone - all logged values are returned as individual metrics.\r\nMergeToVector - merges multiple values into a vector of values.\r\nDefault - the system determines the behavior."]
            pub fn merge_strategy_type(mut self, merge_strategy_type: impl Into<String>) -> Self {
                self.merge_strategy_type = Some(merge_strategy_type.into());
                self
            }
            #[doc = "Controls behavior of the merge strategy in certain cases; e.g. when a metric is not merged."]
            pub fn merge_strategy_options(mut self, merge_strategy_options: impl Into<String>) -> Self {
                self.merge_strategy_options = Some(merge_strategy_options.into());
                self
            }
            #[doc = "The strategy settings version."]
            pub fn merge_strategy_settings_version(mut self, merge_strategy_settings_version: impl Into<String>) -> Self {
                self.merge_strategy_settings_version = Some(merge_strategy_settings_version.into());
                self
            }
            #[doc = "Defines how to select metrics when merging them together."]
            pub fn merge_strategy_settings_select_metrics(mut self, merge_strategy_settings_select_metrics: impl Into<String>) -> Self {
                self.merge_strategy_settings_select_metrics = Some(merge_strategy_settings_select_metrics.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedRunMetricList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                let req_body = if let Some(query_params) = &this.query_params {
                                    req.insert_header("content-type", "application/json");
                                    azure_core::to_json(query_params)?
                                } else {
                                    azure_core::EMPTY_BODY
                                };
                                if let Some(merge_strategy_type) = &this.merge_strategy_type {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("MergeStrategyType", merge_strategy_type);
                                }
                                if let Some(merge_strategy_options) = &this.merge_strategy_options {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("MergeStrategyOptions", merge_strategy_options);
                                }
                                if let Some(merge_strategy_settings_version) = &this.merge_strategy_settings_version {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("MergeStrategySettings.Version", merge_strategy_settings_version);
                                }
                                if let Some(merge_strategy_settings_select_metrics) = &this.merge_strategy_settings_select_metrics {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("MergeStrategySettings.SelectMetrics", merge_strategy_settings_select_metrics);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/history/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/experiments/{}/metrics:query" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . experiment_name)) ? ;
                Ok(url)
            }
        }
    }
}
impl Client {
    #[doc = "Get Datastores list."]
    #[doc = "Get the list of Datastores attached to the workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    pub fn list(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
    ) -> list::RequestBuilder {
        list::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            data_store_names: Vec::new(),
            continuation_token: None,
            count: None,
            include_secret: None,
        }
    }
    #[doc = "Create or update a Datastore."]
    #[doc = "Create or update a Datastore in the given workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    pub fn create(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
    ) -> create::RequestBuilder {
        create::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            dto: None,
            create_if_not_exists: None,
            skip_validation: None,
        }
    }
    #[doc = "Delete all Datastores."]
    #[doc = "Delete all Datastores in the workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    pub fn delete_all(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
    ) -> delete_all::RequestBuilder {
        delete_all::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
        }
    }
    #[doc = "Get Datastore details."]
    #[doc = "Get details of a Datastore with a specific name."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    #[doc = "* `name`: The Datastore name."]
    pub fn get(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
        name: impl Into<String>,
    ) -> get::RequestBuilder {
        get::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            name: name.into(),
        }
    }
    #[doc = "Update or create a Datastore."]
    #[doc = "Update or create a Datastore in the given workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    #[doc = "* `name`: The Datastore name."]
    pub fn update(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
        name: impl Into<String>,
    ) -> update::RequestBuilder {
        update::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            name: name.into(),
            dto: None,
            create_if_not_exists: None,
            skip_validation: None,
        }
    }
    #[doc = "Delete a Datastore."]
    #[doc = "Delete a Datastore with a specific name."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    #[doc = "* `name`: The Datastore name."]
    pub fn delete(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
        name: impl Into<String>,
    ) -> delete::RequestBuilder {
        delete::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            name: name.into(),
        }
    }
    #[doc = "Set a default Datastore."]
    #[doc = "Set a default Datastore in the workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    #[doc = "* `name`: The Datastore name."]
    pub fn set_default(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
        name: impl Into<String>,
    ) -> set_default::RequestBuilder {
        set_default::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
            name: name.into(),
        }
    }
    #[doc = "Get the default Datastore."]
    #[doc = "Get the default Datastore in the workspace."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The Azure Subscription ID."]
    #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
    #[doc = "* `workspace_name`: The name of the workspace."]
    pub fn get_default(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        workspace_name: impl Into<String>,
    ) -> get_default::RequestBuilder {
        get_default::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            workspace_name: workspace_name.into(),
        }
    }
}
pub mod list {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::PaginatedDataStoreList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::PaginatedDataStoreList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) data_store_names: Vec<String>,
        pub(crate) continuation_token: Option<String>,
        pub(crate) count: Option<i32>,
        pub(crate) include_secret: Option<bool>,
    }
    impl RequestBuilder {
        #[doc = "List of Datastore names."]
        pub fn data_store_names(mut self, data_store_names: Vec<String>) -> Self {
            self.data_store_names = data_store_names;
            self
        }
        #[doc = "The Continuation Token."]
        pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
            self.continuation_token = Some(continuation_token.into());
            self
        }
        #[doc = "Count of Datastores to be returned."]
        pub fn count(mut self, count: i32) -> Self {
            self.count = Some(count);
            self
        }
        #[doc = "Whether to include the datastore secret in the response."]
        pub fn include_secret(mut self, include_secret: bool) -> Self {
            self.include_secret = Some(include_secret);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedDataStoreList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = this.url()?;
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
                            let data_store_names = &this.data_store_names;
                            for value in &this.data_store_names {
                                req.url_mut().query_pairs_mut().append_pair("dataStoreNames", &value.to_string());
                            }
                            if let Some(continuation_token) = &this.continuation_token {
                                req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                            }
                            if let Some(count) = &this.count {
                                req.url_mut().query_pairs_mut().append_pair("count", &count.to_string());
                            }
                            if let Some(include_secret) = &this.include_secret {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair("includeSecret", &include_secret.to_string());
                            }
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core::Url::parse(&format!(
                "{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores",
                self.client.endpoint(),
                &self.subscription_id,
                &self.resource_group_name,
                &self.workspace_name
            ))?;
            Ok(url)
        }
    }
}
pub mod create {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) dto: Option<models::DataStore>,
        pub(crate) create_if_not_exists: Option<bool>,
        pub(crate) skip_validation: Option<bool>,
    }
    impl RequestBuilder {
        #[doc = "The Datastore details."]
        pub fn dto(mut self, dto: impl Into<models::DataStore>) -> Self {
            self.dto = Some(dto.into());
            self
        }
        #[doc = "If set to true, the call will create an Datastore if it doesn't exist."]
        pub fn create_if_not_exists(mut self, create_if_not_exists: bool) -> Self {
            self.create_if_not_exists = Some(create_if_not_exists);
            self
        }
        #[doc = "If set to true, the call will skip Datastore validation."]
        pub fn skip_validation(mut self, skip_validation: bool) -> Self {
            self.skip_validation = Some(skip_validation);
            self
        }
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = if let Some(dto) = &this.dto {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(dto)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    if let Some(create_if_not_exists) = &this.create_if_not_exists {
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("createIfNotExists", &create_if_not_exists.to_string());
                    }
                    if let Some(skip_validation) = &this.skip_validation {
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("skipValidation", &skip_validation.to_string());
                    }
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core::Url::parse(&format!(
                "{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores",
                self.client.endpoint(),
                &self.subscription_id,
                &self.resource_group_name,
                &self.workspace_name
            ))?;
            Ok(url)
        }
    }
}
pub mod delete_all {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
    }
    impl RequestBuilder {
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core::Url::parse(&format!(
                "{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores",
                self.client.endpoint(),
                &self.subscription_id,
                &self.resource_group_name,
                &self.workspace_name
            ))?;
            Ok(url)
        }
    }
}
pub mod get {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::DataStore> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::DataStore = serde_json::from_slice(&bytes)?;
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) name: String,
    }
    impl RequestBuilder {
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core :: Url :: parse (& format ! ("{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . name)) ? ;
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::DataStore>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::DataStore>>;
        #[doc = "Returns a future that sends the request and returns the parsed response body."]
        #[doc = ""]
        #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
        #[doc = ""]
        #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
        fn into_future(self) -> Self::IntoFuture {
            Box::pin(async move { self.send().await?.into_body().await })
        }
    }
}
pub mod update {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) name: String,
        pub(crate) dto: Option<models::DataStore>,
        pub(crate) create_if_not_exists: Option<bool>,
        pub(crate) skip_validation: Option<bool>,
    }
    impl RequestBuilder {
        #[doc = "The Datastore details."]
        pub fn dto(mut self, dto: impl Into<models::DataStore>) -> Self {
            self.dto = Some(dto.into());
            self
        }
        #[doc = "If set to true, the call will create an Datastore if it doesn't exist."]
        pub fn create_if_not_exists(mut self, create_if_not_exists: bool) -> Self {
            self.create_if_not_exists = Some(create_if_not_exists);
            self
        }
        #[doc = "If set to true, the call will skip Datastore validation."]
        pub fn skip_validation(mut self, skip_validation: bool) -> Self {
            self.skip_validation = Some(skip_validation);
            self
        }
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    let req_body = if let Some(dto) = &this.dto {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(dto)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    if let Some(create_if_not_exists) = &this.create_if_not_exists {
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("createIfNotExists", &create_if_not_exists.to_string());
                    }
                    if let Some(skip_validation) = &this.skip_validation {
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("skipValidation", &skip_validation.to_string());
                    }
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core :: Url :: parse (& format ! ("{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . name)) ? ;
            Ok(url)
        }
    }
}
pub mod delete {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) name: String,
    }
    impl RequestBuilder {
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core :: Url :: parse (& format ! ("{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/datastores/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . name)) ? ;
            Ok(url)
        }
    }
}
pub mod set_default {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
        pub(crate) name: String,
    }
    impl RequestBuilder {
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core::Url::parse(&format!(
                "{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/default/{}",
                self.client.endpoint(),
                &self.subscription_id,
                &self.resource_group_name,
                &self.workspace_name,
                &self.name
            ))?;
            Ok(url)
        }
    }
}
pub mod get_default {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::DataStore> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::DataStore = serde_json::from_slice(&bytes)?;
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" To finalize and submit the request, invoke `.await`, which"]
    #[doc = r" which will convert the [`RequestBuilder`] into a future"]
    #[doc = r" executes the request and returns a `Result` with the parsed"]
    #[doc = r" response."]
    #[doc = r""]
    #[doc = r" In order to execute the request without polling the service"]
    #[doc = r" until the operation completes, use `.send().await` instead."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details"]
    #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
    #[doc = r" can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
    #[doc = r" that resolves to a lower-level [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) workspace_name: String,
    }
    impl RequestBuilder {
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = this.url()?;
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let url = azure_core::Url::parse(&format!(
                "{}/datastore/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/default",
                self.client.endpoint(),
                &self.subscription_id,
                &self.resource_group_name,
                &self.workspace_name
            ))?;
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::DataStore>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::DataStore>>;
        #[doc = "Returns a future that sends the request and returns the parsed response body."]
        #[doc = ""]
        #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
        #[doc = ""]
        #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
        fn into_future(self) -> Self::IntoFuture {
            Box::pin(async move { self.send().await?.into_body().await })
        }
    }
}
pub mod artifacts {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create Artifact."]
        #[doc = "Create an Artifact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `artifact`: The Artifact details."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            artifact: impl Into<models::Artifact>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                artifact: artifact.into(),
            }
        }
        #[doc = "Create an Artifact for an existing data location."]
        #[doc = "Create an Artifact for an existing dataPath."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `artifact`: The Artifact creation details."]
        pub fn register(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            artifact: impl Into<models::Artifact>,
        ) -> register::RequestBuilder {
            register::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                artifact: artifact.into(),
            }
        }
        #[doc = "Get Artifact metadata by Id."]
        #[doc = "Get Artifact metadata for a specific Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        #[doc = "* `path`: The Artifact Path."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
            path: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: path.into(),
            }
        }
        #[doc = "Delete Artifact Metadata."]
        #[doc = "Delete an Artifact Metadata."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn delete_meta_data(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> delete_meta_data::RequestBuilder {
            delete_meta_data::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
                hard_delete: None,
            }
        }
        #[doc = "Get Artifacts metadata in a container or path."]
        #[doc = "Get Artifacts metadata in a specific container or path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn list_in_container(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> list_in_container::RequestBuilder {
            list_in_container::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
                continuation_token: None,
            }
        }
        #[doc = "Get Artifact content by Id."]
        #[doc = "Get Artifact content of a specific Id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn download(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> download::RequestBuilder {
            download::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
            }
        }
        #[doc = "Upload Artifact content."]
        #[doc = "Upload content to an Artifact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        #[doc = "* `content`: The file upload."]
        pub fn upload(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
            content: impl Into<String>,
        ) -> upload::RequestBuilder {
            upload::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                content: content.into(),
                path: None,
                index: None,
                append: None,
                allow_overwrite: None,
            }
        }
        #[doc = "Get Artifact content information."]
        #[doc = "Get content information of an Artifact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn get_content_information(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> get_content_information::RequestBuilder {
            get_content_information::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
            }
        }
        #[doc = "Get Artifact storage content information."]
        #[doc = "Get storage content information of an Artifact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn get_storage_content_information(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> get_storage_content_information::RequestBuilder {
            get_storage_content_information::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
            }
        }
        #[doc = "Get writable shared access signature for Artifact."]
        #[doc = "Get writable shared access signature for a specific Artifact."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn get_sas(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> get_sas::RequestBuilder {
            get_sas::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
            }
        }
        #[doc = "Get shared access signature for an Artifact"]
        #[doc = "Get shared access signature for an Artifact in specific path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn list_sas_by_prefix(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> list_sas_by_prefix::RequestBuilder {
            list_sas_by_prefix::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
                continuation_token: None,
            }
        }
        #[doc = "Get storage Uri for Artifacts in a path."]
        #[doc = "Get storage Uri for Artifacts in a specific path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn list_storage_uri_by_prefix(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> list_storage_uri_by_prefix::RequestBuilder {
            list_storage_uri_by_prefix::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                path: None,
                continuation_token: None,
            }
        }
        #[doc = "Get Batch Artifacts by Ids."]
        #[doc = "Get Batch Artifacts by the specific Ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `artifact_ids`: The command for Batch Artifact get request."]
        pub fn batch_get_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            artifact_ids: impl Into<models::ArtifactIdList>,
        ) -> batch_get_by_id::RequestBuilder {
            batch_get_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                artifact_ids: artifact_ids.into(),
            }
        }
        #[doc = "Get Batch Artifacts storage by Ids."]
        #[doc = "Get Batch Artifacts storage by specific Ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `artifact_ids`: The list of artifactIds to get."]
        pub fn batch_get_storage_by_id(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            artifact_ids: impl Into<models::ArtifactIdList>,
        ) -> batch_get_storage_by_id::RequestBuilder {
            batch_get_storage_by_id::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                artifact_ids: artifact_ids.into(),
            }
        }
        #[doc = "Batch ingest using shared access signature."]
        #[doc = "Ingest Batch Artifacts using shared access signature."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        #[doc = "* `artifact_container_sas`: The artifact container shared access signature to use for batch ingest."]
        pub fn batch_ingest_from_sas(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
            artifact_container_sas: impl Into<models::ArtifactContainerSas>,
        ) -> batch_ingest_from_sas::RequestBuilder {
            batch_ingest_from_sas::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                artifact_container_sas: artifact_container_sas.into(),
            }
        }
        #[doc = "Create a batch of empty Artifacts."]
        #[doc = "Create a Batch of empty Artifacts from the supplied paths."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        #[doc = "* `artifact_paths`: The list of Artifact paths to create."]
        pub fn batch_create_empty_artifacts(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
            artifact_paths: impl Into<models::ArtifactPathList>,
        ) -> batch_create_empty_artifacts::RequestBuilder {
            batch_create_empty_artifacts::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                artifact_paths: artifact_paths.into(),
            }
        }
        #[doc = "Delete Batch of Artifact Metadata."]
        #[doc = "Delete a Batch of Artifact Metadata."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        #[doc = "* `artifact_paths`: The list of Artifact paths to delete."]
        pub fn delete_batch_meta_data(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
            artifact_paths: impl Into<models::ArtifactPathList>,
        ) -> delete_batch_meta_data::RequestBuilder {
            delete_batch_meta_data::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                artifact_paths: artifact_paths.into(),
                hard_delete: None,
            }
        }
        #[doc = "Delete Artifact Metadata."]
        #[doc = "Delete Artifact Metadata in a specific container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Azure Subscription ID."]
        #[doc = "* `resource_group_name`: The Name of the resource group in which the workspace is located."]
        #[doc = "* `workspace_name`: The name of the workspace."]
        #[doc = "* `origin`: The origin of the Artifact."]
        #[doc = "* `container`: The container name."]
        pub fn delete_meta_data_in_container(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            origin: impl Into<String>,
            container: impl Into<String>,
        ) -> delete_meta_data_in_container::RequestBuilder {
            delete_meta_data_in_container::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                origin: origin.into(),
                container: container.into(),
                hard_delete: None,
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Artifact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Artifact = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) artifact: models::Artifact,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Artifact>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Artifact>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod register {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Artifact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Artifact = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) artifact: models::Artifact,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/register" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Artifact>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Artifact>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Artifact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Artifact = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let path = &this.path;
                        req.url_mut().query_pairs_mut().append_pair("path", path);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Artifact>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Artifact>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_meta_data {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
            pub(crate) hard_delete: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "If set to true. The delete cannot be revert at later time."]
            pub fn hard_delete(mut self, hard_delete: bool) -> Self {
                self.hard_delete = Some(hard_delete);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        if let Some(hard_delete) = &this.hard_delete {
                            req.url_mut().query_pairs_mut().append_pair("hardDelete", &hard_delete.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_in_container {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(path) = &this.path {
                                    req.url_mut().query_pairs_mut().append_pair("path", path);
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod download {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/content" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<bytes::Bytes>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<bytes::Bytes>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod upload {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Artifact> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Artifact = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) content: String,
            pub(crate) path: Option<String>,
            pub(crate) index: Option<i32>,
            pub(crate) append: Option<bool>,
            pub(crate) allow_overwrite: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The index."]
            pub fn index(mut self, index: i32) -> Self {
                self.index = Some(index);
                self
            }
            #[doc = "Whether or not to append the content or replace it."]
            pub fn append(mut self, append: bool) -> Self {
                self.append = Some(append);
                self
            }
            #[doc = "whether to allow overwrite if Artifact Content exist already. when set to true, Overwrite happens if Artifact Content already exists"]
            pub fn allow_overwrite(mut self, allow_overwrite: bool) -> Self {
                self.allow_overwrite = Some(allow_overwrite);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        if let Some(index) = &this.index {
                            req.url_mut().query_pairs_mut().append_pair("index", &index.to_string());
                        }
                        if let Some(append) = &this.append {
                            req.url_mut().query_pairs_mut().append_pair("append", &append.to_string());
                        }
                        if let Some(allow_overwrite) = &this.allow_overwrite {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("allowOverwrite", &allow_overwrite.to_string());
                        }
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.content)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/content" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Artifact>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Artifact>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_content_information {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ArtifactContentInformation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ArtifactContentInformation = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/contentinfo" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ArtifactContentInformation>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ArtifactContentInformation>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_storage_content_information {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ArtifactContentInformation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ArtifactContentInformation = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/contentinfo/storageuri" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ArtifactContentInformation>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ArtifactContentInformation>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_sas {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ArtifactContentInformation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ArtifactContentInformation = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(path) = &this.path {
                            req.url_mut().query_pairs_mut().append_pair("path", path);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/write" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ArtifactContentInformation>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ArtifactContentInformation>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod list_sas_by_prefix {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactContentInformationList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactContentInformationList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactContentInformationList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(path) = &this.path {
                                    req.url_mut().query_pairs_mut().append_pair("path", path);
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/prefix/contentinfo" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod list_storage_uri_by_prefix {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactContentInformationList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactContentInformationList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) path: Option<String>,
            pub(crate) continuation_token: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The Artifact Path."]
            pub fn path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactContentInformationList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                if let Some(path) = &this.path {
                                    req.url_mut().query_pairs_mut().append_pair("path", path);
                                }
                                if let Some(continuation_token) = &this.continuation_token {
                                    req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                                }
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/prefix/contentinfo/storageuri" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod batch_get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchArtifactContentInformationResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchArtifactContentInformationResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) artifact_ids: models::ArtifactIdList,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact_ids)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/batch/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchArtifactContentInformationResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchArtifactContentInformationResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod batch_get_storage_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchArtifactContentInformationResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchArtifactContentInformationResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) artifact_ids: models::ArtifactIdList,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact_ids)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/storageuri/batch/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchArtifactContentInformationResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchArtifactContentInformationResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod batch_ingest_from_sas {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PaginatedArtifactList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PaginatedArtifactList = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) artifact_container_sas: models::ArtifactContainerSas,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PaginatedArtifactList, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
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
                                req.insert_header("content-type", "application/json");
                                let req_body = azure_core::to_json(&this.artifact_container_sas)?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/batch/ingest/containersas" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod batch_create_empty_artifacts {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::BatchArtifactContentInformationResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::BatchArtifactContentInformationResult = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) artifact_paths: models::ArtifactPathList,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact_paths)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/batch/metadata" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::BatchArtifactContentInformationResult>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::BatchArtifactContentInformationResult>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_batch_meta_data {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) artifact_paths: models::ArtifactPathList,
            pub(crate) hard_delete: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If set to true, the delete cannot be reverted at a later time."]
            pub fn hard_delete(mut self, hard_delete: bool) -> Self {
                self.hard_delete = Some(hard_delete);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.artifact_paths)?;
                        if let Some(hard_delete) = &this.hard_delete {
                            req.url_mut().query_pairs_mut().append_pair("hardDelete", &hard_delete.to_string());
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/batch/metadata:delete" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
    pub mod delete_meta_data_in_container {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) origin: String,
            pub(crate) container: String,
            pub(crate) hard_delete: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "If set to true. The delete cannot be revert at later time."]
            pub fn hard_delete(mut self, hard_delete: bool) -> Self {
                self.hard_delete = Some(hard_delete);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(hard_delete) = &this.hard_delete {
                            req.url_mut().query_pairs_mut().append_pair("hardDelete", &hard_delete.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core :: Url :: parse (& format ! ("{}/artifact/v2.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearningServices/workspaces/{}/artifacts/{}/{}/batch" , self . client . endpoint () , & self . subscription_id , & self . resource_group_name , & self . workspace_name , & self . origin , & self . container)) ? ;
                Ok(url)
            }
        }
    }
}
pub mod hyper_drive {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create an Experiment."]
        #[doc = "Create a HyperDrive Experiment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `arm_scope`: The ARM scope passed in through URL with format:\r\n            subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.MachineLearningServices/workspaces/{workspaceName}/experiments/{experimentName}.\r\n"]
        #[doc = "* `config`: The configuration file with experiment JSON content. A text file that is a JSON-serialized '#/definitions/HyperDriveCreateExperiment' object."]
        pub fn create_experiment(
            &self,
            arm_scope: impl Into<String>,
            config: impl Into<bytes::Bytes>,
        ) -> create_experiment::RequestBuilder {
            create_experiment::RequestBuilder {
                client: self.0.clone(),
                arm_scope: arm_scope.into(),
                config: config.into(),
            }
        }
        #[doc = "Cancel an Experiment."]
        #[doc = "Cancel a HyperDrive Experiment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `arm_scope`: The ARM scope passed in through URL with format:\r\n            subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.MachineLearningServices/workspaces/{workspaceName}/experiments/{experimentName}.\r\n"]
        #[doc = "* `run_id`: Hyperdrive run id to cancel."]
        pub fn cancel_experiment(&self, arm_scope: impl Into<String>, run_id: impl Into<String>) -> cancel_experiment::RequestBuilder {
            cancel_experiment::RequestBuilder {
                client: self.0.clone(),
                arm_scope: arm_scope.into(),
                run_id: run_id.into(),
                run_history_host: None,
            }
        }
    }
    pub mod create_experiment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperDriveExperimentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperDriveExperimentResponse = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) arm_scope: String,
            pub(crate) config: bytes::Bytes,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/hyperdrive/v1.0/{}/runs", self.client.endpoint(), &self.arm_scope))?;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::HyperDriveExperimentResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::HyperDriveExperimentResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod cancel_experiment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HyperDriveCancelExperimentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::HyperDriveCancelExperimentResponse = serde_json::from_slice(&bytes)?;
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
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) arm_scope: String,
            pub(crate) run_id: String,
            pub(crate) run_history_host: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The host for run location."]
            pub fn run_history_host(mut self, run_history_host: impl Into<String>) -> Self {
                self.run_history_host = Some(run_history_host.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(run_history_host) = &this.run_history_host {
                            req.insert_header("runhistoryhost", run_history_host);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/hyperdrive/v1.0/{}/runs/{}/cancel",
                    self.client.endpoint(),
                    &self.arm_scope,
                    &self.run_id
                ))?;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::HyperDriveCancelExperimentResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::HyperDriveCancelExperimentResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
