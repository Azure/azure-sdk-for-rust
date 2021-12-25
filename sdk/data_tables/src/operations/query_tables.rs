use super::OdataMetadataLevel;
use crate::clients::table_service_client::TableServiceClient;
use azure_core::{setters, Context, Response, EMPTY_BODY};
use azure_storage::Result;
use http::HeaderValue;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct QueryTablesBuilder<'a, 'b> {
    table_client: &'a TableServiceClient,
    context: Context,

    top: Option<i32>,
    timeout: Option<i32>,
    filter: Option<&'b str>,

    odata_metadata_level: OdataMetadataLevel,
}

impl<'a, 'b> QueryTablesBuilder<'a, 'b> {
    pub(crate) fn new(table_client: &'a TableServiceClient) -> Self {
        Self {
            table_client,
            context: Context::new(),
            odata_metadata_level: OdataMetadataLevel::NoMetadata,
            top: None,
            filter: None,
            timeout: None,
        }
    }

    setters! {
        top: i32 => Some(top),
        context: Context => context,
        timeout: i32 => Some(timeout),
        filter: &'b str => Some(filter),
        odata_metadata_level: OdataMetadataLevel => odata_metadata_level,
    }

    pub fn into_future(&self) -> ListTables {
        let builder = self
            .table_client
            .pipeline_request(http::Method::GET, "Tables")
            .header(
                "accept",
                HeaderValue::from_str(self.odata_metadata_level.as_ref()).unwrap(),
            );

        let uri = builder.uri_ref().unwrap().to_string();
        let mut request = builder
            .uri(self.path_and_query(&uri))
            .body(EMPTY_BODY)
            .unwrap()
            .into();

        let table_client = self.table_client.clone();
        let mut context = self.context.clone();

        Box::pin(async move {
            let response = table_client
                .pipeline()
                .send(&mut context, &mut request)
                .await?;
            QueryTablesResponse::try_from(response).await
        })
    }

    fn path_and_query(&self, uri: &str) -> String {
        let mut uri = Url::parse(uri).unwrap();
        if let Some(top) = self.top {
            uri.query_pairs_mut().append_pair("$top", &top.to_string());
        }
        if let Some(filter) = self.filter {
            uri.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(timeout) = self.timeout {
            uri.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        uri.to_string()
    }
}

type ListTables = futures::future::BoxFuture<'static, Result<QueryTablesResponse>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryTablesResponse {
    #[serde(rename = "value")]
    pub tables: Vec<super::Table>,
}

impl QueryTablesResponse {
    pub async fn try_from(response: Response) -> Result<Self> {
        let body = response.into_body_string().await;
        Ok(serde_json::from_slice(body.as_bytes())?)
    }
}
