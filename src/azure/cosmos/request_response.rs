use azure::cosmos::database::Database;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    _rid: String,
    #[serde(rename = "Databases")]
    pub databases: Vec<Database>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Serialize)]
pub struct CreateDatabaseRequest<'a> {
    pub id: &'a str,
}
