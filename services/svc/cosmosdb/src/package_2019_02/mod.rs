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
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
    pub fn table_client(&self) -> table::Client {
        table::Client(self.clone())
    }
}
pub mod service {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of an account's Table service, including properties for Analytics and CORS (Cross-Origin Resource Sharing) rules."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `restype`: Required query string to set the service properties."]
        #[doc = "* `comp`: Required query string to set the service properties."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_properties(
            &self,
            restype: impl Into<String>,
            comp: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_properties::Builder {
            get_properties::Builder {
                client: self.0.clone(),
                restype: restype.into(),
                comp: comp.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Sets properties for an account's Table service endpoint, including properties for Analytics and CORS (Cross-Origin Resource Sharing) rules."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `restype`: Required query string to set the service properties."]
        #[doc = "* `comp`: Required query string to set the service properties."]
        #[doc = "* `table_service_properties`: The Table Service properties."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn set_properties(
            &self,
            restype: impl Into<String>,
            comp: impl Into<String>,
            table_service_properties: impl Into<models::TableServiceProperties>,
            x_ms_version: impl Into<String>,
        ) -> set_properties::Builder {
            set_properties::Builder {
                client: self.0.clone(),
                restype: restype.into(),
                comp: comp.into(),
                table_service_properties: table_service_properties.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves statistics related to replication for the Table service. It is only available on the secondary location endpoint when read-access geo-redundant replication is enabled for the account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `restype`: Required query string to get service stats."]
        #[doc = "* `comp`: Required query string to get service stats."]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        pub fn get_statistics(
            &self,
            restype: impl Into<String>,
            comp: impl Into<String>,
            x_ms_version: impl Into<String>,
        ) -> get_statistics::Builder {
            get_statistics::Builder {
                client: self.0.clone(),
                restype: restype.into(),
                comp: comp.into(),
                x_ms_version: x_ms_version.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        type Response = models::TableServiceProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) restype: String,
            pub(crate) comp: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?ServiceProperties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let restype = &this.restype;
                        req.url_mut().query_pairs_mut().append_pair("restype", restype);
                        let comp = &this.comp;
                        req.url_mut().query_pairs_mut().append_pair("comp", comp);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
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
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableServiceProperties = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) restype: String,
            pub(crate) comp: String,
            pub(crate) table_service_properties: models::TableServiceProperties,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?ServiceProperties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let restype = &this.restype;
                        req.url_mut().query_pairs_mut().append_pair("restype", restype);
                        let comp = &this.comp;
                        req.url_mut().query_pairs_mut().append_pair("comp", comp);
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.table_service_properties)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
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
    pub mod get_statistics {
        use super::models;
        type Response = models::TableServiceStats;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) restype: String,
            pub(crate) comp: String,
            pub(crate) x_ms_version: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/?ServiceStats", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let restype = &this.restype;
                        req.url_mut().query_pairs_mut().append_pair("restype", restype);
                        let comp = &this.comp;
                        req.url_mut().query_pairs_mut().append_pair("comp", comp);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
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
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableServiceStats = serde_json::from_slice(&rsp_body)?;
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
pub mod table {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries tables under the given account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        pub fn query(&self, x_ms_version: impl Into<String>, data_service_version: impl Into<String>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                x_ms_client_request_id: None,
                format: None,
                top: None,
                select: None,
                filter: None,
                next_table_name: None,
            }
        }
        #[doc = "Creates a new table under the given account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table_properties`: The Table properties."]
        pub fn create(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table_properties: impl Into<models::TableProperties>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table_properties: table_properties.into(),
                x_ms_client_request_id: None,
                format: None,
                prefer: None,
            }
        }
        #[doc = "Operation permanently deletes the specified table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `table`: The name of the table."]
        pub fn delete(&self, x_ms_version: impl Into<String>, table: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                table: table.into(),
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Queries entities in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        pub fn query_entities(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
        ) -> query_entities::Builder {
            query_entities::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
                top: None,
                select: None,
                filter: None,
                next_partition_key: None,
                next_row_key: None,
            }
        }
        #[doc = "Queries a single entity in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `partition_key`: The partition key of the entity."]
        #[doc = "* `row_key`: The row key of the entity."]
        pub fn query_entity_with_partition_and_row_key(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
            partition_key: impl Into<String>,
            row_key: impl Into<String>,
        ) -> query_entity_with_partition_and_row_key::Builder {
            query_entity_with_partition_and_row_key::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                partition_key: partition_key.into(),
                row_key: row_key.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
                select: None,
                filter: None,
            }
        }
        #[doc = "Update entity in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `partition_key`: The partition key of the entity."]
        #[doc = "* `row_key`: The row key of the entity."]
        pub fn update_entity(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
            partition_key: impl Into<String>,
            row_key: impl Into<String>,
        ) -> update_entity::Builder {
            update_entity::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                partition_key: partition_key.into(),
                row_key: row_key.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
                table_entity_properties: None,
                if_match: None,
            }
        }
        #[doc = "Merge entity in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `partition_key`: The partition key of the entity."]
        #[doc = "* `row_key`: The row key of the entity."]
        pub fn merge_entity(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
            partition_key: impl Into<String>,
            row_key: impl Into<String>,
        ) -> merge_entity::Builder {
            merge_entity::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                partition_key: partition_key.into(),
                row_key: row_key.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
                table_entity_properties: None,
                if_match: None,
            }
        }
        #[doc = "Deletes the specified entity in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `partition_key`: The partition key of the entity."]
        #[doc = "* `row_key`: The row key of the entity."]
        #[doc = "* `if_match`: Match condition for an entity to be deleted. If specified and a matching entity is not found, an error will be raised. To force an unconditional delete, set to the wildcard character (*)."]
        pub fn delete_entity(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
            partition_key: impl Into<String>,
            row_key: impl Into<String>,
            if_match: impl Into<String>,
        ) -> delete_entity::Builder {
            delete_entity::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                partition_key: partition_key.into(),
                row_key: row_key.into(),
                if_match: if_match.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
            }
        }
        #[doc = "Retrieves details about any stored access policies specified on the table that may be used with Shared Access Signatures."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `comp`: Required query string to handle stored access policies for the table that may be used with Shared Access Signatures."]
        pub fn get_access_policy(
            &self,
            x_ms_version: impl Into<String>,
            table: impl Into<String>,
            comp: impl Into<String>,
        ) -> get_access_policy::Builder {
            get_access_policy::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                table: table.into(),
                comp: comp.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Insert entity in a table."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `data_service_version`: Specifies the data service version."]
        #[doc = "* `table`: The name of the table."]
        pub fn insert_entity(
            &self,
            x_ms_version: impl Into<String>,
            data_service_version: impl Into<String>,
            table: impl Into<String>,
        ) -> insert_entity::Builder {
            insert_entity::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                data_service_version: data_service_version.into(),
                table: table.into(),
                timeout: None,
                x_ms_client_request_id: None,
                format: None,
                table_entity_properties: None,
                prefer: None,
            }
        }
        #[doc = "Sets stored access policies for the table that may be used with Shared Access Signatures."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_ms_version`: Specifies the version of the operation to use for this request."]
        #[doc = "* `table`: The name of the table."]
        #[doc = "* `comp`: Required query string to handle stored access policies for the table that may be used with Shared Access Signatures."]
        pub fn set_access_policy(
            &self,
            x_ms_version: impl Into<String>,
            table: impl Into<String>,
            comp: impl Into<String>,
        ) -> set_access_policy::Builder {
            set_access_policy::Builder {
                client: self.0.clone(),
                x_ms_version: x_ms_version.into(),
                table: table.into(),
                comp: comp.into(),
                table_acl: None,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::TableQueryResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) next_table_name: Option<String>,
        }
        impl Builder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "A table query continuation token from a previous call."]
            pub fn next_table_name(mut self, next_table_name: impl Into<String>) -> Self {
                self.next_table_name = Some(next_table_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/Tables", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(next_table_name) = &this.next_table_name {
                            req.url_mut().query_pairs_mut().append_pair("NextTableName", next_table_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableQueryResponse = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::TableResponse),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table_properties: models::TableProperties,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) prefer: Option<String>,
        }
        impl Builder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "Specifies whether the response should include the inserted entity in the payload. Possible values are return-no-content and return-content."]
            pub fn prefer(mut self, prefer: impl Into<String>) -> Self {
                self.prefer = Some(prefer.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/Tables", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        req.insert_header("content-type", "application/json;odata=nometadata");
                        let req_body = azure_core::to_json(&this.table_properties)?;
                        if let Some(prefer) = &this.prefer {
                            req.insert_header("prefer", prefer);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) table: String,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/Tables('{}')", this.client.endpoint(), &this.table))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
    pub mod query_entities {
        use super::models;
        type Response = models::TableEntityQueryResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) next_partition_key: Option<String>,
            pub(crate) next_row_key: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "Maximum number of records to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "An entity query continuation token from a previous call."]
            pub fn next_partition_key(mut self, next_partition_key: impl Into<String>) -> Self {
                self.next_partition_key = Some(next_partition_key.into());
                self
            }
            #[doc = "An entity query continuation token from a previous call."]
            pub fn next_row_key(mut self, next_row_key: impl Into<String>) -> Self {
                self.next_row_key = Some(next_row_key.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}()", this.client.endpoint(), &this.table))?;
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
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(next_partition_key) = &this.next_partition_key {
                            req.url_mut().query_pairs_mut().append_pair("NextPartitionKey", next_partition_key);
                        }
                        if let Some(next_row_key) = &this.next_row_key {
                            req.url_mut().query_pairs_mut().append_pair("NextRowKey", next_row_key);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableEntityQueryResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod query_entity_with_partition_and_row_key {
        use super::models;
        type Response = models::TableEntityProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) partition_key: String,
            pub(crate) row_key: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "Select expression using OData notation. Limits the columns on each record to just those requested, e.g. \"$select=PolicyAssignmentId, ResourceId\"."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "OData filter expression."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}(PartitionKey='{}',RowKey='{}')",
                            this.client.endpoint(),
                            &this.table,
                            &this.partition_key,
                            &this.row_key
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
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableEntityProperties = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_entity {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) partition_key: String,
            pub(crate) row_key: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) table_entity_properties: Option<models::TableEntityProperties>,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "The properties for the table entity."]
            pub fn table_entity_properties(mut self, table_entity_properties: impl Into<models::TableEntityProperties>) -> Self {
                self.table_entity_properties = Some(table_entity_properties.into());
                self
            }
            #[doc = "Match condition for an entity to be updated. If specified and a matching entity is not found, an error will be raised. To force an unconditional update, set to the wildcard character (*). If not specified, an insert will be performed when no existing entity is found to update and a replace will be performed if an existing entity is found."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}(PartitionKey='{}',RowKey='{}')",
                            this.client.endpoint(),
                            &this.table,
                            &this.partition_key,
                            &this.row_key
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
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        let req_body = if let Some(table_entity_properties) = &this.table_entity_properties {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(table_entity_properties)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod merge_entity {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) partition_key: String,
            pub(crate) row_key: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) table_entity_properties: Option<models::TableEntityProperties>,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "The properties for the table entity."]
            pub fn table_entity_properties(mut self, table_entity_properties: impl Into<models::TableEntityProperties>) -> Self {
                self.table_entity_properties = Some(table_entity_properties.into());
                self
            }
            #[doc = "Match condition for an entity to be updated. If specified and a matching entity is not found, an error will be raised. To force an unconditional update, set to the wildcard character (*). If not specified, an insert will be performed when no existing entity is found to update and a merge will be performed if an existing entity is found."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}(PartitionKey='{}',RowKey='{}')",
                            this.client.endpoint(),
                            &this.table,
                            &this.partition_key,
                            &this.row_key
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
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
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        let req_body = if let Some(table_entity_properties) = &this.table_entity_properties {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(table_entity_properties)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod delete_entity {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) partition_key: String,
            pub(crate) row_key: String,
            pub(crate) if_match: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}(PartitionKey='{}',RowKey='{}')",
                            this.client.endpoint(),
                            &this.table,
                            &this.partition_key,
                            &this.row_key
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
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        req.insert_header("if-match", &this.if_match);
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
    pub mod get_access_policy {
        use super::models;
        type Response = models::SignedIdentifiers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) table: String,
            pub(crate) comp: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.table))?;
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
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let comp = &this.comp;
                        req.url_mut().query_pairs_mut().append_pair("comp", comp);
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
    pub mod insert_entity {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Created201(models::TableEntityProperties),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) data_service_version: String,
            pub(crate) table: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) format: Option<String>,
            pub(crate) table_entity_properties: Option<models::TableEntityProperties>,
            pub(crate) prefer: Option<String>,
        }
        impl Builder {
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the media type for the response."]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "The properties for the table entity."]
            pub fn table_entity_properties(mut self, table_entity_properties: impl Into<models::TableEntityProperties>) -> Self {
                self.table_entity_properties = Some(table_entity_properties.into());
                self
            }
            #[doc = "Specifies whether the response should include the inserted entity in the payload. Possible values are return-no-content and return-content."]
            pub fn prefer(mut self, prefer: impl Into<String>) -> Self {
                self.prefer = Some(prefer.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.table))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                        req.insert_header("dataserviceversion", &this.data_service_version);
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        let req_body = if let Some(table_entity_properties) = &this.table_entity_properties {
                            req.insert_header("content-type", "application/json;odata=nometadata");
                            azure_core::to_json(table_entity_properties)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(prefer) = &this.prefer {
                            req.insert_header("prefer", prefer);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TableEntityProperties = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
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
    pub mod set_access_policy {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) x_ms_version: String,
            pub(crate) table: String,
            pub(crate) comp: String,
            pub(crate) table_acl: Option<models::SignedIdentifiers>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl Builder {
            #[doc = "The acls for the table."]
            pub fn table_acl(mut self, table_acl: impl Into<models::SignedIdentifiers>) -> Self {
                self.table_acl = Some(table_acl.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds."]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.table))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(table_acl) = &this.table_acl {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(table_acl)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-version", &this.x_ms_version);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let comp = &this.comp;
                        req.url_mut().query_pairs_mut().append_pair("comp", comp);
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
