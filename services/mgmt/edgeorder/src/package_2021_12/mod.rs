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
}
impl Client {
    #[doc = "This method gets all the operations that are exposed for customer."]
    pub fn list_operations(&self) -> list_operations::RequestBuilder {
        list_operations::RequestBuilder { client: self.clone() }
    }
    #[doc = "Lists all the addresses available under the subscription."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    pub fn list_addresses_at_subscription_level(
        &self,
        subscription_id: impl Into<String>,
    ) -> list_addresses_at_subscription_level::RequestBuilder {
        list_addresses_at_subscription_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            filter: None,
            skip_token: None,
        }
    }
    #[doc = "This method provides the list of product families for the given subscription."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `product_families_request`: Filters for showing the product families."]
    pub fn list_product_families(
        &self,
        subscription_id: impl Into<String>,
        product_families_request: impl Into<models::ProductFamiliesRequest>,
    ) -> list_product_families::RequestBuilder {
        list_product_families::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            product_families_request: product_families_request.into(),
            expand: None,
            skip_token: None,
        }
    }
    #[doc = "This method provides the list of configurations for the given product family, product line and product under subscription."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `configurations_request`: Filters for showing the configurations."]
    pub fn list_configurations(
        &self,
        subscription_id: impl Into<String>,
        configurations_request: impl Into<models::ConfigurationsRequest>,
    ) -> list_configurations::RequestBuilder {
        list_configurations::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            configurations_request: configurations_request.into(),
            skip_token: None,
        }
    }
    #[doc = "This method provides the list of product families metadata for the given subscription."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    pub fn list_product_families_metadata(&self, subscription_id: impl Into<String>) -> list_product_families_metadata::RequestBuilder {
        list_product_families_metadata::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            skip_token: None,
        }
    }
    #[doc = "Lists order at subscription level."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    pub fn list_order_at_subscription_level(&self, subscription_id: impl Into<String>) -> list_order_at_subscription_level::RequestBuilder {
        list_order_at_subscription_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            skip_token: None,
        }
    }
    #[doc = "Lists order item at subscription level."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    pub fn list_order_items_at_subscription_level(
        &self,
        subscription_id: impl Into<String>,
    ) -> list_order_items_at_subscription_level::RequestBuilder {
        list_order_items_at_subscription_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            filter: None,
            expand: None,
            skip_token: None,
        }
    }
    #[doc = "Lists all the addresses available under the given resource group."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn list_addresses_at_resource_group_level(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> list_addresses_at_resource_group_level::RequestBuilder {
        list_addresses_at_resource_group_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            filter: None,
            skip_token: None,
        }
    }
    #[doc = "Gets information about the specified address."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `address_name`: The name of the address Resource within the specified resource group. address names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn get_address_by_name(
        &self,
        address_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> get_address_by_name::RequestBuilder {
        get_address_by_name::RequestBuilder {
            client: self.clone(),
            address_name: address_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
        }
    }
    #[doc = "Creates a new address with the specified parameters. Existing address can be updated with this API"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `address_name`: The name of the address Resource within the specified resource group. address names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `address_resource`: Address details from request body."]
    pub fn create_address(
        &self,
        address_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        address_resource: impl Into<models::AddressResource>,
    ) -> create_address::RequestBuilder {
        create_address::RequestBuilder {
            client: self.clone(),
            address_name: address_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            address_resource: address_resource.into(),
        }
    }
    #[doc = "Updates the properties of an existing address."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `address_name`: The name of the address Resource within the specified resource group. address names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `address_update_parameter`: Address update parameters from request body."]
    pub fn update_address(
        &self,
        address_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        address_update_parameter: impl Into<models::AddressUpdateParameter>,
    ) -> update_address::RequestBuilder {
        update_address::RequestBuilder {
            client: self.clone(),
            address_name: address_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            address_update_parameter: address_update_parameter.into(),
            if_match: None,
        }
    }
    #[doc = "Deletes an address."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `address_name`: The name of the address Resource within the specified resource group. address names must be between 3 and 24 characters in length and use any alphanumeric and underscore only"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn delete_address_by_name(
        &self,
        address_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> delete_address_by_name::RequestBuilder {
        delete_address_by_name::RequestBuilder {
            client: self.clone(),
            address_name: address_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
        }
    }
    #[doc = "Lists order at resource group level."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn list_order_at_resource_group_level(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> list_order_at_resource_group_level::RequestBuilder {
        list_order_at_resource_group_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            skip_token: None,
        }
    }
    #[doc = "Gets an order."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_name`: The name of the order"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `location`: The name of Azure region."]
    pub fn get_order_by_name(
        &self,
        order_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        location: impl Into<String>,
    ) -> get_order_by_name::RequestBuilder {
        get_order_by_name::RequestBuilder {
            client: self.clone(),
            order_name: order_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            location: location.into(),
        }
    }
    #[doc = "Lists order item at resource group level."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn list_order_items_at_resource_group_level(
        &self,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> list_order_items_at_resource_group_level::RequestBuilder {
        list_order_items_at_resource_group_level::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            filter: None,
            expand: None,
            skip_token: None,
        }
    }
    #[doc = "Gets an order item."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn get_order_item_by_name(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> get_order_item_by_name::RequestBuilder {
        get_order_item_by_name::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            expand: None,
        }
    }
    #[doc = "Creates an order item. Existing order item cannot be updated with this api and should instead be updated with the Update order item API."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `order_item_resource`: Order item details from request body."]
    pub fn create_order_item(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        order_item_resource: impl Into<models::OrderItemResource>,
    ) -> create_order_item::RequestBuilder {
        create_order_item::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            order_item_resource: order_item_resource.into(),
        }
    }
    #[doc = "Updates the properties of an existing order item."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `order_item_update_parameter`: order item update parameters from request body."]
    pub fn update_order_item(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        order_item_update_parameter: impl Into<models::OrderItemUpdateParameter>,
    ) -> update_order_item::RequestBuilder {
        update_order_item::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            order_item_update_parameter: order_item_update_parameter.into(),
            if_match: None,
        }
    }
    #[doc = "Deletes an order item."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    pub fn delete_order_item_by_name(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
    ) -> delete_order_item_by_name::RequestBuilder {
        delete_order_item_by_name::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
        }
    }
    #[doc = "Cancel order item."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `cancellation_reason`: Reason for cancellation."]
    pub fn cancel_order_item(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        cancellation_reason: impl Into<models::CancellationReason>,
    ) -> cancel_order_item::RequestBuilder {
        cancel_order_item::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            cancellation_reason: cancellation_reason.into(),
        }
    }
    #[doc = "Return order item."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `order_item_name`: The name of the order item"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
    #[doc = "* `return_order_item_details`: Return order item CurrentStatus."]
    pub fn return_order_item(
        &self,
        order_item_name: impl Into<String>,
        subscription_id: impl Into<String>,
        resource_group_name: impl Into<String>,
        return_order_item_details: impl Into<models::ReturnOrderItemDetails>,
    ) -> return_order_item::RequestBuilder {
        return_order_item::RequestBuilder {
            client: self.clone(),
            order_item_name: order_item_name.into(),
            subscription_id: subscription_id.into(),
            resource_group_name: resource_group_name.into(),
            return_order_item_details: return_order_item_details.into(),
        }
    }
}
pub mod list_operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
    #[doc = r""]
    #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
    #[doc = r" parameters can be chained."]
    #[doc = r""]
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
    }
    impl RequestBuilder {
        pub fn into_stream(self) -> azure_core::Pageable<models::OperationListResult, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.EdgeOrder/operations", this.client.endpoint(),))?;
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
pub mod list_addresses_at_subscription_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::AddressResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::AddressResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) filter: Option<String>,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$filter is supported to filter based on shipping address properties. Filter supports only equals operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        #[doc = "$skipToken is supported on Get list of addresses, which provides the next page in the list of addresses."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::AddressResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/addresses",
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod list_product_families {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::ProductFamilies> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::ProductFamilies = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) product_families_request: models::ProductFamiliesRequest,
        pub(crate) expand: Option<String>,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$expand is supported on configurations parameter for product, which provides details on the configurations for the product."]
        pub fn expand(mut self, expand: impl Into<String>) -> Self {
            self.expand = Some(expand.into());
            self
        }
        #[doc = "$skipToken is supported on list of product families, which provides the next page in the list of product families."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::ProductFamilies, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/listProductFamilies",
                        this.client.endpoint(),
                        &this.subscription_id
                    ))?;
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
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            }
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
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(expand) = &this.expand {
                                req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                            }
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                            }
                            req.insert_header("content-type", "application/json");
                            let req_body = azure_core::to_json(&this.product_families_request)?;
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
pub mod list_configurations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::Configurations> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::Configurations = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) configurations_request: models::ConfigurationsRequest,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$skipToken is supported on list of configurations, which provides the next page in the list of configurations."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::Configurations, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/listConfigurations",
                        this.client.endpoint(),
                        &this.subscription_id
                    ))?;
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
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            }
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
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                            }
                            req.insert_header("content-type", "application/json");
                            let req_body = azure_core::to_json(&this.configurations_request)?;
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
pub mod list_product_families_metadata {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::ProductFamiliesMetadata> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::ProductFamiliesMetadata = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$skipToken is supported on list of product families metadata, which provides the next page in the list of product families metadata."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::ProductFamiliesMetadata, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/productFamiliesMetadata",
                        this.client.endpoint(),
                        &this.subscription_id
                    ))?;
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
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            }
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
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
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
pub mod list_order_at_subscription_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$skipToken is supported on Get list of order, which provides the next page in the list of order."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::OrderResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/orders",
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod list_order_items_at_subscription_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderItemResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderItemResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) filter: Option<String>,
        pub(crate) expand: Option<String>,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$filter is supported to filter based on order id. Filter supports only equals operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        #[doc = "$expand is supported on device details, forward shipping details and reverse shipping details parameters. Each of these can be provided as a comma separated list. Device Details for order item provides details on the devices of the product, Forward and Reverse Shipping details provide forward and reverse shipping details respectively."]
        pub fn expand(mut self, expand: impl Into<String>) -> Self {
            self.expand = Some(expand.into());
            self
        }
        #[doc = "$skipToken is supported on Get list of order items, which provides the next page in the list of order items."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::OrderItemResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrder/orderItems",
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(expand) = &this.expand {
                                req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                            }
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod list_addresses_at_resource_group_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::AddressResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::AddressResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$filter is supported to filter based on shipping address properties. Filter supports only equals operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        #[doc = "$skipToken is supported on Get list of addresses, which provides the next page in the list of address."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::AddressResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/addresses",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod get_address_by_name {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::AddressResource> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::AddressResource = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) address_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
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
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/addresses/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.address_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::AddressResource>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::AddressResource>>;
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
pub mod create_address {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::AddressResource> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::AddressResource = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) address_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) address_resource: models::AddressResource,
    }
    impl RequestBuilder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/addresses/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.address_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.address_resource)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::AddressResource>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::AddressResource>>;
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
pub mod update_address {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) address_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) address_update_parameter: models::AddressUpdateParameter,
        pub(crate) if_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "Defines the If-Match condition. The patch will be performed only if the ETag of the job on the server matches this value."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/addresses/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.address_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.address_update_parameter)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod delete_address_by_name {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) address_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
    }
    impl RequestBuilder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/addresses/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.address_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod list_order_at_resource_group_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$skipToken is supported on Get list of order, which provides the next page in the list of order."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::OrderResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orders",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod get_order_by_name {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderResource> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderResource = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) location: String,
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
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/locations/{}/orders/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.location,
                        &this.order_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::OrderResource>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::OrderResource>>;
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
pub mod list_order_items_at_resource_group_level {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderItemResourceList> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderItemResourceList = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) filter: Option<String>,
        pub(crate) expand: Option<String>,
        pub(crate) skip_token: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$filter is supported to filter based on order id. Filter supports only equals operation."]
        pub fn filter(mut self, filter: impl Into<String>) -> Self {
            self.filter = Some(filter.into());
            self
        }
        #[doc = "$expand is supported on device details, forward shipping details and reverse shipping details parameters. Each of these can be provided as a comma separated list. Device Details for order item provides details on the devices of the product, Forward and Reverse Shipping details provide forward and reverse shipping details respectively."]
        pub fn expand(mut self, expand: impl Into<String>) -> Self {
            self.expand = Some(expand.into());
            self
        }
        #[doc = "$skipToken is supported on Get list of order items, which provides the next page in the list of order items."]
        pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
            self.skip_token = Some(skip_token.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::OrderItemResourceList, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                            if let Some(filter) = &this.filter {
                                req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                            }
                            if let Some(expand) = &this.expand {
                                req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                            }
                            if let Some(skip_token) = &this.skip_token {
                                req.url_mut().query_pairs_mut().append_pair("$skipToken", skip_token);
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
    }
}
pub mod get_order_item_by_name {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderItemResource> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderItemResource = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) expand: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "$expand is supported on device details, forward shipping details and reverse shipping details parameters. Each of these can be provided as a comma separated list. Device Details for order item provides details on the devices of the product, Forward and Reverse Shipping details provide forward and reverse shipping details respectively."]
        pub fn expand(mut self, expand: impl Into<String>) -> Self {
            self.expand = Some(expand.into());
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
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    if let Some(expand) = &this.expand {
                        req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::OrderItemResource>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::OrderItemResource>>;
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
pub mod create_order_item {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::OrderItemResource> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::OrderItemResource = serde_json::from_slice(&bytes)?;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) order_item_resource: models::OrderItemResource,
    }
    impl RequestBuilder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.order_item_resource)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::OrderItemResource>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::OrderItemResource>>;
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
pub mod update_order_item {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) order_item_update_parameter: models::OrderItemUpdateParameter,
        pub(crate) if_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "Defines the If-Match condition. The patch will be performed only if the ETag of the order on the server matches this value."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.order_item_update_parameter)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod delete_order_item_by_name {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
    }
    impl RequestBuilder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod cancel_order_item {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) cancellation_reason: models::CancellationReason,
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
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}/cancel",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.cancellation_reason)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
pub mod return_order_item {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
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
    #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
    #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
    #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
    #[doc = r" operation and returns a `Result` with the parsed response."]
    #[doc = r""]
    #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
    #[doc = r" response headers or raw body data) then you can finalize the request using the"]
    #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
    #[doc = r" [`Response`] value."]
    pub struct RequestBuilder {
        pub(crate) client: super::Client,
        pub(crate) order_item_name: String,
        pub(crate) subscription_id: String,
        pub(crate) resource_group_name: String,
        pub(crate) return_order_item_details: models::ReturnOrderItemDetails,
    }
    impl RequestBuilder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
        #[doc = ""]
        #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
        #[doc = "However, this function can provide more flexibility when required."]
        pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.EdgeOrder/orderItems/{}/return",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.resource_group_name,
                        &this.order_item_name
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.return_order_item_details)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
    }
}
