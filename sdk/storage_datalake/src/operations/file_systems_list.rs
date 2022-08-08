use crate::clients::DataLakeClient;
use crate::file_system::{FileSystem, FileSystemList};
use azure_core::{error::Error, prelude::*, Pageable, Response};
use azure_core::{AppendToUrlQuery, Request};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

type ListFileSystems = Pageable<ListFileSystemsResponse, Error>;

#[derive(Debug, Clone)]
pub struct ListFileSystemsBuilder {
    client: DataLakeClient,
    prefix: Option<Prefix>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    context: Option<Context>,
}

impl ListFileSystemsBuilder {
    pub(crate) fn new(client: DataLakeClient, context: Option<Context>) -> Self {
        Self {
            client,
            prefix: None,
            next_marker: None,
            max_results: None,
            client_request_id: None,
            timeout: None,
            context,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        context: Context => Some(context),
    }

    pub fn into_stream(self) -> ListFileSystems {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut url = url::Url::parse(this.client.url()).unwrap();
                url.query_pairs_mut().append_pair("resource", "account");
                this.prefix.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);
                this.timeout.append_to_url_query(&mut url);

                if let Some(c) = continuation {
                    c.append_to_url_query_as_continuation(&mut url);
                } else {
                    this.next_marker.append_to_url_query(&mut url);
                };

                let mut request = Request::new(url, azure_core::Method::Get);

                request.insert_headers(&this.client_request_id);

                let response = this
                    .client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await?;

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
