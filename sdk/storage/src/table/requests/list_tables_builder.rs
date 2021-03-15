use crate::table::prelude::*;
use crate::table::responses::*;
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListTablesBuilder<'a> {
    table_service_client: &'a TableServiceClient,
    filter: Option<Filter<'a>>,
    select: Option<Select<'a>>,
    top: Option<Top>,
    next_marker: Option<NextMarker>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> ListTablesBuilder<'a> {
    pub(crate) fn new(table_service_client: &'a TableServiceClient) -> Self {
        Self {
            table_service_client,
            filter: None,
            select: None,
            top: None,
            next_marker: None,
            client_request_id: None,
        }
    }

    setters! {
        filter: Filter<'a> => Some(filter),
        select: Select<'a> => Some(select),
        top: Top => Some(top),
        next_marker: NextMarker => Some(next_marker),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<ListTablesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.table_service_client.url().to_owned();

        self.filter.append_to_url_query(&mut url);
        self.select.append_to_url_query(&mut url);
        self.top.append_to_url_query(&mut url);
        if let Some(next_marker) = &self.next_marker {
            url.query_pairs_mut()
                .append_pair("NextTableName", next_marker.as_str());
        }

        debug!("list tables url = {}", url);

        let request = self.table_service_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header("Accept", "application/json;odata=fullmetadata");
                request
            },
            None,
        )?;

        debug!("request == {:#?}\n", request);

        let response = self
            .table_service_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }

    pub fn stream(
        self,
    ) -> impl Stream<Item = Result<ListTablesResponse, Box<dyn std::error::Error + Sync + Send>>> + 'a
    {
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

                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = response
                    .next_marker
                    .clone()
                    .map(|next_marker| States::NextMarker(next_marker));

                Some((Ok(response), next_marker))
            }
        })
    }
}
