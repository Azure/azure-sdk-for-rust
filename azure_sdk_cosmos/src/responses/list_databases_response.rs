use crate::database::Database;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    _rid: String,
    #[serde(rename = "Databases")]
    pub databases: Vec<Database>,
    #[serde(rename = "_count")]
    pub count: u32,
}
