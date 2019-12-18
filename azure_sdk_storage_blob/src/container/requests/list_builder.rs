use crate::container::responses::ListContainersResponse;
use crate::container::Container;
use azure_sdk_core::errors::{check_status_extract_headers_and_body_as_string, AzureError};
use azure_sdk_core::incompletevector::IncompleteVector;
use azure_sdk_core::parsing::{cast_optional, traverse};
use azure_sdk_core::{
    request_id_from_headers, ClientRequestIdOption, ClientRequestIdSupport, NextMarkerOption,
    NextMarkerSupport, PrefixOption, PrefixSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};
use xml::Element;

#[derive(Debug, Clone)]
pub struct ListBuilder<'a> {
    client: &'a Client,
    max_results: u64,
    include_metadata: bool,
    next_marker: Option<&'a str>,
    prefix: Option<&'a str>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a> ClientRequired<'a> for ListBuilder<'a> {
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> ListBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> ListBuilder<'a> {
        ListBuilder {
            client,
            max_results: 5000,
            include_metadata: false,
            next_marker: None,
            prefix: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

// regardless implementation
impl<'a> ListBuilder<'a> {
    pub fn max_results(&self) -> u64 {
        self.max_results
    }

    pub fn with_max_results(self, max_results: u64) -> Self {
        ListBuilder {
            client: self.client,
            max_results,
            include_metadata: self.include_metadata,
            next_marker: self.next_marker,
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }

    pub fn is_metadata_included(&self) -> bool {
        self.include_metadata
    }

    pub fn include_metadata(self) -> Self {
        ListBuilder {
            client: self.client,
            max_results: self.max_results,
            include_metadata: true,
            next_marker: self.next_marker,
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }

    pub async fn finalize(self) -> Result<ListContainersResponse, AzureError> {
        let mut uri = format!(
            "{}?comp=list&maxresults={}",
            self.client().blob_uri(),
            self.max_results()
        );

        if self.is_metadata_included() {
            uri = format!("{}&include=metadata", uri);
        }

        if let Some(nm) = PrefixOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        if let Some(nm) = NextMarkerOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::GET,
            |request| ClientRequestIdOption::add_header(&self, request),
            None,
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body_as_string(future_response, StatusCode::OK)
                .await?;
        let incomplete_vector = incomplete_vector_from_response(&body)?;
        let request_id = request_id_from_headers(&headers)?;
        Ok(ListContainersResponse {
            incomplete_vector,
            request_id,
        })
    }
}

impl<'a> PrefixOption<'a> for ListBuilder<'a> {
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a> PrefixSupport<'a> for ListBuilder<'a> {
    type O = ListBuilder<'a>;

    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            next_marker: self.next_marker,
            prefix: Some(prefix),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a> TimeoutOption for ListBuilder<'a> {
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a> TimeoutSupport for ListBuilder<'a> {
    type O = ListBuilder<'a>;

    fn with_timeout(self, timeout: u64) -> Self::O {
        ListBuilder {
            client: self.client,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            next_marker: self.next_marker,
            prefix: self.prefix,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a> NextMarkerOption<'a> for ListBuilder<'a> {
    fn next_marker(&self) -> Option<&'a str> {
        match self.next_marker {
            Some(nm) => Some(nm),
            None => None,
        }
    }
}

impl<'a> NextMarkerSupport<'a> for ListBuilder<'a> {
    type O = ListBuilder<'a>;

    fn with_next_marker(self, next_marker: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            next_marker: Some(next_marker),
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a> ClientRequestIdOption<'a> for ListBuilder<'a> {
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a> ClientRequestIdSupport<'a> for ListBuilder<'a> {
    type O = ListBuilder<'a>;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            next_marker: self.next_marker,
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

fn incomplete_vector_from_response(body: &str) -> Result<IncompleteVector<Container>, AzureError> {
    let elem: Element = body.parse()?;

    let mut v = Vec::new();

    for container in traverse(&elem, &["Containers", "Container"], true)? {
        v.push(Container::parse(container)?);
    }

    let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
        Some(ref nm) if nm == "" => None,
        Some(nm) => Some(nm),
        None => None,
    };

    Ok(IncompleteVector::new(next_marker, v))
}
