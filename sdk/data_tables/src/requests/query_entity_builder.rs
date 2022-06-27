use crate::{prelude::*, responses::*, ContinuationNextPartitionAndRowKey};
use azure_core::Method;
use azure_core::{
    error::{Error, ErrorKind},
    prelude::*,
    AppendToUrlQuery,
};
use futures::stream::{unfold, Stream};
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct QueryEntityBuilder<'a> {
    table_client: &'a TableClient,
    filter: Option<Filter<'a>>,
    select: Option<Select<'a>>,
    top: Option<Top>,
    continuation_next_partition_and_row_key: Option<ContinuationNextPartitionAndRowKey>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> QueryEntityBuilder<'a> {
    pub(crate) fn new(table_client: &'a TableClient) -> Self {
        Self {
            table_client,
            filter: None,
            select: None,
            top: None,
            continuation_next_partition_and_row_key: None,
            client_request_id: None,
        }
    }

    setters! {
        filter: Filter<'a> => Some(filter),
        select: Select<'a> => Some(select),
        top: Top => Some(top),
        continuation_next_partition_and_row_key: ContinuationNextPartitionAndRowKey => Some(continuation_next_partition_and_row_key),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute<E>(&self) -> azure_core::Result<QueryEntityResponse<E>>
    where
        E: DeserializeOwned,
    {
        let mut url = self.table_client.url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(&format!("{}()", self.table_client.table_name()));

        self.filter.append_to_url_query(&mut url);
        self.select.append_to_url_query(&mut url);
        self.top.append_to_url_query(&mut url);
        self.continuation_next_partition_and_row_key
            .append_to_url_query(&mut url);

        let mut request = self
            .table_client
            .prepare_request(url.as_str(), Method::Get, None)?;
        request.add_optional_header(&self.client_request_id);
        request.insert_header("Accept", "application/json;odata=fullmetadata");

        let response = self
            .table_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn stream<E>(self) -> impl Stream<Item = azure_core::Result<QueryEntityResponse<E>>> + 'a
    where
        E: DeserializeOwned,
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            ContinuationNextPartitionAndRowKey(ContinuationNextPartitionAndRowKey),
        }

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::ContinuationNextPartitionAndRowKey(
                        continuation_next_partition_and_row_key,
                    )) => {
                        req.continuation_next_partition_and_row_key(
                            continuation_next_partition_and_row_key,
                        )
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
                    .continuation_next_partition_and_row_key
                    .clone()
                    .map(States::ContinuationNextPartitionAndRowKey);

                Some((Ok(response), next_marker))
            }
        })
    }
}
