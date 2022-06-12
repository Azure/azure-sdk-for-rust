use azure_core::Response;

pub mod create_table;
pub mod delete_table;
pub mod query_tables;

/// open data protocol response details level
#[derive(Debug, Clone)]
pub enum OdataMetadataLevel {
    NoMetadata,
    MinimalMetadata,
    FullMetadata,
}

impl AsRef<str> for OdataMetadataLevel {
    fn as_ref(&self) -> &str {
        match self {
            OdataMetadataLevel::NoMetadata => "application/json;odata=nometadata",
            OdataMetadataLevel::MinimalMetadata => "application/json;odata=minimalmetadata",
            OdataMetadataLevel::FullMetadata => "application/json;odata=fullmetadata",
        }
    }
}

/// Sets if the resource should be included in the response.
/// * NO_CONTENT - the response body will be empty and the status code will be 204.
/// * NO_CONTENT - the response body will contain the create table with the specified metadata details and the status code will be 201.
#[derive(Debug, Clone)]
pub enum EchoContent {
    ReturnNoContent,
    ReturnContent,
}

impl AsRef<str> for EchoContent {
    fn as_ref(&self) -> &str {
        match self {
            EchoContent::ReturnNoContent => "return-no-content",
            EchoContent::ReturnContent => "return-content",
        }
    }
}

const MINIMAL_METADATA: &str = "application/json;odata=minimalmetadata";

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableItem {
    /// The type name of the containing object.
    #[serde(rename = "odata.type")]
    pub(crate) odata_type: Option<String>,

    /// The entity ID, which is generally the URL to the resource.
    #[serde(rename = "odata.id")]
    pub(crate) odata_id: Option<String>,

    /// The link used to edit/update the entry, if the entity is updatable and the odata.id does not represent a URL that can be used to edit the entity.
    #[serde(rename = "odata.editLink")]
    pub(crate) odata_link: Option<String>,

    /// the name of the table
    #[serde(rename = "TableName")]
    pub name: String,
}

impl TableItem {
    pub async fn try_from(response: Response) -> azure_storage::Result<TableItem> {
        let body = response.into_body_string().await;
        println!("{}", body);
        Ok(serde_json::from_slice(body.as_bytes())?)
    }
}
