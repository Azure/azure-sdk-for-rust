mod models;

use models::CosmosResponse;
use serde_json::Value;

pub struct TransactionalBatch {
    // Your fields here
}

impl TransactionalBatch {
    pub fn process_response(&self, response: Value) -> Result<Vec<CosmosResponse>, String> {
        if let Some(array) = response.as_array() {
            let mut results = Vec::new();
            for item in array {
                let cosmos_response: CosmosResponse = serde_json::from_value(item.clone())
                    .map_err(|e| format!("Failed to parse item: {}", e))?;
                results.push(cosmos_response);
            }
            Ok(results)
        } else {
            Err("Invalid response format: expected a list".to_string())
        }
    }
}