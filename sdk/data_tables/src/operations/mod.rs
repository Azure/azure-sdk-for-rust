use azure_core::Response;
pub mod create_table;
pub mod delete_table;
pub mod query_tables;

const MINIMAL_METADATA: &str = "application/json;odata=minimalmetadata";
const RETURN_NO_CONTENT: &str = "return-no-content";

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
        Ok(serde_json::from_slice(body.as_bytes())?)
    }
}
