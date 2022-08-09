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
pub const DEFAULT_ENDPOINT: &str = "https://your-dps.azure-devices-provisioning.net";
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
    pub fn device_registration_state_client(&self) -> device_registration_state::Client {
        device_registration_state::Client(self.clone())
    }
    pub fn enrollment_group_client(&self) -> enrollment_group::Client {
        enrollment_group::Client(self.clone())
    }
    pub fn individual_enrollment_client(&self) -> individual_enrollment::Client {
        individual_enrollment::Client(self.clone())
    }
}
pub mod individual_enrollment {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a device enrollment record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: This id is used to uniquely identify a device registration of an enrollment. A case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Create or update a device enrollment record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: This id is used to uniquely identify a device registration of an enrollment. A case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
        #[doc = "* `enrollment`: The device enrollment record."]
        pub fn create_or_update(
            &self,
            id: impl Into<String>,
            enrollment: impl Into<models::IndividualEnrollment>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                id: id.into(),
                enrollment: enrollment.into(),
                if_match: None,
            }
        }
        #[doc = "Delete a device enrollment record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: This id is used to uniquely identify a device registration of an enrollment. A case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
                if_match: None,
            }
        }
        #[doc = "Query the device enrollment records."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `query_specification`: The query specification."]
        pub fn query(&self, query_specification: impl Into<models::QuerySpecification>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                query_specification: query_specification.into(),
                x_ms_max_item_count: None,
                x_ms_continuation: None,
            }
        }
        #[doc = "Get the attestation mechanism in the device enrollment record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: This id is used to uniquely identify a device registration of an enrollment. A case-insensitive string (up to 128 characters long) of alphanumeric characters plus certain special characters : . _ -. No special characters allowed at start or end."]
        pub fn get_attestation_mechanism(&self, id: impl Into<String>) -> get_attestation_mechanism::Builder {
            get_attestation_mechanism::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Bulk device enrollment operation with maximum of 10 enrollments."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bulk_operation`: Bulk operation."]
        pub fn run_bulk_operation(&self, bulk_operation: impl Into<models::BulkEnrollmentOperation>) -> run_bulk_operation::Builder {
            run_bulk_operation::Builder {
                client: self.0.clone(),
                bulk_operation: bulk_operation.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::IndividualEnrollment;
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
                        let url = azure_core::Url::parse(&format!("{}/enrollments/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IndividualEnrollment = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::IndividualEnrollment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) enrollment: models::IndividualEnrollment,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The ETag of the enrollment record."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollments/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.enrollment)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IndividualEnrollment = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) id: String,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The ETag of the enrollment record."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollments/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod query {
        use super::models;
        type Response = Vec<models::IndividualEnrollment>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) query_specification: models::QuerySpecification,
            pub(crate) x_ms_max_item_count: Option<i32>,
            pub(crate) x_ms_continuation: Option<String>,
        }
        impl Builder {
            #[doc = "Page size"]
            pub fn x_ms_max_item_count(mut self, x_ms_max_item_count: i32) -> Self {
                self.x_ms_max_item_count = Some(x_ms_max_item_count);
                self
            }
            #[doc = "Continuation token"]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollments/query", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(x_ms_max_item_count) = &this.x_ms_max_item_count {
                            req.insert_header("x-ms-max-item-count", &x_ms_max_item_count.to_string());
                        }
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.query_specification)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::IndividualEnrollment> = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_attestation_mechanism {
        use super::models;
        type Response = models::AttestationMechanism;
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
                        let url =
                            azure_core::Url::parse(&format!("{}/enrollments/{}/attestationmechanism", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AttestationMechanism = serde_json::from_slice(&rsp_body)?;
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
    pub mod run_bulk_operation {
        use super::models;
        type Response = models::BulkEnrollmentOperationResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bulk_operation: models::BulkEnrollmentOperation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollments", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.bulk_operation)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BulkEnrollmentOperationResult = serde_json::from_slice(&rsp_body)?;
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
pub mod enrollment_group {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a device enrollment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Enrollment group ID."]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Create or update a device enrollment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Enrollment group ID."]
        #[doc = "* `enrollment_group`: The device enrollment group."]
        pub fn create_or_update(
            &self,
            id: impl Into<String>,
            enrollment_group: impl Into<models::EnrollmentGroup>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                id: id.into(),
                enrollment_group: enrollment_group.into(),
                if_match: None,
            }
        }
        #[doc = "Delete a device enrollment group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Enrollment group ID."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
                if_match: None,
            }
        }
        #[doc = "Query the device enrollment groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `query_specification`: The query specification."]
        pub fn query(&self, query_specification: impl Into<models::QuerySpecification>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                query_specification: query_specification.into(),
                x_ms_max_item_count: None,
                x_ms_continuation: None,
            }
        }
        #[doc = "Get the attestation mechanism in the device enrollment group record."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Enrollment group ID"]
        pub fn get_attestation_mechanism(&self, id: impl Into<String>) -> get_attestation_mechanism::Builder {
            get_attestation_mechanism::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Bulk device enrollment group operation with maximum of 10 groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bulk_operation`: Bulk operation."]
        pub fn run_bulk_operation(&self, bulk_operation: impl Into<models::BulkEnrollmentGroupOperation>) -> run_bulk_operation::Builder {
            run_bulk_operation::Builder {
                client: self.0.clone(),
                bulk_operation: bulk_operation.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::EnrollmentGroup;
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
                        let url = azure_core::Url::parse(&format!("{}/enrollmentGroups/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EnrollmentGroup = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::EnrollmentGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) enrollment_group: models::EnrollmentGroup,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The ETag of the enrollment record."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollmentGroups/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.enrollment_group)?;
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EnrollmentGroup = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) id: String,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The ETag of the enrollment group record."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollmentGroups/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod query {
        use super::models;
        type Response = Vec<models::EnrollmentGroup>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) query_specification: models::QuerySpecification,
            pub(crate) x_ms_max_item_count: Option<i32>,
            pub(crate) x_ms_continuation: Option<String>,
        }
        impl Builder {
            #[doc = "Page size"]
            pub fn x_ms_max_item_count(mut self, x_ms_max_item_count: i32) -> Self {
                self.x_ms_max_item_count = Some(x_ms_max_item_count);
                self
            }
            #[doc = "Continuation token"]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollmentGroups/query", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(x_ms_max_item_count) = &this.x_ms_max_item_count {
                            req.insert_header("x-ms-max-item-count", &x_ms_max_item_count.to_string());
                        }
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.query_specification)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::EnrollmentGroup> = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_attestation_mechanism {
        use super::models;
        type Response = models::AttestationMechanism;
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
                        let url = azure_core::Url::parse(&format!(
                            "{}/enrollmentGroups/{}/attestationmechanism",
                            this.client.endpoint(),
                            &this.id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AttestationMechanism = serde_json::from_slice(&rsp_body)?;
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
    pub mod run_bulk_operation {
        use super::models;
        type Response = models::BulkEnrollmentGroupOperationResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bulk_operation: models::BulkEnrollmentGroupOperation,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/enrollmentGroups", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.bulk_operation)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BulkEnrollmentGroupOperationResult = serde_json::from_slice(&rsp_body)?;
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
pub mod device_registration_state {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the device registration state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Registration ID."]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Deletes the device registration"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Registration ID."]
        pub fn delete(&self, id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                id: id.into(),
                if_match: None,
            }
        }
        #[doc = "Gets the registration state of devices in this enrollmentGroup."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Enrollment group ID."]
        pub fn query(&self, id: impl Into<String>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                id: id.into(),
                x_ms_max_item_count: None,
                x_ms_continuation: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DeviceRegistrationState;
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
                        let url = azure_core::Url::parse(&format!("{}/registrations/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeviceRegistrationState = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) id: String,
            pub(crate) if_match: Option<String>,
        }
        impl Builder {
            #[doc = "The ETag of the registration status record."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/registrations/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod query {
        use super::models;
        type Response = Vec<models::DeviceRegistrationState>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) x_ms_max_item_count: Option<i32>,
            pub(crate) x_ms_continuation: Option<String>,
        }
        impl Builder {
            #[doc = "pageSize"]
            pub fn x_ms_max_item_count(mut self, x_ms_max_item_count: i32) -> Self {
                self.x_ms_max_item_count = Some(x_ms_max_item_count);
                self
            }
            #[doc = "continuation token"]
            pub fn x_ms_continuation(mut self, x_ms_continuation: impl Into<String>) -> Self {
                self.x_ms_continuation = Some(x_ms_continuation.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/registrations/{}/query", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(x_ms_max_item_count) = &this.x_ms_max_item_count {
                            req.insert_header("x-ms-max-item-count", &x_ms_max_item_count.to_string());
                        }
                        if let Some(x_ms_continuation) = &this.x_ms_continuation {
                            req.insert_header("x-ms-continuation", x_ms_continuation);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::DeviceRegistrationState> = serde_json::from_slice(&rsp_body)?;
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
