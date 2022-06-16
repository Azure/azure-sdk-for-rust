use crate::{
    clients::FileSystemClient,
    file_system::{Path, PathList},
    request_options::*,
};
use azure_core::{
    collect_pinned_stream,
    error::{Error, Result},
    prelude::*,
    AppendToUrlQuery, Pageable, Response,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

/// A future of a delete file response
type ListPaths = Pageable<ListPathsResponse, Error>;

#[derive(Debug, Clone)]
pub struct ListPathsBuilder {
    client: FileSystemClient,
    recursive: Option<Recursive>,
    continuation: Option<NextMarker>,
    directory: Option<Directory>,
    max_results: Option<MaxResults>,
    timeout: Option<Timeout>,
    upn: Option<Upn>,
    client_request_id: Option<ClientRequestId>,
    context: Context,
}

impl ListPathsBuilder {
    pub(crate) fn new(client: FileSystemClient, context: Context) -> Self {
        Self {
            client,
            recursive: None,
            continuation: None,
            directory: None,
            max_results: None,
            timeout: None,
            upn: None,
            client_request_id: None,
            context,
        }
    }

    setters! {
        recursive: Recursive => Some(recursive),
        continuation: NextMarker => Some(continuation),
        directory: Directory => Some(directory),
        max_results: MaxResults => Some(max_results),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
        upn: Upn => Some(upn),
        context: Context => context,
    }

    pub fn into_stream(self) -> ListPaths {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();

            async move {
                let mut url = this.client.url().unwrap();
                ResourceType::FileSystem.append_to_url_query(&mut url);
                this.recursive.append_to_url_query(&mut url);
                this.directory.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);
                this.timeout.append_to_url_query(&mut url);
                this.upn.append_to_url_query(&mut url);

                if let Some(c) = continuation {
                    let nm: NextMarker = c.into();
                    nm.append_to_url_query_as_continuation(&mut url);
                } else {
                    this.continuation.append_to_url_query(&mut url);
                };

                let mut request = this.client.prepare_request(url.as_str(), http::Method::GET);

                request.insert_headers(&this.client_request_id);

                let response = this
                    .client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await?;

                ListPathsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Clone, Debug)]
pub struct ListPathsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub paths: Vec<Path>,
    pub continuation: Option<NextMarker>,
}

impl ListPathsResponse {
    pub(crate) async fn try_from(response: Response) -> Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        let path_list: PathList = body.try_into()?;

        Ok(ListPathsResponse {
            common_storage_response_headers: (&headers).try_into()?,
            paths: path_list.paths,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}

impl Continuable for ListPathsResponse {
    fn continuation(&self) -> Option<String> {
        self.continuation.clone().map(|m| m.as_str().into())
    }
}

impl IntoIterator for ListPathsResponse {
    type Item = Path;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}
