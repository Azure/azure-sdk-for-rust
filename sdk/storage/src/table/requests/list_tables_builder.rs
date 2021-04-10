use crate::table::responses::*;
use crate::{table::prelude::*, ContinuationNextTableName};
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[cfg(test)]
use std::println as debug;

#[derive(Debug, Clone)]
pub struct ListTablesBuilder<'a> {
    table_service_client: &'a TableServiceClient,
    filter: Option<Filter<'a>>,
    select: Option<Select<'a>>,
    top: Option<Top>,
    continuation_next_table_name: Option<ContinuationNextTableName>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> ListTablesBuilder<'a> {
    pub(crate) fn new(table_service_client: &'a TableServiceClient) -> Self {
        Self {
            table_service_client,
            filter: None,
            select: None,
            top: None,
            continuation_next_table_name: None,
            client_request_id: None,
        }
    }

    setters! {
        filter: Filter<'a> => Some(filter),
        select: Select<'a> => Some(select),
        top: Top => Some(top),
        continuation_next_table_name: ContinuationNextTableName => Some(continuation_next_table_name),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<ListTablesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.table_service_client.url().to_owned();

        self.filter.append_to_url_query(&mut url);
        self.select.append_to_url_query(&mut url);
        self.top.append_to_url_query(&mut url);
        self.continuation_next_table_name
            .append_to_url_query(&mut url);

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
            ContinuationNextTableName(ContinuationNextTableName),
        }

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::ContinuationNextTableName(continuation_next_table_name)) => {
                        req.continuation_next_table_name(continuation_next_table_name)
                            .execute()
                            .await
                    }
                    None => return None,
                };

                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = response
                    .continuation_next_table_name
                    .clone()
                    .map(States::ContinuationNextTableName);

                Some((Ok(response), next_marker))
            }
        })
    }
}
