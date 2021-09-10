pub mod create_table;
pub mod delete_table;
pub mod query_tables;

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
