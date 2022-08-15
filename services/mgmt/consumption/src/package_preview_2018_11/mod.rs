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
    pub fn billing_profile_pricesheet_client(&self) -> billing_profile_pricesheet::Client {
        billing_profile_pricesheet::Client(self.clone())
    }
    pub fn charges_by_billing_account_client(&self) -> charges_by_billing_account::Client {
        charges_by_billing_account::Client(self.clone())
    }
    pub fn charges_by_billing_profile_client(&self) -> charges_by_billing_profile::Client {
        charges_by_billing_profile::Client(self.clone())
    }
    pub fn charges_by_invoice_section_client(&self) -> charges_by_invoice_section::Client {
        charges_by_invoice_section::Client(self.clone())
    }
    pub fn credit_summary_by_billing_profile_client(&self) -> credit_summary_by_billing_profile::Client {
        credit_summary_by_billing_profile::Client(self.clone())
    }
    pub fn events_by_billing_profile_client(&self) -> events_by_billing_profile::Client {
        events_by_billing_profile::Client(self.clone())
    }
    pub fn invoice_pricesheet_client(&self) -> invoice_pricesheet::Client {
        invoice_pricesheet::Client(self.clone())
    }
    pub fn lots_by_billing_profile_client(&self) -> lots_by_billing_profile::Client {
        lots_by_billing_profile::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available consumption REST API operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Consumption/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod credit_summary_by_billing_profile {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The credit summary by billingAccountId and billingProfileId for given start and end date."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `billing_profile_id`: Billing Profile Id."]
        pub fn get(&self, billing_account_id: impl Into<String>, billing_profile_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                billing_profile_id: billing_profile_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::CreditSummary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) billing_profile_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Consumption/credits/balanceSummary" , this . client . endpoint () , & this . billing_account_id , & this . billing_profile_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CreditSummary = serde_json::from_slice(&rsp_body)?;
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
pub mod events_by_billing_profile {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the events by billingAccountId and billingProfileId for given start and end date."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `billing_profile_id`: Billing Profile Id."]
        #[doc = "* `start_date`: Start date"]
        #[doc = "* `end_date`: End date"]
        pub fn list(
            &self,
            billing_account_id: impl Into<String>,
            billing_profile_id: impl Into<String>,
            start_date: impl Into<String>,
            end_date: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                billing_profile_id: billing_profile_id.into(),
                start_date: start_date.into(),
                end_date: end_date.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::Events;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) billing_profile_id: String,
            pub(crate) start_date: String,
            pub(crate) end_date: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Consumption/events",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.billing_profile_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let start_date = &this.start_date;
                        req.url_mut().query_pairs_mut().append_pair("startDate", start_date);
                        let end_date = &this.end_date;
                        req.url_mut().query_pairs_mut().append_pair("endDate", end_date);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Events = serde_json::from_slice(&rsp_body)?;
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
pub mod lots_by_billing_profile {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the lots by billingAccountId and billingProfileId for given start and end date."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `billing_profile_id`: Billing Profile Id."]
        pub fn list(&self, billing_account_id: impl Into<String>, billing_profile_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                billing_profile_id: billing_profile_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::Lots;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) billing_profile_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Consumption/lots",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.billing_profile_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lots = serde_json::from_slice(&rsp_body)?;
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
pub mod invoice_pricesheet {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get pricesheet data for invoice id (invoiceName)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: Azure Billing Account Id."]
        #[doc = "* `invoice_name`: The name of an invoice resource."]
        pub fn download(&self, billing_account_id: impl Into<String>, invoice_name: impl Into<String>) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                invoice_name: invoice_name.into(),
            }
        }
    }
    pub mod download {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::PricesheetDownloadResponse),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) invoice_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Consumption/billingAccounts/{}/invoices/{}/pricesheet/default/download",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.invoice_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PricesheetDownloadResponse = serde_json::from_slice(&rsp_body)?;
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
}
pub mod billing_profile_pricesheet {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get pricesheet data for invoice id (invoiceName)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: Azure Billing Account Id."]
        #[doc = "* `billing_profile_id`: Azure Billing Profile Id."]
        pub fn download(&self, billing_account_id: impl Into<String>, billing_profile_id: impl Into<String>) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                billing_profile_id: billing_profile_id.into(),
            }
        }
    }
    pub mod download {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::PricesheetDownloadResponse),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) billing_profile_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Consumption/billingAccounts/{}/billingProfiles/{}/pricesheet/default/download",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.billing_profile_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PricesheetDownloadResponse = serde_json::from_slice(&rsp_body)?;
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
}
pub mod charges_by_billing_account {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the charges by billingAccountId for given start and end date. Start and end date are used to determine the billing period. For current month, the data will be provided from month to date. If there are no charges for a month then that month will show all zeroes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `start_date`: Start date"]
        #[doc = "* `end_date`: End date"]
        pub fn list(
            &self,
            billing_account_id: impl Into<String>,
            start_date: impl Into<String>,
            end_date: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                start_date: start_date.into(),
                end_date: end_date.into(),
                apply: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ChargesListByBillingAccount;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) start_date: String,
            pub(crate) end_date: String,
            pub(crate) apply: Option<String>,
        }
        impl Builder {
            #[doc = "May be used to group charges by properties/billingProfileId, or properties/invoiceSectionId."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/providers/Microsoft.Consumption/charges",
                            this.client.endpoint(),
                            &this.billing_account_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let start_date = &this.start_date;
                        req.url_mut().query_pairs_mut().append_pair("startDate", start_date);
                        let end_date = &this.end_date;
                        req.url_mut().query_pairs_mut().append_pair("endDate", end_date);
                        if let Some(apply) = &this.apply {
                            req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChargesListByBillingAccount = serde_json::from_slice(&rsp_body)?;
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
pub mod charges_by_billing_profile {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the charges by billing profile id for given start and end date. Start and end date are used to determine the billing period. For current month, the data will be provided from month to date. If there are no charges for a month then that month will show all zeroes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `billing_profile_id`: Billing Profile Id."]
        #[doc = "* `start_date`: Start date"]
        #[doc = "* `end_date`: End date"]
        pub fn list(
            &self,
            billing_account_id: impl Into<String>,
            billing_profile_id: impl Into<String>,
            start_date: impl Into<String>,
            end_date: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                billing_profile_id: billing_profile_id.into(),
                start_date: start_date.into(),
                end_date: end_date.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ChargesListByBillingProfile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) billing_profile_id: String,
            pub(crate) start_date: String,
            pub(crate) end_date: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Consumption/charges",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.billing_profile_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let start_date = &this.start_date;
                        req.url_mut().query_pairs_mut().append_pair("startDate", start_date);
                        let end_date = &this.end_date;
                        req.url_mut().query_pairs_mut().append_pair("endDate", end_date);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChargesListByBillingProfile = serde_json::from_slice(&rsp_body)?;
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
pub mod charges_by_invoice_section {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the charges by invoice section id for given start and end date. Start and end date are used to determine the billing period. For current month, the data will be provided from month to date. If there are no charges for a month then that month will show all zeroes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_id`: BillingAccount ID"]
        #[doc = "* `invoice_section_id`: Invoice Section Id."]
        #[doc = "* `start_date`: Start date"]
        #[doc = "* `end_date`: End date"]
        pub fn list(
            &self,
            billing_account_id: impl Into<String>,
            invoice_section_id: impl Into<String>,
            start_date: impl Into<String>,
            end_date: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_id: billing_account_id.into(),
                invoice_section_id: invoice_section_id.into(),
                start_date: start_date.into(),
                end_date: end_date.into(),
                apply: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ChargesListByInvoiceSection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_id: String,
            pub(crate) invoice_section_id: String,
            pub(crate) start_date: String,
            pub(crate) end_date: String,
            pub(crate) apply: Option<String>,
        }
        impl Builder {
            #[doc = "May be used to group charges by properties/productName."]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/invoiceSections/{}/providers/Microsoft.Consumption/charges",
                            this.client.endpoint(),
                            &this.billing_account_id,
                            &this.invoice_section_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-11-01-preview");
                        let start_date = &this.start_date;
                        req.url_mut().query_pairs_mut().append_pair("startDate", start_date);
                        let end_date = &this.end_date;
                        req.url_mut().query_pairs_mut().append_pair("endDate", end_date);
                        if let Some(apply) = &this.apply {
                            req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ChargesListByInvoiceSection = serde_json::from_slice(&rsp_body)?;
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
