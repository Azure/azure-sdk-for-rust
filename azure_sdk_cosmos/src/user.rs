pub trait UserName: std::fmt::Debug {
    fn id(&self) -> &str;
}

impl UserName for User {
    fn id(&self) -> &str {
        &self.id
    }
}

impl UserName for String {
    fn id(&self) -> &str {
        &self
    }
}

impl UserName for &str {
    fn id(&self) -> &str {
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct User {
    pub id: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(skip_serializing)]
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_permissions")]
    pub permissions: String,
}

impl std::convert::TryFrom<&[u8]> for User {
    type Error = serde_json::Error;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(body)
    }
}
