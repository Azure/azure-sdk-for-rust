use crate::clients::StorageClient;
use crate::container::incomplete_vector_from_container_response;
use crate::container::responses::ListContainersResponse;
use azure_core::headers::add_optional_header;
use azure_core::headers::request_id_from_headers;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct ListContainersBuilder<'a> {
    storage_client: &'a StorageClient,
    prefix: Option<Prefix<'a>>,
    next_marker: Option<NextMarker>,
    include_metadata: bool,
    include_deleted: bool,
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> ListContainersBuilder<'a> {
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self {
        Self {
            storage_client,
            prefix: None,
            next_marker: None,
            include_metadata: false,
            include_deleted: false,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix<'a> => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        include_metadata: bool => include_metadata,
        include_deleted: bool => include_deleted,
        max_results: MaxResults => Some(max_results),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<ListContainersResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .blob_storage_url()
            .clone();

        url.query_pairs_mut().append_pair("comp", "list");

        self.prefix.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        if let Some(include) = match (self.include_metadata, self.include_deleted) {
            (true, true) => Some("metadata,deleted"),
            (true, false) => Some("metadata"),
            (false, true) => Some("deleted"),
            (false, false) => None,
        } {
            url.query_pairs_mut().append_pair("include", include);
        }
        self.max_results.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        debug!("generated url = {}", url);

        let (request, _url) = self.storage_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .storage_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?;

        debug!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;
        debug!("body == {}", body);

        let incomplete_vector = incomplete_vector_from_container_response(&body)?;
        let request_id = request_id_from_headers(response.headers())?;
        Ok(ListContainersResponse {
            incomplete_vector,
            request_id,
        })
    }

    pub fn stream(
        self,
    ) -> impl Stream<Item = Result<ListContainersResponse, Box<dyn std::error::Error + Sync + Send>>> + 'a
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(NextMarker),
        };

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.next_marker(next_marker).execute().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                // If we have a next marker, let's wrap it
                // in a States::NextMarker and pass it to the next execution.
                // If not, we'll obtain None that will end the loop.
                let next_marker = response
                    .incomplete_vector
                    .next_marker()
                    .map(|next_marker| States::NextMarker(next_marker.clone()));

                Some((Ok(response), next_marker))
            }
        })
    }
}
