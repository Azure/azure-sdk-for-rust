use crate::collection::Collection;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListCollectionsResponse {
    _rid: String,
    #[serde(rename = "DocumentCollections")]
    pub collections: Vec<Collection>,
    #[serde(rename = "_count")]
    pub count: u32,
}
