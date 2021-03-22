use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "odata.type")]
    pub table_type: String,
    #[serde(rename = "odata.id")]
    pub id: String,
    #[serde(rename = "odata.editLink")]
    pub edit_link: String,
    #[serde(rename = "TableName")]
    pub name: String,
}
