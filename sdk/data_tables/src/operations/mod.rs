pub mod query_tables;
pub mod create_table;
pub mod delete_table;

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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TableMetadata {
    /// The type name of the containing object.
    #[serde(rename = "odata.type")]
    pub odata_type: String,

    /// The entity ID, which is generally the URL to the resource.
    #[serde(rename = "odata.id")]
    pub odata_id: String,

    /// The link used to edit/update the entry, if the entity is updatable and the odata.id does not represent a URL that can be used to edit the entity.
    #[serde(rename = "odata.editLink")]
    pub odata_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    /// this field will contain the table metadata only if the user specified the sufficient metadata level in the request options
    #[serde(flatten)]
    pub metadata: Option<TableMetadata>,
    /// the name of the table
    #[serde(rename = "TableName")]
    pub name: String,
}
