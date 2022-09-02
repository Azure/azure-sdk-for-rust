use crate::{
    clients::FileSystemClient,
    file_system::{Path, PathList},
    request_options::*,
};
use azure_core::{error::Error, prelude::*, AppendToUrlQuery, Pageable, Request, Response};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    #[stream]
    ListPaths,
    client: FileSystemClient,
    ?recursive: Recursive,
    ?continuation: Option<NextMarker>,
    ?directory: Directory,
    ?max_results: MaxResults,
    ?upn: Upn
}

impl ListPathsBuilder {
    pub fn into_stream(self) -> Pageable<ListPathsResponse, Error> {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();

            async move {
                let mut url = this.client.url().unwrap();
                ResourceType::FileSystem.append_to_url_query(&mut url);
                this.recursive.append_to_url_query(&mut url);
                this.directory.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);
                this.upn.append_to_url_query(&mut url);

                if let Some(next_marker) = continuation {
                    next_marker.append_to_url_query_as_continuation(&mut url);
                } else {
                    this.continuation.append_to_url_query(&mut url);
                };

                let mut request = Request::new(url, azure_core::Method::Get);

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
    pub(crate) async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let path_list: PathList = body.try_into()?;

        Ok(ListPathsResponse {
            common_storage_response_headers: (&headers).try_into()?,
            paths: path_list.paths,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}

impl Continuable for ListPathsResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation.clone()
    }
}

impl IntoIterator for ListPathsResponse {
    type Item = Path;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}
