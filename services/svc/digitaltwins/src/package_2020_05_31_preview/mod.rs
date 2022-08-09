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
pub const DEFAULT_ENDPOINT: &str = "https://digitaltwins-name.digitaltwins.azure.net";
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
    pub fn digital_twin_models_client(&self) -> digital_twin_models::Client {
        digital_twin_models::Client(self.clone())
    }
    pub fn digital_twins_client(&self) -> digital_twins::Client {
        digital_twins::Client(self.clone())
    }
    pub fn event_routes_client(&self) -> event_routes::Client {
        event_routes::Client(self.clone())
    }
    pub fn query_client(&self) -> query::Client {
        query::Client(self.clone())
    }
}
pub mod digital_twin_models {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves model metadata and, optionally, model definitions.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                dependencies_for: Vec::new(),
                include_model_definition: None,
                x_ms_max_item_count: None,
            }
        }
        #[doc = "Uploads one or more models. When any error occurs, no models are uploaded.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n409 (Conflict): One or more of the provided models already exist."]
        pub fn add(&self) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                models: Vec::new(),
            }
        }
        #[doc = "Retrieves model metadata and optionally the model definition.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is no model with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::Builder {
            get_by_id::Builder {
                client: self.0.clone(),
                id: id.into(),
                include_model_definition: None,
            }
        }
        #[doc = "Updates the metadata for a model.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no model with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        #[doc = "* `update_model`: An update specification described by JSON Patch. Only the decommissioned property can be replaced."]
        pub fn update(&self, id: impl Into<String>, update_model: Vec<serde_json::Value>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                id: id.into(),
                update_model,
            }
        }
        #[doc = "Deletes a model. A model can only be deleted if no other models reference it.\nStatus codes:\n204 (No Content): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no model with the provided id.\n409 (Conflict): There are dependencies on the model that prevent it from being deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for the model. The id is globally unique and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PagedModelDataCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) dependencies_for: Vec<String>,
            pub(crate) include_model_definition: Option<bool>,
            pub(crate) x_ms_max_item_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The set of the models which will have their dependencies retrieved. If omitted, all models are retrieved."]
            pub fn dependencies_for(mut self, dependencies_for: Vec<String>) -> Self {
                self.dependencies_for = dependencies_for;
                self
            }
            #[doc = "When true the model definition will be returned as part of the result."]
            pub fn include_model_definition(mut self, include_model_definition: bool) -> Self {
                self.include_model_definition = Some(include_model_definition);
                self
            }
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested max."]
            pub fn x_ms_max_item_count(mut self, x_ms_max_item_count: i64) -> Self {
                self.x_ms_max_item_count = Some(x_ms_max_item_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/models", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                                let dependencies_for = &this.dependencies_for;
                                for value in &this.dependencies_for {
                                    req.url_mut().query_pairs_mut().append_pair("dependenciesFor", &value.to_string());
                                }
                                if let Some(include_model_definition) = &this.include_model_definition {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeModelDefinition", &include_model_definition.to_string());
                                }
                                if let Some(x_ms_max_item_count) = &this.x_ms_max_item_count {
                                    req.insert_header("x-ms-max-item-count", &x_ms_max_item_count.to_string());
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
                                let rsp_value: models::PagedModelDataCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = models::NonPagedModelDataCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) models: Vec<serde_json::Value>,
        }
        impl Builder {
            #[doc = "An array of models to add."]
            pub fn models(mut self, models: Vec<serde_json::Value>) -> Self {
                self.models = models;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/models", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.models)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NonPagedModelDataCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_by_id {
        use super::models;
        type Response = models::ModelData;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) include_model_definition: Option<bool>,
        }
        impl Builder {
            #[doc = "When true the model definition will be returned as part of the result."]
            pub fn include_model_definition(mut self, include_model_definition: bool) -> Self {
                self.include_model_definition = Some(include_model_definition);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/models/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        if let Some(include_model_definition) = &this.include_model_definition {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeModelDefinition", &include_model_definition.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ModelData = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) update_model: Vec<serde_json::Value>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/models/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.update_model)?;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/models/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
pub mod query {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Executes a query that allows traversing relationships and filtering by property values.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `query_specification`: The query specification to execute."]
        pub fn query_twins(&self, query_specification: impl Into<models::QuerySpecification>) -> query_twins::Builder {
            query_twins::Builder {
                client: self.0.clone(),
                query_specification: query_specification.into(),
            }
        }
    }
    pub mod query_twins {
        use super::models;
        type Response = models::QueryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) query_specification: models::QuerySpecification,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/query", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.query_specification)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryResult = serde_json::from_slice(&rsp_body)?;
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
pub mod digital_twins {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves a digital twin.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::Builder {
            get_by_id::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Adds or replaces a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n412 (Precondition Failed): The model is decommissioned or the digital twin already exists (when using If-None-Match: *)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `twin`: The digital twin instance being added. If provided, the $dtId property is ignored."]
        pub fn add(&self, id: impl Into<String>, twin: impl Into<serde_json::Value>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                id: id.into(),
                twin: twin.into(),
                if_none_match: None,
            }
        }
        #[doc = "Updates a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `patch_document`: An update specification described by JSON Patch. Updates to property values and $model elements may happen in the same request. Operations are limited to add, replace and remove."]
        pub fn update(&self, id: impl Into<String>, patch_document: Vec<serde_json::Value>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                id: id.into(),
                patch_document,
                if_match: None,
            }
        }
        #[doc = "Deletes a digital twin. All relationships referencing the digital twin must already be deleted.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
                if_match: None,
            }
        }
        #[doc = "Retrieves a relationship between two digital twins.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is either no digital twin or relationship with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn get_relationship_by_id(&self, id: impl Into<String>, relationship_id: impl Into<String>) -> get_relationship_by_id::Builder {
            get_relationship_by_id::Builder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
            }
        }
        #[doc = "Adds a relationship between two digital twins.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is either no digital twin, target digital twin, or relationship with the provided id.\n409 (Conflict): A relationship with the provided id already exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn add_relationship(&self, id: impl Into<String>, relationship_id: impl Into<String>) -> add_relationship::Builder {
            add_relationship::Builder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                relationship: None,
                if_none_match: None,
            }
        }
        #[doc = "Updates the properties on a relationship between two digital twins.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is either no digital twin or relationship with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn update_relationship(&self, id: impl Into<String>, relationship_id: impl Into<String>) -> update_relationship::Builder {
            update_relationship::Builder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                patch_document: Vec::new(),
                if_match: None,
            }
        }
        #[doc = "Deletes a relationship between two digital twins.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is either no digital twin or relationship with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `relationship_id`: The id of the relationship. The id is unique within the digital twin and case sensitive."]
        pub fn delete_relationship(&self, id: impl Into<String>, relationship_id: impl Into<String>) -> delete_relationship::Builder {
            delete_relationship::Builder {
                client: self.0.clone(),
                id: id.into(),
                relationship_id: relationship_id.into(),
                if_match: None,
            }
        }
        #[doc = "Retrieves the relationships from a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn list_relationships(&self, id: impl Into<String>) -> list_relationships::Builder {
            list_relationships::Builder {
                client: self.0.clone(),
                id: id.into(),
                relationship_name: None,
            }
        }
        #[doc = "Retrieves all incoming relationship for a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        pub fn list_incoming_relationships(&self, id: impl Into<String>) -> list_incoming_relationships::Builder {
            list_incoming_relationships::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Sends telemetry on behalf of a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is no digital twin with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `telemetry`: The telemetry measurements to send from the digital twin."]
        #[doc = "* `dt_id`: A unique message identifier (in the scope of the digital twin id) that is commonly used for de-duplicating messages."]
        pub fn send_telemetry(
            &self,
            id: impl Into<String>,
            telemetry: impl Into<serde_json::Value>,
            dt_id: impl Into<String>,
        ) -> send_telemetry::Builder {
            send_telemetry::Builder {
                client: self.0.clone(),
                id: id.into(),
                telemetry: telemetry.into(),
                dt_id: dt_id.into(),
                dt_timestamp: None,
            }
        }
        #[doc = "Sends telemetry on behalf of a component in a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is either no digital twin with the provided id or the component path is invalid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        #[doc = "* `telemetry`: The telemetry measurements to send from the digital twin's component."]
        #[doc = "* `dt_id`: A unique message identifier (in the scope of the digital twin id) that is commonly used for de-duplicating messages."]
        pub fn send_component_telemetry(
            &self,
            id: impl Into<String>,
            component_path: impl Into<String>,
            telemetry: impl Into<serde_json::Value>,
            dt_id: impl Into<String>,
        ) -> send_component_telemetry::Builder {
            send_component_telemetry::Builder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
                telemetry: telemetry.into(),
                dt_id: dt_id.into(),
                dt_timestamp: None,
            }
        }
        #[doc = "Retrieves a component from a digital twin.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is either no digital twin with the provided id or the component path is invalid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        pub fn get_component(&self, id: impl Into<String>, component_path: impl Into<String>) -> get_component::Builder {
            get_component::Builder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
            }
        }
        #[doc = "Updates a component on a digital twin.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid.\n404 (Not Found): There is either no digital twin with the provided id or the component path is invalid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id of the digital twin. The id is unique within the service and case sensitive."]
        #[doc = "* `component_path`: The name of the DTDL component."]
        pub fn update_component(&self, id: impl Into<String>, component_path: impl Into<String>) -> update_component::Builder {
            update_component::Builder {
                client: self.0.clone(),
                id: id.into(),
                component_path: component_path.into(),
                patch_document: Vec::new(),
                if_match: None,
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
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
    pub mod add {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(serde_json::Value),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) twin: serde_json::Value,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "Only perform the operation if the entity does not already exist."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.twin)?;
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
            pub(crate) id: String,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/digitaltwins/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
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
    pub mod get_relationship_by_id {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/relationships/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.relationship_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
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
    pub mod add_relationship {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) relationship: Option<serde_json::Value>,
            pub(crate) if_none_match: Option<String>,
        }
        impl Builder {
            #[doc = "The data for the relationship."]
            pub fn relationship(mut self, relationship: impl Into<serde_json::Value>) -> Self {
                self.relationship = Some(relationship.into());
                self
            }
            #[doc = "Only perform the operation if the entity does not already exist."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/relationships/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.relationship_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = if let Some(relationship) = &this.relationship {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(relationship)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
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
    pub mod update_relationship {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "JSON Patch description of the update to the relationship properties."]
            pub fn patch_document(mut self, patch_document: Vec<serde_json::Value>) -> Self {
                self.patch_document = patch_document;
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/relationships/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.relationship_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
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
    pub mod delete_relationship {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_id: String,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/relationships/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.relationship_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
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
    pub mod list_relationships {
        use super::models;
        type Response = models::RelationshipCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) relationship_name: Option<String>,
        }
        impl Builder {
            #[doc = "The name of the relationship."]
            pub fn relationship_name(mut self, relationship_name: impl Into<String>) -> Self {
                self.relationship_name = Some(relationship_name.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/digitaltwins/{}/relationships", this.client.endpoint(), &this.id))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                                if let Some(relationship_name) = &this.relationship_name {
                                    req.url_mut().query_pairs_mut().append_pair("relationshipName", relationship_name);
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
                                let rsp_value: models::RelationshipCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_incoming_relationships {
        use super::models;
        type Response = models::IncomingRelationshipCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/incomingrelationships",
                            this.client.endpoint(),
                            &this.id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IncomingRelationshipCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod send_telemetry {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) telemetry: serde_json::Value,
            pub(crate) dt_id: String,
            pub(crate) dt_timestamp: Option<String>,
        }
        impl Builder {
            #[doc = "An RFC 3339 timestamp that identifies the time the telemetry was measured."]
            pub fn dt_timestamp(mut self, dt_timestamp: impl Into<String>) -> Self {
                self.dt_timestamp = Some(dt_timestamp.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/digitaltwins/{}/telemetry", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.telemetry)?;
                        req.insert_header("dt-id", &this.dt_id);
                        if let Some(dt_timestamp) = &this.dt_timestamp {
                            req.insert_header("dt-timestamp", dt_timestamp);
                        }
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
    pub mod send_component_telemetry {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
            pub(crate) telemetry: serde_json::Value,
            pub(crate) dt_id: String,
            pub(crate) dt_timestamp: Option<String>,
        }
        impl Builder {
            #[doc = "An RFC 3339 timestamp that identifies the time the telemetry was measured."]
            pub fn dt_timestamp(mut self, dt_timestamp: impl Into<String>) -> Self {
                self.dt_timestamp = Some(dt_timestamp.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/components/{}/telemetry",
                            this.client.endpoint(),
                            &this.id,
                            &this.component_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.telemetry)?;
                        req.insert_header("dt-id", &this.dt_id);
                        if let Some(dt_timestamp) = &this.dt_timestamp {
                            req.insert_header("dt-timestamp", dt_timestamp);
                        }
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
    pub mod get_component {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/components/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.component_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
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
    pub mod update_component {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            NoContent204,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) component_path: String,
            pub(crate) patch_document: Vec<serde_json::Value>,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "An update specification described by JSON Patch. Updates to property values and $model elements may happen in the same request. Operations are limited to add, replace and remove."]
            pub fn patch_document(mut self, patch_document: Vec<serde_json::Value>) -> Self {
                self.patch_document = patch_document;
                self
            }
            #[doc = "Only perform the operation if the entity's etag matches one of the etags provided or * is provided."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/digitaltwins/{}/components/{}",
                            this.client.endpoint(),
                            &this.id,
                            &this.component_path
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.patch_document)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
pub mod event_routes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves all event routes.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                x_ms_max_item_count: None,
            }
        }
        #[doc = "Retrieves an event route.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is no event route with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::Builder {
            get_by_id::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Adds or replaces an event route.\nStatus codes:\n200 (OK): Success.\n400 (Bad Request): The request is invalid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        pub fn add(&self, id: impl Into<String>) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                id: id.into(),
                event_route: None,
            }
        }
        #[doc = "Deletes an event route.\nStatus codes:\n200 (OK): Success.\n404 (Not Found): There is no event route with the provided id."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The id for an event route. The id is unique within event routes and case sensitive."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::EventRouteCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_max_item_count: Option<i64>,
        }
        impl Builder {
            #[doc = "The maximum number of items to retrieve per request. The server may choose to return less than the requested max."]
            pub fn x_ms_max_item_count(mut self, x_ms_max_item_count: i64) -> Self {
                self.x_ms_max_item_count = Some(x_ms_max_item_count);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/eventroutes", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                                if let Some(x_ms_max_item_count) = &this.x_ms_max_item_count {
                                    req.insert_header("x-ms-max-item-count", &x_ms_max_item_count.to_string());
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
                                let rsp_value: models::EventRouteCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_by_id {
        use super::models;
        type Response = models::EventRoute;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/eventroutes/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventRoute = serde_json::from_slice(&rsp_body)?;
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
    pub mod add {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) event_route: Option<models::EventRoute>,
        }
        impl Builder {
            #[doc = "The event route data"]
            pub fn event_route(mut self, event_route: impl Into<models::EventRoute>) -> Self {
                self.event_route = Some(event_route.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/eventroutes/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
                        let req_body = if let Some(event_route) = &this.event_route {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(event_route)?
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/eventroutes/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-05-31-preview");
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
