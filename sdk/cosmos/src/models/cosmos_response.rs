struct CosmosResponse {
    data: CosmosResponseData,
}

enum CosmosResponseData {
    Dictionary(std::collections::HashMap<String, serde_json::Value>),
    List(Vec<serde_json::Value>),
}

impl CosmosResponse {
    pub fn from_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        let data = if value.is_object() {
            CosmosResponseData::Dictionary(value.as_object().unwrap().clone())
        } else if value.is_array() {
            CosmosResponseData::List(value.as_array().unwrap().to_vec())
        } else {
            return Err(serde_json::Error::custom("Invalid response format"));
        };

        Ok(CosmosResponse { data })
    }
}