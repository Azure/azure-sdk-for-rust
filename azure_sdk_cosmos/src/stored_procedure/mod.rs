mod parameters;
mod stored_procedure_name;

pub use self::parameters::Parameters;
pub use self::stored_procedure_name::StoredProcedureName;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredProcedure {
    pub id: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    pub body: String,
}

impl StoredProcedureName for StoredProcedure {
    fn name(&self) -> &str {
        &self.id
    }
}
