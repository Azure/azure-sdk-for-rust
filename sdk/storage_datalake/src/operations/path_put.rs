use crate::clients::PathClient;
use crate::request_options::*;
use crate::Properties;
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::*;
use azure_core::Request;
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct PutPathBuilder<C>
where
    C: PathClient,
{
    client: C,
    mode: Option<PathRenameMode>,
    resource: Option<ResourceType>,
    continuation: Option<NextMarker>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    properties: Option<Properties>,
    timeout: Option<Timeout>,
    context: Context,
}

impl<C: PathClient + 'static> PutPathBuilder<C> {
    pub(crate) fn new(client: C, context: Context) -> Self {
        Self {
            client,
            mode: None,
            continuation: None,
            resource: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            properties: None,
            timeout: None,
            context,
        }
    }

    setters! {
        mode: PathRenameMode => Some(mode),
        resource: ResourceType => Some(resource),
        continuation: NextMarker => Some(continuation),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        properties: Properties => Some(properties),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(self) -> PutPath {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.resource.append_to_url_query(&mut url);
            self.mode.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Put);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.properties);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            PutPathResponse::try_from(response).await
        })
    }
}

azure_core::future!(PutPath);

#[derive(Debug, Clone)]
pub struct PutPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub continuation: Option<NextMarker>,
}

impl PutPathResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RenamePathBuilder<C>
where
    C: PathClient,
{
    client: C,
    mode: Option<PathRenameMode>,
    continuation: Option<NextMarker>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    rename_source: Option<RenameSource>,
    properties: Option<Properties>,
    timeout: Option<Timeout>,
    context: Context,
}

impl<C: PathClient + 'static> RenamePathBuilder<C> {
    pub(crate) fn new(client: C, context: Context) -> Self {
        Self {
            client,
            mode: None,
            continuation: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            rename_source: None,
            properties: None,
            timeout: None,
            context,
        }
    }

    setters! {
        mode: PathRenameMode => Some(mode),
        continuation: NextMarker => Some(continuation),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        rename_source: RenameSource => Some(rename_source),
        properties: Properties => Some(properties),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(self) -> RenamePath {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.mode.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Put);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.properties);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);
            request.insert_headers(&this.rename_source);
            request.insert_headers(&ContentLength::new(0));

            self.client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            Ok(())
        })
    }
}

azure_core::future!(RenamePath);

type RenamePathResponse = ();
