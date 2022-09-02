use crate::clients::DataLakeClient;
use crate::file_system::{FileSystem, FileSystemList};
use azure_core::{error::Error, prelude::*, Pageable, Response};
use azure_core::{AppendToUrlQuery, Request};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

type ListFileSystems = Pageable<ListFileSystemsResponse, Error>;

operation! {
    #[stream]
    ListFileSystems,
    client: DataLakeClient,
    ?prefix: Prefix,
    ?next_marker: NextMarker,
    ?max_results: MaxResults
}

impl ListFileSystemsBuilder {
    pub fn into_stream(self) -> ListFileSystems {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();

            async move {
                let mut url = this.client.url()?;
                url.query_pairs_mut().append_pair("resource", "account");
                this.prefix.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);

                if let Some(c) = continuation {
                    c.append_to_url_query_as_continuation(&mut url);
                } else {
                    this.next_marker.append_to_url_query(&mut url);
                };

                let mut request = Request::new(url, azure_core::Method::Get);

                let response = this.client.send(&mut ctx, &mut request).await?;

                ListFileSystemsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Clone, Debug)]
pub struct ListFileSystemsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub file_systems: Vec<FileSystem>,
    pub next_marker: Option<NextMarker>,
}

impl ListFileSystemsResponse {
    pub(crate) async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let file_system_list: FileSystemList = body.try_into()?;

        Ok(ListFileSystemsResponse {
            common_storage_response_headers: (&headers).try_into()?,
            file_systems: file_system_list.file_systems,
            next_marker: NextMarker::from_header_optional(&headers)?,
        })
    }
}

impl Continuable for ListFileSystemsResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}

impl IntoIterator for ListFileSystemsResponse {
    type Item = FileSystem;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.file_systems.into_iter()
    }
}
