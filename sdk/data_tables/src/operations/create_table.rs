use super::{EchoContent, OdataMetadataLevel};
use crate::clients::table_service_client::TableServiceClient;
use azure_core::{setters, Context, Response};
use azure_storage::Result;
use http::HeaderValue;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub struct CreateTableBuilder<'a> {
    table_client: &'a TableServiceClient,
    table_name: String,
    context: Context,
    timeout: Option<i32>,
    echo_content: EchoContent,
    odata_metadata_level: OdataMetadataLevel,
}

impl<'a> CreateTableBuilder<'a> {
    pub fn new(table_client: &'a TableServiceClient, table_name: String) -> Self {
        Self {
            table_name,
            table_client,
            context: Context::new(),
            timeout: None,
            echo_content: EchoContent::ReturnContent,
            odata_metadata_level: OdataMetadataLevel::NoMetadata,
        }
    }

    setters! {
        context: Context => context,
        timeout: i32 => Some(timeout),
        echo_content: EchoContent => echo_content,
        odata_metadata_level: OdataMetadataLevel => odata_metadata_level,
    }

    pub fn into_future(&self) -> CreateTable {
        let builder = self
            .table_client
            .pipeline_request(http::Method::POST, "Tables");

        let body = self.body();
        let md5 = base64::encode(&md5::compute(body.as_ref())[..]);
        let uri = builder.uri_ref().unwrap().to_string();

        let mut request = builder
            .uri(self.path_and_query(&uri))
            .header(
                "prefer",
                HeaderValue::from_str(self.echo_content.as_ref()).unwrap(),
            )
            .header(
                "accept",
                HeaderValue::from_str(self.odata_metadata_level.as_ref()).unwrap(),
            )
            .header("Content-Length", HeaderValue::from(body.len()))
            .header("Content-MD5", HeaderValue::from_str(md5.as_str()).unwrap())
            .body(body)
            .unwrap()
            .into();

        let table_client = self.table_client.clone();
        let mut context = self.context.clone();

        Box::pin(async move {
            CreateTableResponse::try_from(
                table_client
                    .pipeline()
                    .send(&mut context, &mut request)
                    .await?,
            )
            .await
        })
    }

    fn path_and_query(&self, uri: &str) -> String {
        let mut uri = Url::parse(uri).unwrap();
        if let Some(timeout) = self.timeout {
            uri.query_pairs_mut()
                .append_pair("timeout", &timeout.to_string());
        }
        uri.to_string()
    }

    fn body(&self) -> bytes::Bytes {
        #[derive(serde::Serialize)]
        struct CreateTableRequest<'a> {
            #[serde(rename = "TableName")]
            pub table_name: &'a str,
        }
        let body = CreateTableRequest {
            table_name: &self.table_name,
        };
        bytes::Bytes::from(serde_json::to_string(&body).unwrap())
    }
}

/// A future of a create database response
type CreateTable = futures::future::BoxFuture<'static, Result<CreateTableResponse>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableResponse {
    #[serde(flatten)]
    pub table: super::Table,
}

impl CreateTableResponse {
    pub async fn try_from(response: Response) -> Result<Self> {
        let body = response.into_body_string().await;
        Ok(serde_json::from_slice(body.as_bytes())?)
    }
}
