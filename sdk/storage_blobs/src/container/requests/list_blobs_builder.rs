use crate::{blob::responses::ListBlobsResponse, prelude::*};
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListBlobsBuilder<'a> {
    container_client: &'a ContainerClient,
    prefix: Option<Prefix>,
    delimiter: Option<Delimiter<'a>>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    include_snapshots: bool,
    include_metadata: bool,
    include_uncommitted_blobs: bool,
    include_copy: bool,
    include_deleted: bool,
    include_tags: bool,
    include_versions: bool,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ListBlobsBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        Self {
            container_client,
            prefix: None,
            delimiter: None,
            next_marker: None,
            max_results: None,
            include_snapshots: false,
            include_metadata: false,
            include_uncommitted_blobs: false,
            include_copy: false,
            include_deleted: false,
            include_tags: false,
            include_versions: false,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        delimiter: Delimiter<'a> => Some(delimiter),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        include_snapshots: bool => include_snapshots,
        include_metadata: bool => include_metadata,
        include_uncommitted_blobs: bool => include_uncommitted_blobs,
        include_copy: bool => include_copy,
        include_deleted: bool => include_deleted,
        include_tags: bool => include_tags,
        include_versions: bool => include_versions,
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(&self) -> azure_core::Result<ListBlobsResponse> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "list");

        self.prefix.append_to_url_query(&mut url);
        self.delimiter.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        self.max_results.append_to_url_query(&mut url);

        // This code will construct the "include" query pair
        // attribute. It only allocates a Vec of references ('static
        // str) and, finally, a single string.
        let mut optional_includes = Vec::new();
        if self.include_snapshots {
            optional_includes.push("snapshots");
        }
        if self.include_metadata {
            optional_includes.push("metadata");
        }
        if self.include_uncommitted_blobs {
            optional_includes.push("uncommittedblobs");
        }
        if self.include_copy {
            optional_includes.push("copy");
        }
        if self.include_deleted {
            optional_includes.push("deleted");
        }
        if self.include_tags {
            optional_includes.push("tags");
        }
        if self.include_versions {
            optional_includes.push("versions");
        }
        if !optional_includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &optional_includes.join(","));
        }

        self.timeout.append_to_url_query(&mut url);

        let mut request = self
            .container_client
            .prepare_request(url.as_str(), Method::GET, None)?;
        request.add_optional_header(self.client_request_id);

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn stream(self) -> impl Stream<Item = azure_core::Result<ListBlobsResponse>> + 'a {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(NextMarker),
        }

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

                let next_marker = response.next_marker.clone().map(States::NextMarker);

                Some((Ok(response), next_marker))
            }
        })
    }
}
