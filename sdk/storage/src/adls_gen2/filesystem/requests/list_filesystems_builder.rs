use crate::core::prelude::*;
use crate::filesystem::responses::ListFilesystemsResponse;
use azure_core::errors::{check_status_extract_headers_and_body_as_string, AzureError};
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    client: &'a C,
    continuation: Option<&'a str>,
    max_results: Option<u32>,
    prefix: Option<&'a str>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    pub(crate) fn new(client: &'a C) -> ListFilesystemsBuilder<'a, C> {
        ListFilesystemsBuilder {
            client,
            continuation: None,
            max_results: None,
            prefix: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C> ClientRequired<'a, C> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

impl<'a, C> ContinuationOption<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn continuation(&self) -> Option<&'a str> {
        self.continuation
    }
}

impl<'a, C> MaxResultsOption for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, C> PrefixOption<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, C> TimeoutOption for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> ContinuationSupport<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    type O = ListFilesystemsBuilder<'a, C>;

    #[inline]
    fn with_continuation(self, continuation: &'a str) -> Self::O {
        ListFilesystemsBuilder {
            client: self.client,
            continuation: Some(continuation),
            max_results: self.max_results,
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> MaxResultsSupport for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    type O = ListFilesystemsBuilder<'a, C>;

    #[inline]
    fn with_max_results(self, max_results: u32) -> Self::O {
        ListFilesystemsBuilder {
            client: self.client,
            continuation: self.continuation,
            max_results: Some(max_results),
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> PrefixSupport<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    type O = ListFilesystemsBuilder<'a, C>;

    #[inline]
    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListFilesystemsBuilder {
            client: self.client,
            continuation: self.continuation,
            max_results: self.max_results,
            prefix: Some(prefix),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    type O = ListFilesystemsBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ListFilesystemsBuilder {
            client: self.client,
            continuation: self.continuation,
            max_results: self.max_results,
            prefix: self.prefix,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    type O = ListFilesystemsBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListFilesystemsBuilder {
            client: self.client,
            continuation: self.continuation,
            max_results: self.max_results,
            prefix: self.prefix,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C> ListFilesystemsBuilder<'a, C>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<ListFilesystemsResponse, AzureError> {
        let mut uri = format!("{}/?resource=account", self.client().filesystem_uri(),);

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        if let Some(nm) = MaxResultsOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        if let Some(nm) = PrefixOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let (_, future_response) = self.client().perform_request(
            &uri,
            &Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body_as_string(future_response, StatusCode::OK)
                .await?;
        ListFilesystemsResponse::from_response(&headers, &body)
    }
}

impl<'a, C> ListFilesystemsBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn stream(self) -> impl Stream<Item = Result<ListFilesystemsResponse, AzureError>> + 'a {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(Some(States::Init), move |continuation: Option<States>| {
            let req = self.clone();
            async move {
                debug!("continuation == {:?}", &continuation);
                let response = match continuation {
                    Some(States::Init) => req.finalize().await,
                    Some(States::Continuation(continuation)) => {
                        req.with_continuation(&continuation).finalize().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let continuation = match response.incomplete_vector.token() {
                    Some(ct) => Some(States::Continuation(ct.to_owned())),
                    None => None,
                };

                Some((Ok(response), continuation))
            }
        })
    }
}
