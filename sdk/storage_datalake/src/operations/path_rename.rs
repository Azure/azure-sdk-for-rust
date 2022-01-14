use crate::clients::PathClient;
use crate::request_options::*;
use crate::Properties;
use azure_core::prelude::{
    ClientRequestId, Context, IfMatchCondition, IfModifiedSinceCondition, NextMarker, Timeout, *,
};
use azure_core::{
    headers::{add_mandatory_header2, add_optional_header2},
    AppendToUrlQuery,
};
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
pub struct RenamePathBuilder<C>
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
            resource: None,
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
        resource: ResourceType => Some(resource),
        continuation: NextMarker => Some(continuation),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        rename_source: RenameSource => Some(rename_source),
        properties: Properties => Some(properties),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(self) -> BoxFuture<'static, crate::Result<C>> {
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

            let mut request = this.client.prepare_request(url.as_str(), http::Method::PUT);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;
            add_optional_header2(&this.if_match_condition, &mut request)?;
            add_optional_header2(&this.if_modified_since, &mut request)?;
            add_optional_header2(&this.rename_source, &mut request)?;
            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            self.client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            Ok(this.client)
        })
    }
}
