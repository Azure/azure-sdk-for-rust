use crate::file_system::{FileSystem, FileSystemList};
use azure_core::AppendToUrlQuery;
use azure_core::{collect_pinned_stream, prelude::*, Pageable, Response};
use azure_storage::{core::headers::CommonStorageResponseHeaders, prelude::*};
use std::convert::TryFrom;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct ListFileSystems {
    client: StorageAccountClient,
    prefix: Option<Prefix>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    context: Option<Context>,
}

impl ListFileSystems {
    pub(crate) fn new(client: StorageAccountClient) -> Self {
        Self {
            client,
            prefix: None,
            next_marker: None,
            max_results: None,
            client_request_id: None,
            timeout: None,
            context: None,
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

    pub fn into_stream(self) -> Pin<Box<Pageable<ListFileSystemsResponse>>> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut url = this.client.filesystem_url().clone();
                url.query_pairs_mut().append_pair("resource", "account");
                this.prefix.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);
                this.timeout.append_to_url_query(&mut url);

                if let Some(c) = continuation {
                    let nm: NextMarker = c.into();
                    nm.append_to_url_query_as_continuation(&mut url);
                } else {
                    this.next_marker.append_to_url_query(&mut url);
                };

                let mut request = this
                    .client
                    .prepare_request_pipeline(url.as_str(), http::Method::GET);

                azure_core::headers::add_optional_header2(&this.client_request_id, &mut request)?;

                trace!("request == {:?}", request);

                let response = match this
                    .client
                    .pipeline()
                    .send(ctx.clone().insert(ServiceType::Blob), &mut request)
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                match ListFileSystemsResponse::try_from(response).await {
                    Ok(r) => Ok(r),
                    Err(e) => Err(azure_core::Error::Other(Box::new(e))),
                }
            }
        };

        Box::pin(Pageable::new(make_request))
    }
}

#[derive(Clone, Debug)]
pub struct ListFileSystemsResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub file_systems: Vec<FileSystem>,
    pub next_marker: Option<NextMarker>,
}

impl ListFileSystemsResponse {
    pub(crate) async fn try_from(response: Response) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        trace!("{}", std::str::from_utf8(&body)?);
        trace!("{:?}", headers);

        let file_system_list: FileSystemList = serde_json::from_slice(&body)?;

        Ok(ListFileSystemsResponse {
            common_storage_response_headers: CommonStorageResponseHeaders::try_from(&headers)?,
            file_systems: file_system_list.file_systems,
            next_marker: NextMarker::from_header_optional(&headers)?,
        })
    }
}

impl Continuable for ListFileSystemsResponse {
    fn continuation(&self) -> Option<String> {
        self.next_marker.clone().map(|m| m.as_str().into())
    }
}

impl IntoIterator for ListFileSystemsResponse {
    type Item = FileSystem;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.file_systems.into_iter()
    }
}
